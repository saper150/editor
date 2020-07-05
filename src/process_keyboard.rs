use crate::app;
use crate::undo;
use crate::cursor;

use app::App;
use cursor::Point;

use glfw::Key;


const PAGE_SIZE: i64 = 20;

fn is_last_line(app: &App) -> bool {
    return app.text.len_lines() as i64 == app.cursor.position.y + 1;
}

fn move_to_last_char(app: &mut App) {
    if is_last_line(app) {
        move_cursor_x(
            app,
            app.text.line(app.cursor.position.y as usize).len_chars() as i64,
        );
    } else {
        move_cursor_x(
            app,
            app.text.line(app.cursor.position.y as usize).len_chars() as i64 - 1,
        );
    }
}

fn move_cursor_x(app: &mut App, x: i64) {
    if x < 0 {
        if app.cursor.position.y != 0 {
            move_cursor_y(app, app.cursor.position.y - 1);
            app.cursor.position.x =
                app.text.line(app.cursor.position.y as usize).len_chars() as i64;
        } else {
            app.cursor.position.x = 0;
        }
    } else if x >= app.text.line(app.cursor.position.y as usize).len_chars() as i64 {
        if is_last_line(app) {
            app.cursor.position.x =
                app.text.line(app.cursor.position.y as usize).len_chars() as i64;
        } else {
            move_cursor_y(app, app.cursor.position.y + 1);
            app.cursor.position.x = 0;
        }
    } else {
        app.cursor.position.x = x;
    }
    app.cursor.remembered_x = app.cursor.position.x;
}


fn move_cursor_y(app: &mut App, y: i64) {
    let cursor = &mut app.cursor;

    cursor.position.y = y.max(0).min(app.text.len_lines() as i64 - 1);

    if app.text.line(cursor.position.y as usize).len_chars() == 0 {
        cursor.position.x = 0
    } else {
        cursor.position.x = cursor
            .remembered_x
            .max(0)
            .min(app.text.line(cursor.position.y as usize).len_chars() as i64 - 1);

        if cursor.position.y as usize + 1 == app.text.len_lines() {
            cursor.position.x += 1;
        }
    }
}

fn process_selection(app: &mut App, modifiers: &glfw::Modifiers) {
    if modifiers.contains(glfw::Modifiers::Shift) {
        if app.selection.is_none() {
            app.selection = Some(app.cursor.position.clone());
        }
    } else {
        app.selection = None;
    }
}


fn selection_range(app: &App) -> std::ops::Range<usize> {
    let a = app.cursor.position.to_char(&app.text);
    let b = app.selection.unwrap().to_char(&app.text);
    a.min(b)..b.max(a)
}

fn clamp(x: i64, min: i64, max: i64) -> i64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

struct BackwardIterator<'a> {
    src: &'a mut ropey::iter::Chars<'a>,
}

impl<'a> Iterator for BackwardIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.src.prev()
    }
}

fn next_word<'a, I>(iter: &mut I) -> usize
where
    I: Iterator<Item = char>,
{
    let mut found_word;
    match iter.next() {
        Some(char) => {
            found_word = char.is_alphanumeric();
        }
        None => return 0,
    }

    iter.take_while(|char| {
        if found_word {
            return char.is_alphanumeric();
        } else if char.is_alphanumeric() {
            found_word = true;
        }
        true
    })
    .count()
        + 1
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
    modifiers: &glfw::Modifiers,
) {
    if *action == glfw::Action::Release {
        return;
    }

    app.should_rerender = true;

    if modifiers.contains(glfw::Modifiers::Control) {
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

            if app.selection.is_some() {
                undo::delete_text(app, selection_range(app));
            } else {
                let char_idx = app.cursor.position.to_char(&app.text);
                if char_idx > 0 {
                    undo::delete_text(app, char_idx - 1..char_idx);
                }
            }

        }
        Key::Delete => {
            if app.selection.is_some() {
                undo::delete_text(app, selection_range(app));
            } else {
                let char_idx = app.cursor.position.to_char(&app.text);
                if char_idx + 1 < app.text.len_chars() {
                    undo::delete_text(app, char_idx..char_idx + 1);
                }
            }
        }
        Key::Left => {
            process_selection(app, modifiers);

            if modifiers.contains(glfw::Modifiers::Control) {
                let char_idx = app.cursor.position.to_char(&app.text);

                let next_char_idx = char_idx
                    - next_word(&mut BackwardIterator {
                        src: &mut app.text.chars_at(char_idx),
                    });

                app.cursor.position = Point::from_char(next_char_idx, &app.text);
                app.cursor.remembered_x = app.cursor.position.x;

            } else {
                move_cursor_x(app, app.cursor.position.x - 1);
            }
        }
        Key::Right => {
            process_selection(app, modifiers);

            if modifiers.contains(glfw::Modifiers::Control) {
                let char_idx = app.cursor.position.to_char(&app.text);

                let next_char_idx = char_idx + next_word(&mut app.text.chars_at(char_idx));

                app.cursor.position = Point::from_char(next_char_idx, &app.text);

                app.cursor.remembered_x = app.cursor.position.x;
            } else {
                move_cursor_x(app, app.cursor.position.x + 1);
            }
        }
        Key::Up => {
            process_selection(app, modifiers);
            move_cursor_y(app, app.cursor.position.y - 1);
            if (app.cursor.position.y) < app.scroll.1 {
                app.scroll.1 -= 1;
            }
        }
        Key::Down => {
            process_selection(app, modifiers);
            move_cursor_y(app, app.cursor.position.y + 1);
            // let (_, y_height) = app.window.get_framebuffer_size();

            // if (app.cursor_position.y as f32 * app.font_renderer.advance_height)
            //     > app.scroll.1 + (y_height as f32 - app.font_renderer.advance_height)
            // {
            //     app.scroll.1 += app.font_renderer.advance_height as f32;
            // }
        }
        Key::End => {
            process_selection(app, modifiers);
            move_to_last_char(app);
        }
        Key::Home => {
            process_selection(app, modifiers);
            move_cursor_x(app, 0);
        }
        Key::PageDown => {
            process_selection(app, modifiers);
            move_cursor_y(app, app.cursor.position.y + PAGE_SIZE);
            app.scroll.1 = clamp(app.scroll.1 + PAGE_SIZE, 0, app.text.len_lines() as i64 - 10);
        }
        Key::PageUp => {
            process_selection(app, modifiers);
            move_cursor_y(app, app.cursor.position.y - PAGE_SIZE);
            app.scroll.1 = clamp(app.scroll.1 - PAGE_SIZE, 0, app.text.len_lines() as i64 - 10);
        }
        _ => {}
    }
}
