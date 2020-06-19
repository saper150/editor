use crate::app;
use crate::undo;

use app::App;

use glfw::Key;


const PAGE_SIZE: i64 = 20;

fn is_last_line(app: &App) -> bool {
    return app.text.len_lines() as i64 == app.cursor_position.y + 1;
}


fn move_to_last_char(app: &mut App) {
    if is_last_line(app) {
        move_cursor_x(
            app,
            app.text.line(app.cursor_position.y as usize).len_chars() as i64,
        );
    } else {
        move_cursor_x(
            app,
            app.text.line(app.cursor_position.y as usize).len_chars() as i64 - 1,
        );
    }
}

fn move_cursor_x(app: &mut App, x: i64) {
    if x < 0 {
        if app.cursor_position.y != 0 {
            move_cursor_y(app, app.cursor_position.y - 1);
            app.cursor_position.x =
                app.text.line(app.cursor_position.y as usize).len_chars() as i64;
        } else {
            app.cursor_position.x = 0;
        }
    } else if x >= app.text.line(app.cursor_position.y as usize).len_chars() as i64 {
        if is_last_line(app) {
            app.cursor_position.x =
                app.text.line(app.cursor_position.y as usize).len_chars() as i64;
        } else {
            move_cursor_y(app, app.cursor_position.y + 1);
            app.cursor_position.x = 0;
        }
    } else {
        app.cursor_position.x = x;
    }
    app.cursor_position.remembered_x = app.cursor_position.x;
}

fn move_cursor_y(app: &mut App, y: i64) {
    let cursor = &mut app.cursor_position;

    cursor.y = y.max(0).min(app.text.len_lines() as i64 - 1);

    if app.text.line(cursor.y as usize).len_chars() == 0 {
        cursor.x = 0
    } else {
        cursor.x = cursor
            .remembered_x
            .max(0)
            .min(app.text.line(cursor.y as usize).len_chars() as i64 - 1);

        if cursor.y as usize + 1 == app.text.len_lines() {
            cursor.x += 1;
        }
    }
}

pub fn process_char(app: &mut App, char: &char) {
    let mut tmp = [0; 4];
    undo::insert_text(app, char.encode_utf8(&mut tmp));
    app.should_rerender = true;
}

pub fn process_keyboard(
    app: &mut App,
    key: &glfw::Key,
    _scancode: &i32,
    action: &glfw::Action,
    _modifiers: &glfw::Modifiers,
) {
    if *action == glfw::Action::Release {
        return;
    }

    if _modifiers.contains(glfw::Modifiers::Control) {
        match key {
            Key::Z => {
                undo::back(app);
            }
            Key::Y => {
                undo::forward(app);
            }
            _ => {}
        }
    }

    match key {
        Key::Enter => {
            undo::insert_text(app, "\n");
        }
        Key::Tab => {
            undo::insert_text(app, "\t");
        }
        Key::Backspace => {
            if app.cursor_position.x != 0 || app.cursor_position.y != 0 {
                let start = if app.cursor_position.x == 0 {
                    (
                        (app.cursor_position.y - 1) as usize,
                        app.text
                            .line(app.cursor_position.y as usize - 1)
                            .len_chars()
                            - 1,
                    )
                } else {
                    (
                        app.cursor_position.y as usize,
                        (app.cursor_position.x - 1) as usize,
                    )
                };

                undo::delete_text(app, &undo::Range::new(start, (start.0, start.1 + 1)));
            }
        }
        Key::Delete => {
            if app.text.line(app.cursor_position.y as usize).len_chars()
                > app.cursor_position.x as usize
            {
                let start = (
                    app.cursor_position.y as usize,
                    app.cursor_position.x as usize,
                );
                let end = (
                    app.cursor_position.y as usize,
                    app.cursor_position.x as usize + 1,
                );

                undo::delete_text(app, &undo::Range::new(start, end));
            }
        }
        Key::Left => {
            move_cursor_x(app, app.cursor_position.x - 1);
        }
        Key::Right => {
            move_cursor_x(app, app.cursor_position.x + 1);
        }
        Key::Up => {
            move_cursor_y(app, app.cursor_position.y - 1);
            if (app.cursor_position.y as f32 * app.font_renderer.font_atlas.advance_height)
                < -app.scroll.1
            {
                app.scroll.1 += app.font_renderer.font_atlas.advance_height as f32;
            }
        }
        Key::Down => {
            move_cursor_y(app, app.cursor_position.y + 1);
            let (_, y_height) = app.window.get_framebuffer_size();

            if (app.cursor_position.y as f32 * app.font_renderer.font_atlas.advance_height)
                > -app.scroll.1 + (y_height as f32 - app.font_renderer.font_atlas.advance_height)
            {
                app.scroll.1 -= app.font_renderer.font_atlas.advance_height as f32;
            }
        }
        Key::End => {
            move_to_last_char(app);
        }
        Key::Home => {
            move_cursor_x(app, 0);
        }
        Key::PageDown => {
            move_cursor_y(app, app.cursor_position.y + PAGE_SIZE);
            app.scroll.1 -= app.font_renderer.font_atlas.advance_height * PAGE_SIZE as f32;
            app.scroll.1 = app.scroll.1.max(
                app.text.len_lines() as f32 * -app.font_renderer.font_atlas.advance_height as f32,
            )
        }
        Key::PageUp => {
            move_cursor_y(app, app.cursor_position.y - PAGE_SIZE);
            app.scroll.1 += app.font_renderer.font_atlas.advance_height * PAGE_SIZE as f32;
            app.scroll.1 = app.scroll.1.min(0.0);
        }
        _ => {}
    }
    app.should_rerender = true;
}
