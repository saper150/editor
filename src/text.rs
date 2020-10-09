use crate::cursor;

use cursor::{Cursor, Point};

#[derive(Clone, Debug)]
pub struct UndoPoint {
    pub text: ropey::Rope,
    pub cursor: Cursor,
}

#[derive(Debug)]
pub struct UndoState {
    pub history: Vec<UndoPoint>,
    pub index: usize,
}

pub struct Text {
    pub text: ropey::Rope,
    pub history: Vec<UndoPoint>,
    pub index: usize,
}

impl Text {
    pub fn new<T: std::io::Read>(reader: T) -> Text {
        let initial_text = ropey::Rope::from_reader(reader).unwrap();
        return Text {
            text: initial_text.clone(),
            history: vec![UndoPoint {
                text: initial_text,
                cursor: Cursor::new(),
            }],
            index: 0,
        };
    }
}

fn selection_range(text: &Text, cursor: &Cursor) -> std::ops::Range<usize> {
    let a = cursor.position.to_char(&text.text);
    let b = cursor.selection.unwrap().to_char(&text.text);
    a.min(b)..b.max(a)
}

pub fn undo(text: &mut Text, cursor: &mut Cursor) {
    if text.index == 0 {
        return;
    }
    let undo_point = &text.history[text.index - 1];
    text.text = undo_point.text.clone();
    cursor.position = undo_point.cursor.position;
    cursor.selection = undo_point.cursor.selection;
    cursor.remembered_x = undo_point.cursor.remembered_x;
    text.index -= 1;
}

pub fn redo(text: &mut Text, cursor: &mut Cursor) {
    if text.index + 1 >= text.history.len() {
        return;
    }

    let undo_point = &text.history[text.index + 1];
    text.text = undo_point.text.clone();
    cursor.position = undo_point.cursor.position;
    cursor.selection = undo_point.cursor.selection;

    cursor.remembered_x = undo_point.cursor.remembered_x;
    text.index += 1;
}

fn initialize_undo(text: &mut Text, cursor: &mut Cursor) {
    if text.index == 0 {
        text.history[0].cursor = cursor.clone();
    }
}

fn add_undo_point(text: &mut Text, cursor: &mut Cursor) {
    if text.index < text.history.len() {
        text.history.truncate(text.index + 1)
    }

    text.history.push(UndoPoint {
        text: text.text.clone(),
        cursor: cursor.clone(),
    });
    text.index += 1;
}

pub fn insert_text(text: &mut Text, cursor: &mut Cursor, str: &str) {
    initialize_undo(text, cursor);
    let start_idx = cursor.position.to_char(&text.text);

    if cursor.selection.is_some() {
        let range = selection_range(text, cursor);
        text.text.remove(range.clone());
        text.text.insert(start_idx, str);
        cursor.position = Point::from_char(range.start + str.chars().count(), &text.text);
    } else {
        text.text.insert(start_idx, str);

        let end_idx = start_idx + str.chars().count();
        cursor.position = Point::from_char(end_idx, &text.text);
    }

    cursor.remembered_x = cursor.position.x;
    add_undo_point(text, cursor);
}

pub enum DeleteKey {
    Del,
    Backspace,
}

pub fn delete_text(text: &mut Text, cursor: &mut Cursor, key: DeleteKey) {
    fn delete_next_char(text: &mut Text, cursor: &mut Cursor) {
        let start_idx = cursor.position.to_char(&text.text);
        text.text
            .remove(start_idx..(start_idx + 1).min(text.text.len_chars()));
    }

    fn delete_previous_char(text: &mut Text, cursor: &mut Cursor) {
        let start_idx = cursor.position.to_char(&text.text);
        text.text.remove((start_idx.max(1) - 1)..start_idx);

        cursor.position = Point::from_char(start_idx.max(1) - 1, &text.text);
    }

    initialize_undo(text, cursor);

    if cursor.selection.is_some() {
        let range = selection_range(text, cursor);
        text.text.remove(range.clone());
        cursor.position = Point::from_char(range.start, &text.text);
        cursor.selection = None;
    } else {
        match key {
            DeleteKey::Del => {
                delete_next_char(text, cursor);
            }
            DeleteKey::Backspace => {
                delete_previous_char(text, cursor);
            }
        }
    }

    add_undo_point(text, cursor);
}

#[cfg(test)]
mod tests {

    use crate::cursor;
    use crate::text::{delete_text, insert_text, redo, undo, DeleteKey, Text};
    use cursor::{Cursor, Point};

