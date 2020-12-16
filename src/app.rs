extern crate gl;
extern crate glfw;

use crate::font;
use crate::matrix;
use crate::rect;
use crate::text;

use crate::scroll;
use scroll::Scroll;

use font::font_renderer::FontRenderer;
use rect::rect_renderer::RectRenderer;

use std::fs::File;

pub struct App {
    pub file_path: String,
    pub font_renderer: FontRenderer,
    pub rect_renderer: RectRenderer,
    pub projection: matrix::Matrix,
    pub scroll: Scroll,
    pub should_rerender: bool,
    pub window: glfw::Window,
    pub glfw: glfw::Glfw,
    pub text: text::Text,
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

pub fn visible_range(app: &App, scroll_y: f32) -> std::ops::Range<usize> {
    let (_, y_size) = app.window.get_framebuffer_size();

    scroll_y as usize
        ..scroll_y as usize + ((y_size as f32) / app.font_renderer.advance_height).ceil() as usize
}

impl App {
    pub fn new(
        window: glfw::Window,
        glfw: glfw::Glfw,
        width: i32,
        height: i32,
        file_path: String,
    ) -> App {
        let text = text::Text::new(File::open(file_path.clone()).unwrap());

        unsafe {
            gl::Viewport(0, 0, width, height);
        }

        return App {
            file_path: file_path,
            font_renderer: FontRenderer::new(),
            rect_renderer: RectRenderer::new(),
            should_rerender: true,
            window: window,
            glfw: glfw,
            scroll: Scroll::new(),
            projection: projection_from_size(width, height),
            text: text,
        };
    }
}
