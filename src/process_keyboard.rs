use crate::text;
use crate::{app, text::Selection};

use app::App;

use glfw::Key;
use std::fs;

struct BackwardIterator<'a> {
    src: &'a mut ropey::iter::Chars<'a>,
}

impl<'a> Iterator for BackwardIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.src.prev()
    }
}

pub fn process_char(app: &mut App, char: &char) {
    let mut tmp = [0; 4];
    app.text.insert_text(char.encode_utf8(&mut tmp));
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
                app.text.undo();
            }
            Key::Y => {
                app.text.redo();
            }
            Key::S => {
                fs::write(&app.file_path, &app.text.get_text().to_string()).unwrap();
            }
            _ => {}
        }
    }

    let selection = if modifiers.contains(glfw::Modifiers::Shift) {
        Selection::Select
    } else {
        Selection::NotSelect
    };

    match key {
        Key::Enter => {
            app.text.insert_text("\n");
        }
        Key::Tab => {
            app.text.insert_text("\t");
        }
        Key::Backspace => {
            app.text.delete_text(text::DeleteKey::Backspace);
        }
        Key::Delete => {
            app.text.delete_text(text::DeleteKey::Del);
        }
        Key::Left => {
            if modifiers.contains(glfw::Modifiers::Control) {
                app.text.move_to_prev_word(selection);
            } else {
                app.text.move_cursor(-1, selection);
            }
        }
        Key::Right => {
            if modifiers.contains(glfw::Modifiers::Control) {
                app.text.move_to_next_word(selection);
            } else {
                app.text.move_cursor(1, selection);
            }
        }
        Key::Up => {
            app.text.move_cursor_y(-1, selection);
        }
        Key::Down => {
            app.text.move_cursor_y(1, selection);
        }
        Key::End => {
            if modifiers.contains(glfw::Modifiers::Control) {
                app.text.move_to_end(selection);
            } else {
                app.text.move_to_end_of_line(selection);
            }
        }
        Key::Home => {
            if modifiers.contains(glfw::Modifiers::Control) {
                app.text.move_to_begging(selection);
            } else {
                app.text.move_to_beginning_of_line(selection);
            }
        }
        Key::PageDown => {
            app.text.move_cursor_y(20, selection);
        }
        Key::PageUp => {
            app.text.move_cursor_y(-20, selection);
        }
        Key::V => {
            if modifiers.contains(glfw::Modifiers::Control) {
                if let Some(s) = app.window.get_clipboard_string() {
                    app.text.insert_text(s.as_str())
                }
            }
        }
        Key::C => {
            if modifiers.contains(glfw::Modifiers::Control) {
                if app.text.get_cursor().selection.is_some() {
                    let selection_str = app.text.get_selection();
                    app.window.set_clipboard_string(selection_str.as_str());
                } else {
                    let selection_str = app.text.get_current_line();
                    app.window.set_clipboard_string(selection_str.as_str());
                }
            }
        }
        _ => {}
    }
}
