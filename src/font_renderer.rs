extern crate gl;

use crate::shaders;
use std::ffi::CString;

use crate::font;
use crate::matrix;
use crate::check_error;

pub struct FontRenderer {
    program: shaders::Program,
    vao: gl::types::GLuint,
    quad_buffer_object: gl::types::GLuint,
    index_buffer: gl::types::GLuint,
    font_atlas: font::FontAtlas,
    transform_loc: gl::types::GLint,
}


fn create_shader_program() -> shaders::Program {
    let vert_shader =
        shaders::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        shaders::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
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

        let mut index_buffer: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut index_buffer);
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
            font_atlas: font::FontAtlas::new(14),
            index_buffer: index_buffer,
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

    pub fn render(&mut self, text: &str, projection: &matrix::Matrix) {
        self.program.set_used();
        self.set_projection(projection);

        let xpos = 0.0;
        let ypos = -100.0;

        let mut v: Vec<[[f32; 4]; 4]> = Vec::new();
        v.reserve(text.len());

        let mut line_offset: f32 = 0.0;
        for line in text.lines() {
            line_offset += self.font_atlas.advance_height;
            let mut advance: f32 = 0.0;
            for char in line.chars() {
                {

                    let g = self.font_atlas.get_glyph(char);
                    v.push(g.quad(xpos + advance, ypos - line_offset));
                    advance += g.advance_width;
                }
            }
        }

        let mut indices: Vec<[i32; 6]> = Vec::new();
        indices.reserve(v.len());
        for i in 0..v.len() {
            indices.push(font::AtlasGlyph::indices(i as i32));
        }

        unsafe {
            gl::BlendFunc(gl::SRC1_COLOR, gl::ONE_MINUS_SRC1_COLOR);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (v.len() * std::mem::size_of::<[[f32; 4]; 4]>()) as gl::types::GLsizeiptr,
                v.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<[i32; 6]>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }

        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindTexture(gl::TEXTURE_2D, self.font_atlas.texture);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);

            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32 * 6,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
        check_error!();
    }
}
