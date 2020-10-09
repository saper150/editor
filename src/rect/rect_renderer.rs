use crate::shaders;
use std::ffi::CString;

use crate::check_error;
use crate::matrix;

#[repr(C)]
pub struct RectInstance {
    pos: [f32; 2],
    dimensions: [f32; 2],
    color: [f32; 3],
}

pub fn create_rect(x: f32, y: f32, width: f32, height: f32, color: [f32; 3]) -> RectInstance {
    RectInstance {
        pos: [x, y],
        dimensions: [width, height],
        color: color,
    }
}

pub struct RectRenderer {
    program: shaders::Program,
    vao: gl::types::GLuint,
    instance_buffer_object: gl::types::GLuint,
    index_buffer: gl::types::GLuint,
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

        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<RectInstance>() as gl::types::GLint,
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
                std::mem::size_of::<RectInstance>() as gl::types::GLint,
                (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );

            gl::VertexAttribDivisor(1, 1);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<RectInstance>() as gl::types::GLint,
                (4 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl::VertexAttribDivisor(2, 1);
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
            instance_buffer_object: vbo,
            index_buffer: index_buffer,
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

    pub fn render(&mut self, rect: &Vec<RectInstance>, projection: &matrix::Matrix) {
        self.program.set_used();
        self.set_projection(projection);

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instance_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<RectInstance>() * rect.len()) as isize,
                rect.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
            check_error!();
        }

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                rect.len() as i32,
            );
        }

        check_error!();
    }
}
