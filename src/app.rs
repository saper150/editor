extern crate gl;
extern crate glfw;

use crate::font;
use crate::gl_buffer;
use crate::matrix;
use crate::rect;
use crate::undo;

use font::font_renderer::FontRenderer;
use rect::rect_renderer::RectRenderer;
#[derive(Copy, Clone, Debug)]
pub struct CursorPosition {
    pub x: i64,
    pub y: i64,
    pub remembered_x: i64,
}

impl CursorPosition {
    pub fn new() -> CursorPosition {
        CursorPosition {
            x: 0,
            y: 0,
            remembered_x: 0,
        }
    }
}

pub struct App {
    pub font_renderer: FontRenderer,
    pub rect_renderer: RectRenderer,
    pub projection: matrix::Matrix,
    pub cursor_position: CursorPosition,
    pub scroll: (f32, f32),
    pub quad_index_buffer: gl_buffer::QuadIndexBuffer,
    pub should_rerender: bool,
    pub window: glfw::Window,
    pub text: ropey::Rope,
    pub undo: undo::UndoState,
    pub selection: Option<(i64, i64)>
}

pub fn projection_from_size(width: i32, height: i32) -> matrix::Matrix {
    matrix::orto(matrix::OrtoParams {
        left: 0.0,
        right: width as f32,
        top: 0.0,
        bottom: -(height as f32),
        far: 1.0,
        near: 0.0,
    })
}

impl App {
    pub fn new(window: glfw::Window, width: i32, height: i32) -> App {
        let t = include_str!("text.txt");
        let text = ropey::Rope::from_str(t);
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
        return App {
            font_renderer: FontRenderer::new(),
            rect_renderer: RectRenderer::new(),
            should_rerender: true,
            window: window,
            cursor_position: CursorPosition::new(),
            scroll: (0.0, 0.0),
            projection: projection_from_size(width, height),
            quad_index_buffer: gl_buffer::QuadIndexBuffer::new(1000),
            undo: undo::UndoState::new(text.clone()),
            text: text,
            selection: None,
        };
    }
}
