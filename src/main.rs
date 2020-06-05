extern crate gl;

extern crate glfw;
use glfw::{Context};

mod font;
mod matrix;
mod shaders;
mod check_error;

use font::font_renderer::FontRenderer;

pub struct App {
    font_renderer: FontRenderer,
    projection: matrix::Matrix,
    should_rerender: bool,
    window: glfw::Window,
    text: String,
}

fn projection_from_size(width: i32, height: i32) -> matrix::Matrix {
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
    fn new(window: glfw::Window ,width: i32, height: i32) -> App {
        let text = include_str!("text.txt").to_string();
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
        return App {
            font_renderer: FontRenderer::new(),
            should_rerender: true,
            window: window,
            projection: projection_from_size(width, height),
            text: text,
        };
    }
}


fn process_event(app: &mut App, event: &glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            app.projection = projection_from_size(*width, *height);
            unsafe {
                gl::Viewport(0, 0, *width, *height);
            }
            app.should_rerender = true;
        },
        glfw::WindowEvent::Refresh => {
            app.should_rerender = true;
        },
        glfw::WindowEvent::Key(key, _scancode, action, _modifiers) => {
            if *action == glfw::Action::Release {
                return
            }

            match key {
                glfw::Key::Enter => {
                    app.text.push_str("\n");
                },
                glfw::Key::Tab => {
                    app.text.push_str("    ");
                },
                glfw::Key::Backspace => {
                    app.text.pop();
                },
                _ => {},
            }
            app.should_rerender = true;
        },
        glfw::WindowEvent::Char(character) => {
            app.text.push_str(&character.to_string());
            app.should_rerender = true;
        }
        _ => {}
    }
}

fn render_app(app: &mut App) {

    if app.should_rerender {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        app.font_renderer.render(&app.text, &app.projection);
        app.window.swap_buffers();

        app.should_rerender = false;
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

    window.set_framebuffer_size_polling(true);
    window.set_refresh_polling(true);
    window.set_char_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::ClearColor(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0, 1.0);
        gl::Enable(gl::BLEND);
        gl::Enable(gl::MULTISAMPLE);
        // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

    }

    let mut app = App::new(window ,800, 600);

    while !app.window.should_close() {
        glfw.wait_events();

        for (_, event) in glfw::flush_messages(&events) {
            process_event(&mut app, &event);
        }
        render_app(&mut app);

    }
    return;

}
