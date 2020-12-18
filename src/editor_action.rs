use std::fs;

use memchr::memchr_iter;

use crate::{
    app::App,
    text::{remove_crlf_from_buff, DeleteDirection, Selection},
};

#[derive(Clone, Copy)]
pub enum EditorAction {
    CursorUp,
    CursorUpSelect,

    CursorDown,
    CursorDownSelect,

    CursorLeft,
    CursorLeftSelect,
    CursorPrevWord,
    CursorPrevWordSelect,

    CursorRight,
    CursorRightSelect,

    CursorNextWord,
    CursorNextWordSelect,

    CursorHome,
    CursorHomeSelect,

    CursorEnd,
    CursorEndSelect,

    CursorPageDown,
    CursorPageDownSelect,

    CursorPageUp,
    CursorPageUpSelect,

    CursorEndOfFile,
    CursorEndOfFileSelect,

    CursorBeginningOfFile,
    CursorBeginningOfFileSelect,

    Copy,
    Paste,
    Cut,

    DeleteForward,
    DeleteBackward,

    Undo,
    Redo,

    Save,
}

pub fn dispatch_action(app: &mut App, action: EditorAction) {
    match action {
        EditorAction::CursorUp => {
            app.text.move_cursor_y(-1, Selection::NotSelect);
        }
        EditorAction::CursorUpSelect => {
            app.text.move_cursor_y(-1, Selection::Select);
        }
        EditorAction::CursorDown => {
            app.text.move_cursor_y(1, Selection::NotSelect);
        }
        EditorAction::CursorDownSelect => {
            app.text.move_cursor_y(1, Selection::Select);
        }
        EditorAction::CursorLeft => {
            app.text.move_cursor(-1, Selection::NotSelect);
        }
        EditorAction::CursorLeftSelect => {
            app.text.move_cursor(-1, Selection::Select);
        }
        EditorAction::CursorPrevWord => {
            app.text.move_to_prev_word(Selection::NotSelect);
        }
        EditorAction::CursorPrevWordSelect => {
            app.text.move_to_prev_word(Selection::Select);
        }
        EditorAction::CursorRight => {
            app.text.move_cursor(1, Selection::NotSelect);
        }
        EditorAction::CursorRightSelect => {
            app.text.move_cursor(1, Selection::Select);
        }
        EditorAction::CursorNextWord => app.text.move_to_next_word(Selection::NotSelect),
        EditorAction::CursorNextWordSelect => app.text.move_to_next_word(Selection::Select),
        EditorAction::CursorHome => {
            app.text.move_to_beginning_of_line(Selection::NotSelect);
        }
        EditorAction::CursorHomeSelect => {
            app.text.move_to_beginning_of_line(Selection::Select);
        }
        EditorAction::CursorEnd => app.text.move_to_end_of_line(Selection::NotSelect),
        EditorAction::CursorEndSelect => app.text.move_to_end_of_line(Selection::Select),
        EditorAction::CursorPageDown => {
            app.text.move_cursor_y(20, Selection::NotSelect);
        }
        EditorAction::CursorPageDownSelect => {
            app.text.move_cursor_y(20, Selection::Select);
        }
        EditorAction::CursorPageUp => {
            app.text.move_cursor_y(-20, Selection::NotSelect);
        }
        EditorAction::CursorPageUpSelect => {
            app.text.move_cursor_y(-20, Selection::Select);
        }
        EditorAction::CursorEndOfFile => {
            app.text.move_to_end(Selection::NotSelect);
        }
        EditorAction::CursorEndOfFileSelect => {
            app.text.move_to_end(Selection::Select);
        }
        EditorAction::CursorBeginningOfFile => {
            app.text.move_to_begging(Selection::NotSelect);
        }
        EditorAction::CursorBeginningOfFileSelect => {
            app.text.move_to_begging(Selection::Select);
        }
        EditorAction::Copy => {
            if let Some(selection_text) = app.text.get_selection_str() {
                app.window.set_clipboard_string(selection_text.as_str());
            } else {
                let selection_str = app.text.get_current_line();
                app.window.set_clipboard_string(selection_str.as_str());
            }
        }
        EditorAction::Cut => {
            if let Some(removed_text) = app.text.remove_selection() {
                app.window.set_clipboard_string(removed_text.as_str());
            } else {
                let removed_text = app.text.remove_current_line();
                app.window.set_clipboard_string(removed_text.as_str());
            }
        }
        EditorAction::Paste => {
            if let Some(mut s) = app.window.get_clipboard_string() {
                unsafe {
                    let m = s.as_bytes_mut();
                    let mut matches: Vec<usize> = memchr_iter('\r' as u8, m).collect();
                    matches.push(m.len());
                    remove_crlf_from_buff(m, &matches);
                    s.truncate(s.len() - (matches.len() - 1));
                }
                app.text.insert_text(s.as_str())
            }
        }
        EditorAction::DeleteForward => {
            app.text.delete_text(DeleteDirection::Forward);
        }
        EditorAction::DeleteBackward => {
            app.text.delete_text(DeleteDirection::Back);
        }

        EditorAction::Undo => {
            app.text.undo();
        }
        EditorAction::Redo => {
            app.text.redo();
        }
        EditorAction::Save => {
            fs::write(&app.file_path, &app.text.get_text().to_string()).unwrap();
        }
    }
}
