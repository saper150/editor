extern crate freetype as ft;
extern crate gl;

use crate::shaders;
use std::ffi::CString;

use crate::check_error;
use crate::font::font::FontAtlas;
use crate::gl_buffer::QuadIndexBuffer;
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
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        let shader_program = create_shader_program();
        let transform_loc;
        unsafe {
            transform_loc = gl::GetUniformLocation(
                shader_program.id,
                CString::new("projection").unwrap().as_ptr(),
            );

            if transform_loc == -1 {
                panic!("location not found")
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

    fn fill_quads(&mut self, text: &ropey::Rope, range: std::ops::Range<usize>) -> usize {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_buffer_object);
        }

        let lines = text.lines_at(range.start).take(range.end - range.start);

        let car_count = lines.clone().map(|x| { x.len_chars() }).sum();
        if self.quad_buffer_size < car_count {
            unsafe {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (car_count * 2 * std::mem::size_of::<[[f32; 4]; 4]>()) as gl::types::GLsizeiptr,
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
            b = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut [[f32; 4]; 4];
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
                        *b.offset(i as isize) = g.quad(xpos + advance, ypos + line_offset);
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
        index_buffer: &mut QuadIndexBuffer,
    ) {
        self.program.set_used();
        self.set_projection(projection);

        let size = self.fill_quads(text, range);
        index_buffer.ensure_size(size as u32);

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BlendFunc(gl::SRC1_COLOR, gl::ONE_MINUS_SRC1_COLOR);

            gl::BindTexture(gl::TEXTURE_2D, self.font_atlas.texture);

            index_buffer.bind();

            gl::DrawElements(
                gl::TRIANGLES,
                (size * 6) as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            index_buffer.unbind();
        }

        check_error!();
    }
}
