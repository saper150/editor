extern crate gl;

use crate::check_error;

pub struct QuadIndexBuffer {
    buffer_id: gl::types::GLuint,
    pub size: gl::types::GLsizeiptr,
}

type QuadIndices = [gl::types::GLuint; 6];

fn quad_indices(index: gl::types::GLuint) -> QuadIndices {
    [
        0 + index * 4,
        1 + index * 4,
        2 + index * 4,
        0 + index * 4,
        2 + index * 4,
        3 + index * 4,
    ]
}

impl QuadIndexBuffer {
    pub fn new(size: u32) -> QuadIndexBuffer {
        let mut buffer_id: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer_id);
        }
        let mut b = QuadIndexBuffer {
            buffer_id: buffer_id,
            size: 0,
        };
        b.ensure_size(size / 2);
        b
    }

    pub fn ensure_size(&mut self, size: u32) {
        if size as gl::types::GLsizeiptr <= self.size {
            return;
        }
        self.size = (size * 2) as gl::types::GLsizeiptr;
        let mut indices: Vec<QuadIndices> = Vec::new();
        indices.reserve(self.size as usize);

        for i in 0..self.size {
            indices.push(quad_indices(i as gl::types::GLuint));
        }

        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.buffer_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.size as usize * std::mem::size_of::<QuadIndices>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
        check_error!();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.buffer_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}
