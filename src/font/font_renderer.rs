extern crate freetype as ft;
extern crate gl;

use ropey::RopeSlice;
use crate::timer;

use crate::shaders;
use std::{ffi::CString, mem::MaybeUninit};

use crate::check_error;
use crate::offset_of;

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
    buffer_position: isize,
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
            offset_of!(GlyphInstance, pos) as *const gl::types::GLvoid,
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
            offset_of!(GlyphInstance, dimensions) as *const gl::types::GLvoid,
        );
        gl::VertexAttribDivisor(1, 1);

        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<GlyphInstance>() as gl::types::GLint,
            offset_of!(GlyphInstance, uv_pos) as *const gl::types::GLvoid,
        );
        gl::VertexAttribDivisor(2, 1);

        gl::EnableVertexAttribArray(3);
        gl::VertexAttribPointer(
            3,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<GlyphInstance>() as gl::types::GLint,
            offset_of!(GlyphInstance, uv_dimensions) as *const gl::types::GLvoid,
        );
        gl::VertexAttribDivisor(3, 1);

        gl::EnableVertexAttribArray(4);
        gl::VertexAttribPointer(
            4,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<GlyphInstance>() as gl::types::GLint,
            offset_of!(GlyphInstance, color) as *const gl::types::GLvoid,
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
            buffer_position: 0,

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

    fn add_line(
        &mut self,
        buff: *mut GlyphInstance,
        line_number: usize,
        line: impl Iterator<Item = char>,
    ) {
        let mut advance: f32 = 0.0;
        let line_offset = line_number as f32 * self.font_atlas.advance_height as f32;
        for char in line {
            if char == '\n' {
                continue;
            }
            if char == '\t' {
                let g = self.font_atlas.get_glyph(' ');
                advance += g.advance_width * 4.0;
            } else {
                let g = self.font_atlas.get_glyph(char);
                unsafe {
                    *buff.offset(self.buffer_position) = g.instance(
                        advance,
                        line_offset,
                        [213.0 / 255.0, 213.0 / 255.0, 213.0 / 255.0],
                    );
                }
                self.buffer_position += 1;
                advance += g.advance_width;
            }
        }
    }

    fn ensure_buffer_size(&mut self, size: usize) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_buffer_object);
        }
        if self.quad_buffer_size < size {
            unsafe {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (size * 2 * std::mem::size_of::<GlyphInstance>()) as gl::types::GLsizeiptr,
                    std::ptr::null(),
                    gl::DYNAMIC_DRAW,
                );
            }
            self.quad_buffer_size = size * 2;
        }
    }

    fn fill_buffer<'a>(
        &mut self,
        buffer: *mut GlyphInstance,
        lines: impl Iterator<Item = RopeSlice<'a>>,
    ) {
        let mut current_line: usize = 0;
        for line in lines {
            current_line += 1;
            self.add_line(buffer, current_line, line.chars());
        }
    }

    fn draw_buffer(&mut self) {
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
                self.buffer_position as i32,
            );
        }
        self.buffer_position = 0;
    }

    pub fn render_text_with_line_numbers(
        &mut self,
        text: &ropey::Rope,
        range: std::ops::Range<usize>,
        projection: &matrix::Matrix,
    ) {
        let lines = text
            .lines_at(range.start.min(text.len_lines()))
            .take(range.end - range.start);

        let car_count: usize = lines.clone().map(|x| x.len_chars()).sum();
        self.ensure_buffer_size(car_count + range.len() * 15);

        let buffer =
            unsafe { gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut GlyphInstance };

		{
			
			timer!("text_buffer");
			self.fill_buffer(buffer, lines);
		}

		{
			timer!("line_numbers_buffer");
			self.fill_line_numbers(buffer, range);
		}

        unsafe {
            gl::UnmapBuffer(gl::ARRAY_BUFFER);
        }

        self.program.set_used();
        self.set_projection(projection);

		{
			timer!("draw");
			self.draw_buffer();
		}


        check_error!();
    }

    fn fill_line_numbers(&mut self, buffer: *mut GlyphInstance, range: std::ops::Range<usize>) {

		let mut stack_buffer: [u8; 20] = unsafe { MaybeUninit::uninit().assume_init() };

        for (index, range_value) in range.clone().into_iter().enumerate() {
            let n = itoa::write(&mut stack_buffer[..], range_value + 1).unwrap();

            self.add_line(
                buffer,
                index + 1,
                CharBytesIterator::new(&stack_buffer[..n]),
            );
        }
    }
}

struct CharBytesIterator<'a> {
    src: &'a [u8],
    current: usize,
}

impl<'a> CharBytesIterator<'a> {
    pub fn new(buff: &'a [u8]) -> CharBytesIterator<'a> {
        CharBytesIterator {
            src: buff,
            current: 0,
        }
    }
}

impl<'a> Iterator for CharBytesIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.current < self.src.len() {
            let s = Some(self.src[self.current] as char);
            self.current += 1;
            return s;
        } else {
            return None;
        }
    }
}
