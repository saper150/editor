use crate::shaders;
use std::ffi::CString;

use crate::check_error;
use crate::gl_buffer::QuadIndexBuffer;
use crate::matrix;

type Rect = [[f32; 6]; 4];

pub fn create_rect(x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) -> Rect {
    let top_left = [x, y, color[0], color[1], color[2], color[3]];
    let top_right = [x + width, y, color[0], color[1], color[2], color[3]];
    let bottom_right = [x + width, y + height, color[0], color[1], color[2], color[3]];
    let bottom_left = [x, y + height, color[0], color[1], color[2], color[3]];

    [bottom_left, top_left, top_right, bottom_right]
}

pub struct RectRenderer {
    program: shaders::Program,
    vao: gl::types::GLuint,
    quad_buffer_object: gl::types::GLuint,
    transform_loc: gl::types::GLint,
}

fn create_shader_program() -> shaders::Program {
    let vert_shader =
        shaders::Shader::from_vert_source(&CString::new(include_str!("rect.vert")).unwrap())
            .unwrap();

    let frag_shader =
        shaders::Shader::from_frag_source(&CString::new(include_str!("rect.frag")).unwrap())
            .unwrap();

    shaders::Program::from_shaders(&[vert_shader, frag_shader]).unwrap()
}

impl RectRenderer {
    pub fn new() -> RectRenderer {
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
                2,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
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

        RectRenderer {
            program: shader_program,
            quad_buffer_object: vbo,
            transform_loc: transform_loc,
            vao: vao,
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

    pub fn render(
        &mut self,
        rect: &Rect,
        projection: &matrix::Matrix,
        index_buffer: &mut QuadIndexBuffer,
    ) {
        self.program.set_used();
        self.set_projection(projection);

        let size = 1;

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of::<Rect>() as isize,
                rect.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }

        index_buffer.ensure_size(size as u32);

        unsafe {
			gl::BindVertexArray(self.vao); 
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

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
