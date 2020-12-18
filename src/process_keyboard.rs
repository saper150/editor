use crate::app;
use crate::editor_action::{dispatch_action, EditorAction};

use app::App;

use glfw::{Key, Modifiers};
use std::collections::HashMap;

pub fn process_char(app: &mut App, char: &char) {
    let mut tmp = [0; 4];
    app.text.insert_text(char.encode_utf8(&mut tmp));
    app.should_rerender = true;
}

#[derive(PartialEq, Eq, Hash)]
pub struct KeyAction {
    pub key: Key,
    pub modifiers: Modifiers,
}

fn default_shortcuts() -> Vec<(KeyAction, EditorAction)> {
    return vec![
        (
            KeyAction {
                key: Key::Z,
                modifiers: Modifiers::Control,
            },
            EditorAction::Undo,
        ),
        (
            KeyAction {
                key: Key::Y,
                modifiers: Modifiers::Control,
            },
            EditorAction::Redo,
        ),
        (
            KeyAction {
                key: Key::S,
                modifiers: Modifiers::Control,
            },
            EditorAction::Save,
        ),
        (
            KeyAction {
                key: Key::Backspace,
                modifiers: Modifiers::empty(),
            },
            EditorAction::DeleteBackward,
        ),
        (
            KeyAction {
                key: Key::Delete,
                modifiers: Modifiers::empty(),
            },
            EditorAction::DeleteForward,
        ),
        (
            KeyAction {
                key: Key::Left,
                modifiers: Modifiers::empty(),
            },
            EditorAction::CursorLeft,
        ),
        (
            KeyAction {
                key: Key::Left,
                modifiers: Modifiers::Shift,
            },
            EditorAction::CursorLeftSelect,
        ),
        (
            KeyAction {
                key: Key::Left,
                modifiers: Modifiers::Control,
            },
            EditorAction::CursorPrevWord,
        ),
        (
            KeyAction {
                key: Key::Left,
                modifiers: Modifiers::Control | Modifiers::Shift,
            },
            EditorAction::CursorPrevWordSelect,
        ),
        (
            KeyAction {
                key: Key::Right,
                modifiers: Modifiers::empty(),
            },
            EditorAction::CursorRight,
        ),
        (
            KeyAction {
                key: Key::Right,
                modifiers: Modifiers::Shift,
            },
            EditorAction::CursorRightSelect,
        ),
        (
            KeyAction {
                key: Key::Right,
                modifiers: Modifiers::Control,
            },
            EditorAction::CursorNextWord,
        ),
        (
            KeyAction {
                key: Key::Right,
                modifiers: Modifiers::Control | Modifiers::Shift,
            },
            EditorAction::CursorNextWordSelect,
        ),
        (
            KeyAction {
                key: Key::Up,
                modifiers: Modifiers::empty(),
            },
            EditorAction::CursorUp,
        ),
        (
            KeyAction {
                key: Key::Up,
                modifiers: Modifiers::Shift,
            },
            EditorAction::CursorUpSelect,
        ),
        (
            KeyAction {
                key: Key::Down,
                modifiers: Modifiers::empty(),
            },
            EditorAction::CursorDown,
        ),
        (
            KeyAction {
                key: Key::Down,
                modifiers: Modifiers::Shift,
            },
            EditorAction::CursorDownSelect,
        ),
        (
            KeyAction {
                key: Key::End,
                modifiers: Modifiers::empty(),
            },
            EditorAction::CursorEnd,
        ),
        (
            KeyAction {
                key: Key::End,
                modifiers: Modifiers::Shift,
            },
            EditorAction::CursorEndSelect,
        ),
        (
            KeyAction {
                key: Key::End,
                modifiers: Modifiers::Control,
            },
            EditorAction::CursorEndOfFile,
        ),
        (
            KeyAction {
                key: Key::End,
                modifiers: Modifiers::Control | Modifiers::Shift,
            },
            EditorAction::CursorEndOfFileSelect,
        ),
        (
            KeyAction {
                key: Key::Home,
                modifiers: Modifiers::empty(),
            },
            EditorAction::CursorHome,
        ),
        (
            KeyAction {
                key: Key::Home,
                modifiers: Modifiers::Shift,
            },
            EditorAction::CursorHomeSelect,
        ),
        (
            KeyAction {
                key: Key::Home,
                modifiers: Modifiers::Control,
            },
            EditorAction::CursorBeginningOfFile,
        ),
        (
            KeyAction {
                key: Key::Home,
                modifiers: Modifiers::Control | Modifiers::Shift,
            },
            EditorAction::CursorBeginningOfFileSelect,
        ),
        (
            KeyAction {
                key: Key::PageUp,
                modifiers: Modifiers::empty(),
            },
            EditorAction::CursorPageUp,
        ),
        (
            KeyAction {
                key: Key::PageUp,
                modifiers: Modifiers::Shift,
            },
            EditorAction::CursorPageUpSelect,
        ),
        (
            KeyAction {
                key: Key::PageDown,
                modifiers: Modifiers::empty(),
            },
            EditorAction::CursorPageDown,
        ),
        (
            KeyAction {
                key: Key::PageDown,
                modifiers: Modifiers::Shift,
            },
            EditorAction::CursorPageDownSelect,
        ),
        (
            KeyAction {
                key: Key::PageDown,
                modifiers: Modifiers::empty(),
            },
            EditorAction::CursorPageDown,
        ),
        (
            KeyAction {
                key: Key::PageDown,
                modifiers: Modifiers::Shift,
            },
            EditorAction::CursorPageDownSelect,
        ),
        (
            KeyAction {
                key: Key::V,
                modifiers: Modifiers::Control,
            },
            EditorAction::Paste,
        ),
        (
            KeyAction {
                key: Key::C,
                modifiers: Modifiers::Control,
            },
            EditorAction::Copy,
        ),
        (
            KeyAction {
                key: Key::X,
                modifiers: Modifiers::Control,
            },
            EditorAction::Cut,
        ),
    ];
}

pub struct KeyBoardShortcuts {
    shortcuts: HashMap<KeyAction, EditorAction>,
}

impl KeyBoardShortcuts {
    pub fn new() -> KeyBoardShortcuts {
        let mut e = HashMap::new();
        e.extend(default_shortcuts().into_iter());

        return KeyBoardShortcuts { shortcuts: e };
    }

    pub fn get_action(&self, key_action: &KeyAction) -> Option<EditorAction> {
        let e = self.shortcuts.get(key_action);
        return e.map(|x| *x);
    }
}

pub fn process_keyboard(app: &mut App, key: KeyAction) {
    println!("{:?}", key.key);
    if let Some(action) = app.shortcuts.get_action(&key) {
        dispatch_action(app, action);
        return;
    }

    match key.key {
        Key::Enter => {
            app.text.insert_text("\n");
        }
        Key::Tab => {
            app.text.insert_text("\t");
        }
        _ => {}
    }
}