    fn create_text(initial_text: &str) -> (Text, Cursor) {
        let reader = std::io::BufReader::new(initial_text.as_bytes());
        let text = Text::new(reader);
        let cursor = Cursor::new();
        return (text, cursor);
    }

    #[test]
    fn should_insert_text_at_cursor() {
        let (mut text, mut cursor) = create_text("initial text");

        insert_text(&mut text, &mut cursor, "str ");
        assert_eq!(text.text.to_string(), "str initial text");
    }

    #[test]
    fn should_insert_text_at_cursor_2() {
        let (mut text, mut cursor) = create_text("initial text");

        cursor.position = cursor::Point { x: 1, y: 0 };

        insert_text(&mut text, &mut cursor, "str ");
        assert_eq!(text.text.to_string(), "istr nitial text");
    }

    #[test]
    fn should_insert_text_at_cursor_3() {
        let (mut text, mut cursor) = create_text("initial text\nabc");

        cursor.position = cursor::Point { x: 0, y: 1 };

        insert_text(&mut text, &mut cursor, " str ");
        assert_eq!(text.text.to_string(), "initial text\n str abc");
    }

    #[test]
    fn should_move_cursor_position_after_insert() {
        let (mut text, mut cursor) = create_text("initial text");

        insert_text(&mut text, &mut cursor, "str");
        assert_eq!(cursor.position.x, 3);
    }

    #[test]
    fn should_move_cursor_position_after_insert_2() {
        let (mut text, mut cursor) = create_text("initial text");

        insert_text(&mut text, &mut cursor, "str\n1");
        assert_eq!(cursor.position.x, 1);
        assert_eq!(cursor.position.y, 1);
        assert_eq!(cursor.remembered_x, 1);
    }

    #[test]
    fn should_replace_selection() {
        let (mut text, mut cursor) = create_text("initial text");
        cursor.selection = Some(Point { x: 2, y: 0 });

        insert_text(&mut text, &mut cursor, "str");

        assert_eq!(text.text.to_string(), "stritial text");

        assert_eq!(cursor.position.x, 3);
        assert_eq!(cursor.position.y, 0);
    }

    #[test]
    fn delete_should_delete_next_char() {
        let (mut text, mut cursor) = create_text("initial text");

        delete_text(&mut text, &mut cursor, DeleteKey::Del);

        assert_eq!(text.text.to_string(), "nitial text");
        assert_eq!(cursor.position.x, 0);
        assert_eq!(cursor.position.y, 0);
    }

    #[test]
    fn delete_empty_string() {
        let (mut text, mut cursor) = create_text("");

        delete_text(&mut text, &mut cursor, DeleteKey::Del);

        assert_eq!(text.text.to_string(), "");
        assert_eq!(cursor.position.x, 0);
        assert_eq!(cursor.position.y, 0);
    }

    #[test]
    fn delete_cursor_on_last_position() {
        let (mut text, mut cursor) = create_text("text");
        cursor.position.x = 4;
        delete_text(&mut text, &mut cursor, DeleteKey::Del);

        assert_eq!(text.text.to_string(), "text");
        assert_eq!(cursor.position.x, 4);
    }

    #[test]
    fn delete_text_backspace() {
        let (mut text, mut cursor) = create_text("initial text");

        cursor.position.x = 2;

        delete_text(&mut text, &mut cursor, DeleteKey::Backspace);

        assert_eq!(text.text.to_string(), "iitial text");
        assert_eq!(cursor.position.x, 1);
        assert_eq!(cursor.position.y, 0);
    }

    #[test]
    fn delete_text_backspace_empty_string() {
        let (mut text, mut cursor) = create_text("");

        delete_text(&mut text, &mut cursor, DeleteKey::Backspace);

        assert_eq!(text.text.to_string(), "");
        assert_eq!(cursor.position.x, 0);
        assert_eq!(cursor.position.y, 0);
    }

    #[test]
    fn delete_text_backspace_cursor_at_start() {
        let (mut text, mut cursor) = create_text("text");

        delete_text(&mut text, &mut cursor, DeleteKey::Backspace);

        assert_eq!(text.text.to_string(), "text");
        assert_eq!(cursor.position.x, 0);
    }

    #[test]
    fn delete_selection() {
        let (mut text, mut cursor) = create_text("text");
        cursor.position.x = 3;
        cursor.selection = Some(Point { x: 1, y: 0 });

        delete_text(&mut text, &mut cursor, DeleteKey::Backspace);
        assert_eq!(text.text.to_string(), "tt");
        assert_eq!(cursor.position.x, 1);
    }

