extern crate freetype as ft;
extern crate gl;

use crate::shaders;
use std::ffi::CString;

use crate::check_error;
use crate::font::font::{FontAtlas, GlyphInstance};
use crate::matrix;

pub struct FontRenderer {
    pub char_width: f32,
    pub advance_height: f32,
    pub ascender: f32,

    font_atlas: FontAtlas,
    program: shaders::Program,
    vao: gl::types::GLuint,
    quad_buffer_object: gl::types::GLuint,
    quad_buffer_size: usize,
    transform_loc: gl::types::GLint,
    index_buffer: gl::types::GLuint,
}

fn create_shader_program() -> shaders::Program {
    let vert_shader =
        shaders::Shader::from_vert_source(&CString::new(include_str!("font.vert")).unwrap())
            .unwrap();

    let frag_shader =
        shaders::Shader::from_frag_source(&CString::new(include_str!("font.frag")).unwrap())
            .unwrap();

    shaders::Program::from_shaders(&[vert_shader, frag_shader]).unwrap()
}

fn vao_setup() {
    unsafe {
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<GlyphInstance>() as gl::types::GLint,
            std::ptr::null(),
        );
        gl::VertexAttribDivisor(0, 1);
        check_error!();

        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<GlyphInstance>() as gl::types::GLint,
            (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl::VertexAttribDivisor(1, 1);

        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<GlyphInstance>() as gl::types::GLint,
            (4 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl::VertexAttribDivisor(2, 1);

        gl::EnableVertexAttribArray(3);
        gl::VertexAttribPointer(
            3,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<GlyphInstance>() as gl::types::GLint,
            (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl::VertexAttribDivisor(3, 1);

        gl::EnableVertexAttribArray(4);
        gl::VertexAttribPointer(
            4,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<GlyphInstance>() as gl::types::GLint,
            (8 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl::VertexAttribDivisor(4, 1);
    }
}

impl FontRenderer {
    pub fn new() -> FontRenderer {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            vao_setup();
        }

        let mut index_buffer: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut index_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);

            let indices = [0, 1, 2, 0, 2, 3];

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(&indices) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        let shader_program = create_shader_program();
        let transform_loc;
        unsafe {
            transform_loc = gl::GetUniformLocation(
                shader_program.id,
                CString::new("projection").unwrap().as_ptr(),
            );

            if transform_loc == -1 {
                panic!("location not found");
            }
        }

        let mut atlas = FontAtlas::new(14);
        let char_with = atlas.get_glyph(' ').advance_width;
        let advance_height = atlas.advance_height;

        let ascender = (atlas.face.ascender() >> 6) as f32;

        FontRenderer {
            char_width: char_with,
            advance_height: advance_height,
            ascender: ascender,

            program: shader_program,
            vao: vao,
            quad_buffer_object: vbo,
            quad_buffer_size: 0,
            font_atlas: atlas,
            transform_loc: transform_loc,
            index_buffer,
        }
    }

    fn set_projection(&self, projection: &matrix::Matrix) {
        unsafe {
            gl::UniformMatrix4fv(
                self.transform_loc,
                1,
                gl::FALSE,
                projection.as_ptr() as *const f32,
            );
        }
    }

    fn fill_buffer(&mut self, text: &ropey::Rope, range: std::ops::Range<usize>) -> usize {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_buffer_object);
        }

        let lines = text.lines_at(range.start).take(range.end - range.start);

        let car_count = lines.clone().map(|x| x.len_chars()).sum();
        if self.quad_buffer_size < car_count {
            unsafe {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (car_count * 2 * std::mem::size_of::<GlyphInstance>()) as gl::types::GLsizeiptr,
                    std::ptr::null(),
                    gl::DYNAMIC_DRAW,
                );
            }
            self.quad_buffer_size = car_count * 2;
        }

        let xpos = 0.0;
        let ypos = 0.0;

        let mut line_offset: f32 = range.start as f32 * self.font_atlas.advance_height;

        let mut i: usize = 0;
        let b;
        unsafe {
            b = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut GlyphInstance;
        }

        for line in lines {
            line_offset += self.font_atlas.advance_height;
            let mut advance: f32 = 0.0;
            for char in line.chars() {
                if char == '\t' {
                    let g = self.font_atlas.get_glyph(' ');
                    advance += g.advance_width * 4.0;
                } else {
                    let g = self.font_atlas.get_glyph(char);
                    unsafe {
                        *b.offset(i as isize) = g.instance(
                            xpos + advance,
                            ypos + line_offset,
                            [213.0 / 255.0, 213.0 / 255.0, 213.0 / 255.0],
                        );
                    }
                    i += 1;
                    advance += g.advance_width;
                }
            }
        }

        unsafe {
            gl::UnmapBuffer(gl::ARRAY_BUFFER);
        }
        i
    }

    pub fn render(
        &mut self,
        text: &ropey::Rope,
        range: std::ops::Range<usize>,
        projection: &matrix::Matrix,
    ) {
        self.program.set_used();
        self.set_projection(projection);

        let size = self.fill_buffer(text, range);

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BlendFunc(gl::SRC1_COLOR, gl::ONE_MINUS_SRC1_COLOR);
            gl::BindTexture(gl::TEXTURE_2D, self.font_atlas.texture);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                size as i32,
            );
        }

        check_error!();
    }
}
