extern crate gl;

use crate::shaders;
use rusttype::{Font, Point, Scale};
use std::ffi::CString;

use crate::font;
use crate::matrix;

macro_rules! check_error {
    () => {
        let line = line!();
        unsafe {
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                let message = match error {
                    gl::INVALID_ENUM => "INVALID_ENUM",
                    gl::INVALID_VALUE => "INVALID_VALUE",
                    gl::INVALID_OPERATION => "INVALID_OPERATION",
                    gl::STACK_OVERFLOW => "STACK_OVERFLOW",
                    gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
                    gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
                    gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
                    _ => "Unknown error",
                };
                println!("file: {} error on line {} {}", file!(), line, message);
            }
        }
    };
}

fn init() {
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
}
pub struct Renderer {
    program: shaders::Program,
    projection: matrix::Matrix,
    vao: gl::types::GLuint,
    quad_buffer_object: gl::types::GLuint,
    index_buffer: gl::types::GLuint,
    font_atlas: font::FontAtlas,
    transform_loc: gl::types::GLint,
}

fn projection_from_size(size: glutin::dpi::PhysicalSize<u32>) -> matrix::Matrix {
    matrix::orto(matrix::OrtoParams {
        left: 0.0,
        right: size.width as f32,
        top: 0.0,
        bottom: -(size.height as f32),
        far: 1.0,
        near: 0.0,
    })
}

impl Renderer {
    pub fn on_resize(&mut self, size: glutin::dpi::PhysicalSize<u32>) {
        self.projection = projection_from_size(size);
        unsafe {
            gl::Viewport(0, 0, size.width as i32, size.height as i32);
        }
    }

    pub fn new(size: glutin::dpi::PhysicalSize<u32>) -> Renderer {
        let vert_shader = shaders::Shader::from_vert_source(
            &CString::new(include_str!("triangle.vert")).unwrap(),
        )
        .unwrap();

        let frag_shader = shaders::Shader::from_frag_source(
            &CString::new(include_str!("triangle.frag")).unwrap(),
        )
        .unwrap();

        let shader_program = shaders::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        init();

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

        let mut indedx_buffer: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut indedx_buffer);
        }

        let mut transform_loc = 0;
        unsafe {
            transform_loc = gl::GetUniformLocation(
                shader_program.id,
                CString::new("projection").unwrap().as_ptr(),
            );

            if transform_loc == -1 {
                panic!("location not found")
            }
        }

        unsafe {
            gl::Viewport(0, 0, size.width as i32, size.height as i32);
        }

        Renderer {
            program: shader_program,
            vao: vao,
            quad_buffer_object: vbo,
            projection: projection_from_size(size),
            font_atlas: font::FontAtlas::new(),
            index_buffer: indedx_buffer,
            transform_loc: transform_loc,
        }
    }

    fn set_projection(&self) {
        self.program.set_used();

        unsafe {
            gl::UniformMatrix4fv(
                self.transform_loc,
                1,
                gl::FALSE,
                self.projection.as_ptr() as *const f32,
            );
        }
    }

    pub fn render(&self) {
        self.program.set_used();
        self.set_projection();

        let xpos = 100.0;
        let ypos = -100.0;

        let text = "glyph";

        let mut v: Vec<[[f32; 4]; 4]> = Vec::new();
        let mut advance: f32 = 0.0;

        let mut indices: Vec<[i32; 6]> = Vec::new();
        indices.reserve(text.len());

        for (i, char) in text.chars().enumerate() {
            let g = self.font_atlas.glyphs.get(&char).unwrap();
            v.push(g.quad(xpos + advance, ypos));
            advance += g.advance_width;
            indices.push(font::AtlasGlyph::indices(i as i32));
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                v.len() as isize * std::mem::size_of::<[f32; 24]>() as gl::types::GLsizeiptr,
                v.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                indices.len() as isize * std::mem::size_of::<[i32; 6]>() as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
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
