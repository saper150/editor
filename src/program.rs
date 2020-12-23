extern crate gl;

extern crate glfw;
use glfw::{Action, Context};

use crate::render;
use crate::scroll;
use crate::{app, process_keyboard::KeyAction};
use crate::{app::visible_range_x, process_keyboard};
use app::{projection_from_size, App};

use std::{env, time::Instant};

fn clamp_scroll(app: &mut App) {
    let visible_range = app::visible_range(app, app.scroll.target_scroll.y);
    let cursor_position = app.text.get_cursor().position;
    if cursor_position.y > visible_range.end as i64 - 3 {
        let target_scroll =
            cursor_position.y + 3 - ((visible_range.end - visible_range.start) as i64);
        scroll::scroll_to(&mut app.scroll, target_scroll as f32)
    }

    if cursor_position.y < (visible_range.start + 1) as i64 {
        let target_scroll = (cursor_position.y - 1).max(0);
        scroll::scroll_to(&mut app.scroll, target_scroll as f32);
    }

    let range_x = visible_range_x(app, app.scroll.current_scroll.x);

    if cursor_position.x > range_x.end as i64 - 1 {
        let target_scroll = cursor_position.x + 1 - ((range_x.end - range_x.start) as i64);
        scroll::scroll_to_x(&mut app.scroll, target_scroll as f32);
    }

    if cursor_position.x < (range_x.start + 1) as i64 {
        let target_scroll = (cursor_position.x - 1).max(0);
        scroll::scroll_to_x(&mut app.scroll, target_scroll as f32);
    }
}

pub struct Program {
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    pub app: App,
    // pub task_executor: Executor<'t>,
}

impl<'t> Program {
    pub fn new() -> Program {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

        let (mut window, events) = glfw
            .create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_framebuffer_size_polling(true);
        window.set_refresh_polling(true);
        window.set_char_polling(true);

        window.make_current();
        window.set_key_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

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

        let app = App::new(window, glfw, 800, 600, file_path);

        Program { events, app }
    }

    pub fn run(self) {
        let mut app = self.app;

        let mut now = Instant::now();

        while !app.window.should_close() {
            app.glfw.wait_events();

            let dt = now.elapsed().as_secs_f32();
            now = Instant::now();

            for (_, event) in glfw::flush_messages(&self.events) {
                Program::process_event(&mut app, &event);
            }

            if scroll::advance_scroll(&mut app.scroll, dt) {
                app.glfw.post_empty_event();
                app.should_rerender = true
            }

            render::render_app(&mut app);
        }
    }

    pub fn process_event(app: &mut App, event: &glfw::WindowEvent) {
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
            glfw::WindowEvent::Key(key, _scancode, action, modifiers) => {
                if *action == Action::Press || *action == Action::Repeat {
                    let key = KeyAction {
                        key: *key,
                        modifiers: *modifiers,
                    };
                    process_keyboard::process_keyboard(app, key);
                    clamp_scroll(app);
                    app.should_rerender = true;
                }
            }
            glfw::WindowEvent::Char(char) => {
                process_keyboard::process_char(app, char);
                clamp_scroll(app);
            }
            _ => {}
        }
    }
}
