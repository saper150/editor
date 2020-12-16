use glfw::{Action, Key, Modifiers};

use crate::glfw::WindowEvent;
use crate::text::Selection;
use crate::{app::App, program::Program, text::DeleteKey};

fn select_all(app: &mut App) {
    app.text.move_to_begging(Selection::NotSelect);
    app.text.move_to_end(Selection::Select);
}

fn reset_text(app: &mut App) {
    select_all(app);
    app.text.delete_text(DeleteKey::Backspace);
    app.text.insert_text("line 0\nline 1\nline 2\n");
    app.text.move_to_begging(Selection::NotSelect);
}

#[test]
fn should_launch() {
    let mut program = Program::new();

    {
        reset_text(&mut program.app);
        Program::process_event(
            &mut program.app,
            &WindowEvent::Key(Key::End, 0, Action::Press, Modifiers::Shift),
        );

        Program::process_event(
            &mut program.app,
            &WindowEvent::Key(Key::C, 0, Action::Press, Modifiers::Control),
        );

        assert_eq!(
            program.app.window.get_clipboard_string(),
            Some("line 0".to_owned()),
            "copy selection"
        );
    }

    {
        reset_text(&mut program.app);
        Program::process_event(
            &mut program.app,
            &WindowEvent::Key(Key::Down, 0, Action::Press, Modifiers::empty()),
        );

        Program::process_event(
            &mut program.app,
            &WindowEvent::Key(Key::C, 0, Action::Press, Modifiers::Control),
        );

        assert_eq!(
            program.app.window.get_clipboard_string(),
            Some("line 1\n".to_owned()),
            "copy line when no selection"
        );
    }

    {
        reset_text(&mut program.app);

        program.app.window.set_clipboard_string("string");
        Program::process_event(
            &mut program.app,
            &WindowEvent::Key(Key::V, 0, Action::Press, Modifiers::Control),
        );

        assert_eq!(
            program.app.text.get_text(),
            "stringline 0\nline 1\nline 2\n",
            "paste text"
        );
    }
}
