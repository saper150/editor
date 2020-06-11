extern crate freetype as ft;
extern crate gl;

use crate::shaders;
use std::ffi::CString;

use crate::check_error;
use crate::font::font::FontAtlas;
use crate::gl_buffer::QuadIndexBuffer;
use crate::matrix;

pub struct FontRenderer {
    program: shaders::Program,
    vao: gl::types::GLuint,
    quad_buffer_object: gl::types::GLuint,
    quad_buffer_size: usize,
    pub font_atlas: FontAtlas,
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
            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                0,         // index of the generic vertex attribute ("layout (location = 0)")
                4,         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (4 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(), // offset of the first component
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

        FontRenderer {
            program: shader_program,
            vao: vao,
            quad_buffer_object: vbo,
            quad_buffer_size: 0,
            font_atlas: FontAtlas::new(14),
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

    fn fill_quads(&mut self, text: &ropey::Rope) -> usize {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_buffer_object);
        }

        let car_count = text.chars().count();
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

        let mut line_offset: f32 = 0.0;

        let mut i: usize = 0;
        let b;
        unsafe {
            b = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut [[f32; 4]; 4];
        }

        for line in text.lines() {
            line_offset += self.font_atlas.advance_height;
            let mut advance: f32 = 0.0;
            for char in line.chars() {
                if char == '\t' {
                    let g = self.font_atlas.get_glyph(' ');
                    advance += g.advance_width * 4.0;
                } else {
                    let g = self.font_atlas.get_glyph(char);
                    unsafe {
                        *b.offset(i as isize) = g.quad(xpos + advance, ypos - line_offset);
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
        projection: &matrix::Matrix,
        index_buffer: &mut QuadIndexBuffer,
    ) {
        self.program.set_used();
        self.set_projection(projection);

        let size = self.fill_quads(text);
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
