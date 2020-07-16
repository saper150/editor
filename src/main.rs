extern crate gl;

extern crate glfw;
use glfw::Context;

mod app;
mod check_error;
mod cursor;
mod font;
mod matrix;
mod process_keyboard;
mod rect;
mod shaders;
mod timer;
mod undo;
mod highlight;

use app::{projection_from_size, App};
use rect::rect_renderer::create_rect;

use cursor::Point;

// use syntect::easy::HighlightLines;
// use syntect::highlighting::{HighlightIterator, HighlightState, Highlighter, Style, ThemeSet};
// use syntect::parsing::{ScopeStack, SyntaxSet, SyntaxSetBuilder, SyntaxDefinition};
// use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// fn f() {
//     let ps = SyntaxSet::load_defaults_newlines();

//     let ts = ThemeSet::load_defaults();
//     let syntax = ps.find_syntax_by_extension("html").unwrap();

    
//     let mut ee = syntect::parsing::ParseState::new(syntax);
//     let s = "<div>abcabcab</div>";

//     let highlighter = Highlighter::new(&ts.themes["base16-ocean.dark"]);
//     let mut highlight_state = HighlightState::new(&highlighter, ScopeStack::new());

//     let ops = ee.parse_line(s, &ps);
//     let mut iter = HighlightIterator::new(&mut highlight_state, &ops[..], s, &highlighter);

//     // let e = syntect::highlighting::Highlighter::new(&ts.themes["base16-ocean.dark"]);

//     // let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

//     // for line in LinesWithEndings::from(s) {
//     //     let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);

//     //     let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
//     //     println!("{}", escaped);
//     // }
// }

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

fn x_to_screen(app: &App, x: i64) -> f32 {
    let char_width = app.font_renderer.char_width;
    x as f32 * char_width
}

fn y_to_screen(app: &App, y: i64) -> f32 {
    let height = app.font_renderer.advance_height;
    ((y + 2) as f32 * height) - app.font_renderer.ascender
}

fn grid_to_screen(app: &App, pos: Point) -> (f32, f32) {
    (x_to_screen(app, pos.x), y_to_screen(app, pos.y))
}

fn render_selection(app: &mut App, projection: &matrix::Matrix) {
    let height = app.font_renderer.advance_height;

    if app.selection.is_none() {
        return;
    }

    let selection = app.selection.unwrap();

    let (start, end) = if selection.y == app.cursor.position.y {
        if selection.x >= app.cursor.position.x {
            (app.cursor.position, selection)
        } else {
            (selection, app.cursor.position)
        }
    } else {
        if selection.y > app.cursor.position.y {
            (app.cursor.position, selection)
        } else {
            (selection, app.cursor.position)
        }
    };

    let mut v = Vec::new();

    let start_screen = grid_to_screen(app, start);
    if start.y == end.y {
        v.push(create_rect(
            start_screen.0,
            start_screen.1,
            x_to_screen(app, end.x - start.x),
            height,
            [0.5, 0.5, 0.5],
        ));
    } else {
        v.push(create_rect(
            start_screen.0,
            start_screen.1,
            x_to_screen(
                app,
                app.text.line(start.y as usize).len_chars() as i64 - start.x,
            ),
            height,
            [0.5, 0.5, 0.5],
        ));

        for (i, l) in app
            .text
            .lines_at(start.y as usize + 1)
            .take((end.y - start.y) as usize - 1)
            .enumerate()
        {
            let line = i as i64 + start.y + 1;

            v.push(create_rect(
                0.0,
                y_to_screen(app, line),
                x_to_screen(app, l.len_chars() as i64),
                height,
                [0.5, 0.5, 0.5],
            ));
        }

        let end_screen = grid_to_screen(app, end);

        v.push(create_rect(
            0.0,
            end_screen.1,
            end_screen.0,
            height,
            [0.5, 0.5, 0.5],
        ));
    };

    app.rect_renderer.render(&v, &projection);
}

fn render_app(app: &mut App) {
    if app.should_rerender {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let visible_range = app::visible_range(app);

        let screen_scroll = app.scroll.1 as f32 * app.font_renderer.advance_height;

        let p = matrix::mul(&app.projection, &matrix::translate(0.0, screen_scroll, 0.0));

        render_selection(app, &p);

        let height = app.font_renderer.advance_height;

        app.font_renderer
            .render(&app.text, visible_range, &p);

        let width = 2.0;

        let screen_pos = grid_to_screen(app, app.cursor.position);

        app.rect_renderer.render(
            &vec![create_rect(
                screen_pos.0,
                screen_pos.1,
                width,
                height,
                [1.0, 1.0, 1.0],
            )],
            &p,
        );

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
