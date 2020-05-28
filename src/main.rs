extern crate gl;

extern crate glfw;
use glfw::{Action, Context, Key};

mod font;
mod matrix;
mod renderer;
mod shaders;

pub struct App {
    renderer: renderer::Renderer,
    should_rerender: bool,
    window: glfw::Window,
    text: String,
}

impl App {
    fn new(window: glfw::Window ,width: i32, height: i32) -> App {
        let text = include_str!("text.txt").to_string();
        return App {
            renderer: renderer::Renderer::new(width, height),
            should_rerender: true,
            window: window,
            text: text,
        };
    }
}


fn process_event(app: &mut App, event: &glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            app.renderer.on_resize(width.clone(), height.clone());
            app.should_rerender = true;
        },
        glfw::WindowEvent::Refresh => {
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
        app.renderer.render(&app.text);
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

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_refresh_polling(true);
    window.set_char_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

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