    #[test]
    fn multiple_undo() {
        let (mut text, mut cursor) = create_text("");
        insert_text(&mut text, &mut cursor, "1");
        insert_text(&mut text, &mut cursor, "1");
        insert_text(&mut text, &mut cursor, "1");

        undo(&mut text, &mut cursor);
        assert_eq!(text.text.to_string(), "11");
        assert_eq!(cursor.position.x, 2);

        undo(&mut text, &mut cursor);
        assert_eq!(text.text.to_string(), "1");
        assert_eq!(cursor.position.x, 1);

        undo(&mut text, &mut cursor);
        assert_eq!(text.text.to_string(), "");
        assert_eq!(cursor.position.x, 0);
    }

    #[test]
    fn redo_test() {
        let (mut text, mut cursor) = create_text("");
        insert_text(&mut text, &mut cursor, "str");
        undo(&mut text, &mut cursor);
        redo(&mut text, &mut cursor);

        insert_text(&mut text, &mut cursor, "str");
        insert_text(&mut text, &mut cursor, "str");
        undo(&mut text, &mut cursor);
        undo(&mut text, &mut cursor);
        redo(&mut text, &mut cursor);
        redo(&mut text, &mut cursor);

        assert_eq!(text.text.to_string(), "strstrstr");
        assert_eq!(cursor.position.x, 9);
        assert_eq!(cursor.position.y, 0);
        assert_eq!(cursor.remembered_x, 9);
    }

    #[test]
    fn should_reset_history_when_modify_undo() {
        let (mut text, mut cursor) = create_text("");
        insert_text(&mut text, &mut cursor, "1");
        insert_text(&mut text, &mut cursor, "2");
        undo(&mut text, &mut cursor);

        insert_text(&mut text, &mut cursor, "3");
        undo(&mut text, &mut cursor);
        redo(&mut text, &mut cursor);

        assert_eq!(text.text.to_string(), "13");
        assert_eq!(cursor.position.x, 2);
        assert_eq!(cursor.position.y, 0);
        assert_eq!(cursor.remembered_x, 2);
    }

    #[test]
    fn undo_past_history() {
        let (mut text, mut cursor) = create_text("");

        undo(&mut text, &mut cursor);

        insert_text(&mut text, &mut cursor, "2");
        undo(&mut text, &mut cursor);
        undo(&mut text, &mut cursor);

        assert_eq!(text.text.to_string(), "");
        assert_eq!(cursor.position.x, 0);
        assert_eq!(cursor.position.y, 0);
        assert_eq!(cursor.remembered_x, 0);
    }

    #[test]
    fn redo_past_history() {
        let (mut text, mut cursor) = create_text("");
        redo(&mut text, &mut cursor);
        insert_text(&mut text, &mut cursor, "2");
        undo(&mut text, &mut cursor);
        redo(&mut text, &mut cursor);
        redo(&mut text, &mut cursor);

        assert_eq!(text.text.to_string(), "2");
        assert_eq!(cursor.position.x, 1);
        assert_eq!(cursor.position.y, 0);
        assert_eq!(cursor.remembered_x, 1);
    }

    #[test]
    fn undo_redo_delete_text() {
        let (mut text, mut cursor) = create_text("123");

        cursor.selection = Some(Point { x: 2, y: 0 });
        delete_text(&mut text, &mut cursor, DeleteKey::Del);
        undo(&mut text, &mut cursor);

        assert_eq!(text.text.to_string(), "123");

        redo(&mut text, &mut cursor);

        assert_eq!(text.text.to_string(), "3");
    }
    #[test]
    fn remove_selection_after_delete() {
        let (mut text, mut cursor) = create_text("123");

        cursor.selection = Some(Point { x: 2, y: 0 });
        delete_text(&mut text, &mut cursor, DeleteKey::Del);

        assert_eq!(cursor.selection.is_none(), true);
    }

    #[test]
    fn retain_cursor_position_of_first_edit_2() {
        let (mut text, mut cursor) = create_text("123");
        cursor.position.x = 1;
        delete_text(&mut text, &mut cursor, DeleteKey::Backspace);
        undo(&mut text, &mut cursor);
        assert_eq!(cursor.position.x, 1);

        cursor.position.x = 2;

        delete_text(&mut text, &mut cursor, DeleteKey::Backspace);
        undo(&mut text, &mut cursor);

        assert_eq!(cursor.position.x, 2);
    }

    // #[test]
    // fn add_undo_point_on_whitespace() {
    // 	let (mut text, mut cursor) = create_text("123");
    // }
}
