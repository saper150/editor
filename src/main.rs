extern crate gl;

extern crate glfw;
use glfw::Context;

mod app;
mod check_error;
mod cursor;
mod font;
mod highlight;
mod matrix;
mod process_keyboard;
mod rect;
mod render;
mod shaders;
mod text;
mod timer;

use app::{projection_from_size, App};

fn process_event(app: &mut App, event: &glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            app.projection = projection_from_size(*width, *height);
            unsafe {
                gl::Viewport(0, 0, *width, *height);
            }
            app.should_rerender = true;
        }
        glfw::WindowEvent::Refresh => {
            app.should_rerender = true;
        }
        glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
            process_keyboard::process_keyboard(app, key, scancode, action, modifiers);
        }
        glfw::WindowEvent::Char(char) => {
            process_keyboard::process_char(app, char);
        }
        _ => {}
    }
}

use std::env;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

    window.set_framebuffer_size_polling(true);
    window.set_refresh_polling(true);
    window.set_char_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::ClearColor(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0, 1.0);
        gl::Enable(gl::BLEND);
        gl::Enable(gl::MULTISAMPLE);
    }

    let file_path = if env::args().count() > 1 {
        env::args().last().unwrap()
    } else {
        "./text.txt".into()
    };

    let mut app = App::new(window, 800, 600, &file_path);

    while !app.window.should_close() {
        glfw.wait_events();

        for (_, event) in glfw::flush_messages(&events) {
            process_event(&mut app, &event);
        }
        render::render_app(&mut app);
    }

    return;
}
