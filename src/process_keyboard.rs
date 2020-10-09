use crate::app;
use crate::cursor;
use crate::text;

use app::App;
use cursor::Point;

use glfw::Key;
use std::fs;

const PAGE_SIZE: i64 = 20;

fn is_last_line(app: &App) -> bool {
    return app.text.text.len_lines() as i64 == app.cursor.position.y + 1;
}

fn move_to_last_char(app: &mut App) {
    if is_last_line(app) {
        move_cursor_x(
            app,
            app.text
                .text
                .line(app.cursor.position.y as usize)
                .len_chars() as i64,
        );
    } else {
        move_cursor_x(
            app,
            app.text
                .text
                .line(app.cursor.position.y as usize)
                .len_chars() as i64
                - 1,
        );
    }
}

fn move_cursor_x(app: &mut App, x: i64) {
    if x < 0 {
        if app.cursor.position.y != 0 {
            move_cursor_y(app, app.cursor.position.y - 1);
            app.cursor.position.x = app
                .text
                .text
                .line(app.cursor.position.y as usize)
                .len_chars() as i64;
        } else {
            app.cursor.position.x = 0;
        }
    } else if x
        >= app
            .text
            .text
            .line(app.cursor.position.y as usize)
            .len_chars() as i64
    {
        if is_last_line(app) {
            app.cursor.position.x = app
                .text
                .text
                .line(app.cursor.position.y as usize)
                .len_chars() as i64;
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

    cursor.position.y = y.max(0).min(app.text.text.len_lines() as i64 - 1);

    if app.text.text.line(cursor.position.y as usize).len_chars() == 0 {
        cursor.position.x = 0
    } else {
        cursor.position.x = cursor
            .remembered_x
            .max(0)
            .min(app.text.text.line(cursor.position.y as usize).len_chars() as i64 - 1);

        if cursor.position.y as usize + 1 == app.text.text.len_lines() {
            cursor.position.x += 1;
        }
    }
}

fn process_selection(app: &mut App, modifiers: &glfw::Modifiers) {
    if modifiers.contains(glfw::Modifiers::Shift) {
        if app.cursor.selection.is_none() {
            app.cursor.selection = Some(app.cursor.position.clone());
        }
    } else {
        app.cursor.selection = None;
    }
}

fn selection_range(app: &App) -> std::ops::Range<usize> {
    let a = app.cursor.position.to_char(&app.text.text);
    let b = app.cursor.selection.unwrap().to_char(&app.text.text);
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
    text::insert_text(&mut app.text, &mut app.cursor, char.encode_utf8(&mut tmp));
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
                text::undo(&mut app.text, &mut app.cursor);
            }
            Key::Y => {
                text::redo(&mut app.text, &mut app.cursor);
            }
            Key::S => {
                fs::write(&app.file_path, &app.text.text.to_string()).unwrap();
            }
            _ => {}
        }
    }

    match key {
        Key::Enter => {
            text::insert_text(&mut app.text, &mut app.cursor, "\n");
        }
        Key::Tab => {
            text::insert_text(&mut app.text, &mut app.cursor, "\t");
        }
        Key::Backspace => {
            text::delete_text(&mut app.text, &mut app.cursor, text::DeleteKey::Backspace);
        }
        Key::Delete => {
            text::delete_text(&mut app.text, &mut app.cursor, text::DeleteKey::Del);
        }
        Key::Left => {
            process_selection(app, modifiers);

            if modifiers.contains(glfw::Modifiers::Control) {
                let char_idx = app.cursor.position.to_char(&app.text.text);

                let next_char_idx = char_idx
                    - next_word(&mut BackwardIterator {
                        src: &mut app.text.text.chars_at(char_idx),
                    });

                app.cursor.position = Point::from_char(next_char_idx, &app.text.text);
                app.cursor.remembered_x = app.cursor.position.x;
            } else {
                move_cursor_x(app, app.cursor.position.x - 1);
            }
        }
        Key::Right => {
            process_selection(app, modifiers);

            if modifiers.contains(glfw::Modifiers::Control) {
                let char_idx = app.cursor.position.to_char(&app.text.text);

                let next_char_idx = char_idx + next_word(&mut app.text.text.chars_at(char_idx));

                app.cursor.position = Point::from_char(next_char_idx, &app.text.text);

                app.cursor.remembered_x = app.cursor.position.x;
            } else {
                move_cursor_x(app, app.cursor.position.x + 1);
            }
        }
        Key::Up => {
            process_selection(app, modifiers);
            move_cursor_y(app, app.cursor.position.y - 1);
        }
        Key::Down => {
            process_selection(app, modifiers);
            move_cursor_y(app, app.cursor.position.y + 1);
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
            app.scroll.1 = clamp(
                app.scroll.1 + PAGE_SIZE,
                0,
                app.text.text.len_lines() as i64 - 10,
            );
        }
        Key::PageUp => {
            process_selection(app, modifiers);
            move_cursor_y(app, app.cursor.position.y - PAGE_SIZE);
            app.scroll.1 = clamp(
                app.scroll.1 - PAGE_SIZE,
                0,
                app.text.text.len_lines() as i64 - 10,
            );
        }
        _ => {}
    }
}
