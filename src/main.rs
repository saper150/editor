extern crate gl;

extern crate glfw;
use glfw::Context;

mod app;
mod check_error;
mod font;
mod gl_buffer;
mod matrix;
mod process_keyboard;
mod rect;
mod shaders;
mod timer;

use app::{projection_from_size, App};
use rect::rect_renderer::{create_rect, RectRenderer};

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

fn render_app(app: &mut App) {
    if app.should_rerender {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        {
            let _e = timer::Timer::new();
            let p = matrix::mul(&app.projection, &matrix::translate(0.0, -app.scroll.1, 0.0));

            app.font_renderer
                .render(&app.text, &p, &mut app.quad_index_buffer);

            let char_width = 8.0;

            let width = 2.0;
            let height = app.font_renderer.font_atlas.advance_height;

            let mut x = char_width * app.cursor_position.x as f32;
            let y = (height * (app.cursor_position.y + 2) as f32)
                - (app.font_renderer.font_atlas.face.ascender() >> 6) as f32;

            let tabs_count = app
                .text
                .line(app.cursor_position.y)
                .chars()
                .take(app.cursor_position.x)
                .fold(0, |acc, c| if c == '\t' { acc + 1 } else { acc });

            x += tabs_count as f32 * char_width * 3.0;

            app.rect_renderer.render(
                &create_rect(x, -y, width, -height, [1.0, 1.0, 1.0, 1.0]),
                &p,
                &mut app.quad_index_buffer,
            )
        }

        app.window.swap_buffers();
        app.should_rerender = false;
    }
}

fn main() {
    // println!(
    //     "{:#?}",
    //     matrix::mul(
    //         matrix::translate(5.0, 5.0, 0.0),
    //         matrix::translate(0.0, 5.0, 0.0)
    //     )
    // );

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

    let mut app = App::new(window, 800, 600);

    while !app.window.should_close() {
        glfw.wait_events();

        for (_, event) in glfw::flush_messages(&events) {
            process_event(&mut app, &event);
        }
        render_app(&mut app);
    }
    return;
}
