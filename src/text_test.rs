use crate::cursor;
use crate::text::{DeleteKey, Selection, Text};
use cursor::Point;

fn create_text(initial_text: &str) -> Text {
    let reader = std::io::BufReader::new(initial_text.as_bytes());
    let text = Text::new(reader);
    return text;
}

#[test]
fn should_insert_text_at_cursor() {
    let mut text = create_text("initial text");

    text.insert_text("str ");

    assert_eq!(text.get_string(), "str initial text");
}

#[test]
fn should_insert_text_at_cursor_2() {
    let mut text = create_text("initial text");
    text.get_cursor().position = cursor::Point { x: 1, y: 0 };

    text.insert_text("str ");
    assert_eq!(text.get_string(), "istr nitial text");
}

#[test]
fn should_insert_text_at_cursor_3() {
    let mut text = create_text("initial text\nabc");
    text.get_cursor().position = cursor::Point { x: 0, y: 1 };

    text.insert_text(" str ");
    assert_eq!(text.get_string(), "initial text\n str abc");
}

#[test]
fn should_move_cursor_position_after_insert() {
    let mut text = create_text("initial text");

    text.insert_text("str");
    assert_eq!(text.get_cursor().position.x, 3);
}

#[test]
fn should_move_cursor_position_after_insert_2() {
    let mut text = create_text("initial text");

    text.insert_text("str\n1");
    assert_eq!(text.get_cursor().position.x, 1);
    assert_eq!(text.get_cursor().position.y, 1);
    assert_eq!(text.get_cursor().remembered_x, 1);
}

#[test]
fn should_replace_selection() {
    let mut text = create_text("initial text");
    text.get_cursor().selection = Some(Point { x: 2, y: 0 });

    text.insert_text("str");

    assert_eq!(text.get_string(), "stritial text");

    assert_eq!(text.get_cursor().position.x, 3);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn delete_should_delete_next_char() {
    let mut text = create_text("initial text");

    text.delete_text(DeleteKey::Del);

    assert_eq!(text.get_string(), "nitial text");
    assert_eq!(text.get_cursor().position.x, 0);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn delete_empty_string() {
    let mut text = create_text("");

    text.delete_text(DeleteKey::Del);

    assert_eq!(text.get_string(), "");
    assert_eq!(text.get_cursor().position.x, 0);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn delete_cursor_on_last_position() {
    let mut text = create_text("text");
    text.get_cursor().position.x = 4;
    text.delete_text(DeleteKey::Del);

    assert_eq!(text.get_string(), "text");
    assert_eq!(text.get_cursor().position.x, 4);
}

#[test]
fn delete_text_backspace() {
    let mut text = create_text("initial text");

    text.get_cursor().position.x = 2;

    text.delete_text(DeleteKey::Backspace);

    assert_eq!(text.get_string(), "iitial text");
    assert_eq!(text.get_cursor().position.x, 1);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn delete_text_backspace_empty_string() {
    let mut text = create_text("");

    text.delete_text(DeleteKey::Backspace);

    assert_eq!(text.get_string(), "");
    assert_eq!(text.get_cursor().position.x, 0);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn delete_text_backspace_cursor_at_start() {
    let mut text = create_text("text");

    text.delete_text(DeleteKey::Backspace);

    assert_eq!(text.get_string(), "text");
    assert_eq!(text.get_cursor().position.x, 0);
}

#[test]
fn delete_selection() {
    let mut text = create_text("text");
    text.get_cursor().position.x = 3;
    text.get_cursor().selection = Some(Point { x: 1, y: 0 });

    text.delete_text(DeleteKey::Backspace);
    assert_eq!(text.get_string(), "tt");
    assert_eq!(text.get_cursor().position.x, 1);
}

#[test]
fn multiple_undo() {
    let mut text = create_text("");
    text.insert_text("11");
    text.insert_text("11");
    text.insert_text("11");

    println!("{:?}", text.history);

    text.undo();
    assert_eq!(text.get_string(), "1111");
    assert_eq!(text.get_cursor().position.x, 4);

    text.undo();
    assert_eq!(text.get_string(), "11");
    assert_eq!(text.get_cursor().position.x, 2);

    text.undo();
    assert_eq!(text.get_string(), "");
    assert_eq!(text.get_cursor().position.x, 0);
}

#[test]
fn redo_test() {
    let mut text = create_text("");
    text.insert_text("str");
    text.undo();
    text.redo();

    text.insert_text("str");
    text.insert_text("str");
    text.undo();
    text.undo();
    text.redo();
    text.redo();

    assert_eq!(text.get_string(), "strstrstr");
    assert_eq!(text.get_cursor().position.x, 9);
    assert_eq!(text.get_cursor().position.y, 0);
    assert_eq!(text.get_cursor().remembered_x, 9);
}

#[test]
fn should_reset_history_when_modify_undo() {
    let mut text = create_text("");
    text.insert_text("11");
    text.insert_text("22");
    text.undo();

    text.insert_text("33");
    text.undo();
    text.redo();

    assert_eq!(text.get_string(), "1133");
    assert_eq!(text.get_cursor().position.x, 4);
    assert_eq!(text.get_cursor().position.y, 0);
    assert_eq!(text.get_cursor().remembered_x, 4);
}

#[test]
fn undo_past_history() {
    let mut text = create_text("");

    text.undo();

    text.insert_text("22");
    text.undo();
    text.undo();

    assert_eq!(text.get_string(), "");
    assert_eq!(text.get_cursor().position.x, 0);
    assert_eq!(text.get_cursor().position.y, 0);
    assert_eq!(text.get_cursor().remembered_x, 0);
}

#[test]
fn redo_past_history() {
    let mut text = create_text("");
    text.redo();
    text.insert_text("2");
    text.undo();
    text.redo();
    text.redo();

    assert_eq!(text.get_string(), "2");
    assert_eq!(text.get_cursor().position.x, 1);
    assert_eq!(text.get_cursor().position.y, 0);
    assert_eq!(text.get_cursor().remembered_x, 1);
}

#[test]
fn undo_redo_delete_text() {
    let mut text = create_text("123");

    text.get_cursor().selection = Some(Point { x: 2, y: 0 });
    text.delete_text(DeleteKey::Del);
    text.undo();

    assert_eq!(text.get_string(), "123");

    text.redo();

    assert_eq!(text.get_string(), "3");
}

#[test]
fn remove_selection_after_delete() {
    let mut text = create_text("123");

    text.get_cursor().selection = Some(Point { x: 2, y: 0 });
    text.delete_text(DeleteKey::Del);

    assert_eq!(text.get_cursor().selection.is_none(), true);
}

#[test]
fn retain_cursor_position_of_first_edit_2() {
    let mut text = create_text("123");
    text.get_cursor().position.x = 1;
    text.delete_text(DeleteKey::Backspace);
    text.undo();
    assert_eq!(text.get_cursor().position.x, 1);

    text.get_cursor().position.x = 2;

    text.delete_text(DeleteKey::Backspace);
    text.undo();

    assert_eq!(text.get_cursor().position.x, 2);
}

#[test]
fn not_add_undopoint_on_single_char() {
    let mut text = create_text("");
    text.undo();
    text.undo();

    text.insert_text("1");
    text.undo();
    text.redo();
    text.insert_text("2");
    text.insert_text("3");
    text.undo();

    assert_eq!(text.get_string(), "1");
}

#[test]
fn add_undo_on_whitespace() {
    let mut text = create_text("");
    text.undo();
    text.insert_text(" ");
    text.insert_text("\t");
    text.insert_text("\n");

    text.undo();
    assert_eq!(text.get_string(), " \t");

    text.undo();
    assert_eq!(text.get_string(), " ");

    text.undo();
    assert_eq!(text.get_string(), "");
}

#[test]
fn move_cursor_right() {
    let mut text = create_text("xyz");
    text.move_cursor(1, Selection::NotSelect);
    assert_eq!(text.get_cursor().position.x, 1);
}

#[test]
fn move_cursor_right_empty_string() {
    let mut text = create_text("");
    text.move_cursor(1, Selection::NotSelect);
    assert_eq!(text.get_cursor().position.x, 0);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn move_cursor_right_line_break() {
    let mut text = create_text("a\nb");
    text.move_cursor(2, Selection::NotSelect);
    assert_eq!(text.get_cursor().position.x, 0);
    assert_eq!(text.get_cursor().position.y, 1);
}

#[test]
fn move_cursor_left() {
    let mut text = create_text("a\nb");
    text.move_cursor(2, Selection::NotSelect);
    text.move_cursor(-1, Selection::NotSelect);
    assert_eq!(text.get_cursor().position.x, 1);
}

#[test]
fn move_cursor_below_0() {
    let mut text = create_text("a\nb");
    text.move_cursor(-2, Selection::NotSelect);
    assert_eq!(text.get_cursor().position.x, 0);
}

#[test]
fn move_cursor_down() {
    let mut text = create_text("a\nb\nc");
    text.move_cursor_y(2, Selection::NotSelect);
    assert_eq!(text.get_cursor().position.y, 2);
    assert_eq!(text.get_cursor().position.x, 0);
}

#[test]
fn move_cursor_down_past_end() {
    let mut text = create_text("abc");
    text.move_cursor(3, Selection::NotSelect);
    text.move_cursor_y(2, Selection::NotSelect);
    text.move_cursor_y(1, Selection::NotSelect);
    assert_eq!(text.get_cursor().position.y, 0);
    assert_eq!(text.get_cursor().position.x, 3);
}

#[test]
fn move_cursor_up() {
    let mut text = create_text("a\nb\nc");
    text.move_cursor_y(2, Selection::NotSelect);
    text.move_cursor_y(-1, Selection::NotSelect);
    assert_eq!(text.get_cursor().position.y, 1);
}

#[test]
fn move_cursor_up_past_end() {
    let mut text = create_text("a\nb\nc");
    text.move_cursor_y(-1, Selection::NotSelect);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn move_down_remember_x() {
    let mut text = create_text("a\nb\nc");
    text.move_cursor(1, Selection::NotSelect);
    text.move_cursor_y(1, Selection::NotSelect);
    assert_eq!(text.get_cursor().position.y, 1);
    assert_eq!(text.get_cursor().position.x, 1);
}

#[test]
fn move_down_remember_pase_end() {
    let mut text = create_text("aa\nb\nccccc");
    text.move_cursor(2, Selection::NotSelect);
    text.move_cursor_y(1, Selection::NotSelect);
    assert_eq!(text.get_cursor().position.y, 1);
    assert_eq!(text.get_cursor().position.x, 1);

    text.move_cursor_y(1, Selection::NotSelect);

    assert_eq!(text.get_cursor().position.y, 2);
    assert_eq!(text.get_cursor().position.x, 2);
}

#[test]
fn move_next_word() {
    let mut text = create_text("abc abc abc");
    text.move_to_next_word(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 3);
    assert_eq!(text.get_cursor().position.y, 0);

    text.move_cursor(-1, Selection::NotSelect);
    text.move_to_next_word(Selection::NotSelect);
    assert_eq!(text.get_cursor().position.x, 3);
}

#[test]
fn move_next_word_end_of_line() {
    let mut text = create_text("abcd\n");
    text.move_to_next_word(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 4);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn move_next_word_skipp_initial_whitespace() {
    let mut text = create_text("  abcd abc");
    text.move_to_next_word(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 6);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn move_next_word_next_line() {
    let mut text = create_text("abc\nabc");
    text.move_to_next_word(Selection::NotSelect);
    text.move_to_next_word(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 3);
    assert_eq!(text.get_cursor().position.y, 1);
}

#[test]
fn move_next_word_empty_string() {
    let mut text = create_text("");
    text.move_to_next_word(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 0);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn move_prev_word() {
    let mut text = create_text("abc abc abc");
    text.move_cursor(4, Selection::NotSelect);
    text.move_to_prev_word(Selection::NotSelect);
    assert_eq!(text.get_cursor().position.x, 0);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn move_prev_word_up_line() {
    let mut text = create_text("abc\nabc\n");
    text.move_cursor_y(1, Selection::NotSelect);
    text.move_cursor(1, Selection::NotSelect);
    text.move_to_prev_word(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 0);
    assert_eq!(text.get_cursor().position.y, 1);

    text.move_to_prev_word(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 0);
    assert_eq!(text.get_cursor().position.y, 0);
}

#[test]
fn move_next_word_stop_on_line_brake() {
    let mut text = create_text("abc   \nabc\n");
    text.move_to_next_word(Selection::NotSelect);
    text.move_to_next_word(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 6);
}

#[test]
fn move_to_end_of_line() {
    let mut text = create_text("abc   \nabc\n");
    text.move_to_end_of_line(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 6);
}

#[test]
fn move_to_end_of_line_empty_string() {
    let mut text = create_text("");
    text.move_to_end_of_line(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 0);
}

#[test]
fn move_to_end_of_line_remember_x() {
    let mut text = create_text("abc\nabc");
    text.move_to_end_of_line(Selection::NotSelect);
    text.move_cursor_y(1, Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 3);
    assert_eq!(text.get_cursor().position.y, 1);
}

#[test]
fn move_to_end_of_line_without_line_brake() {
    let mut text = create_text("abc");
    text.move_to_end_of_line(Selection::NotSelect);
    text.move_to_end_of_line(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 3);
}

#[test]
fn move_to_beginning_of_line() {
    let mut text = create_text("abc   \nabc\n");
    text.move_cursor(7, Selection::NotSelect);
    text.move_to_beginning_of_line(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 0);
}

#[test]
fn move_to_beginning_of_line_empty_string() {
    let mut text = create_text("");
    text.move_to_beginning_of_line(Selection::NotSelect);

    assert_eq!(text.get_cursor().position.x, 0);
}

#[test]
fn move_to_beginning_remember_x() {
    let mut text = create_text("abcabc\ncba");
    text.move_cursor(2, Selection::NotSelect);
    text.move_to_beginning_of_line(Selection::NotSelect);
    text.move_cursor_y(1, Selection::NotSelect);

    assert_eq!(text.get_cursor().position.y, 1);
    assert_eq!(text.get_cursor().position.x, 0);
}

#[test]
fn process_selection() {
    let mut text = create_text("abcabc\ncba");
    text.move_cursor(2, Selection::Select);

    assert_eq!(text.get_cursor().selection.unwrap().x, 0);

    text.move_cursor(-2, Selection::NotSelect);
    assert_eq!(text.get_cursor().selection.is_none(), true);

    text.move_cursor(2, Selection::NotSelect);
    text.move_cursor(-2, Selection::Select);

    assert_eq!(text.get_cursor().selection.unwrap().x, 2);
}

#[test]
fn process_selection_y() {
    let mut text = create_text("abcabc\ncba");

    text.move_cursor(2, Selection::NotSelect);
    text.move_cursor_y(1, Selection::Select);

    assert_eq!(text.get_cursor().selection.unwrap().x, 2);
    assert_eq!(text.get_cursor().selection.unwrap().y, 0);
}

#[test]
fn process_selection_next_word() {
    let mut text = create_text("abc abc abc");
    text.move_to_next_word(Selection::Select);

    assert_eq!(text.get_cursor().selection.unwrap().x, 0);
    assert_eq!(text.get_cursor().selection.unwrap().y, 0);

    text.move_to_next_word(Selection::NotSelect);
    text.move_to_prev_word(Selection::Select);

    assert_eq!(text.get_cursor().selection.unwrap().x, 7);
    assert_eq!(text.get_cursor().selection.unwrap().y, 0);
}

#[test]
fn process_selection_end_of_line() {
    let mut text = create_text("abc abc");
    text.move_to_end_of_line(Selection::Select);

    assert_eq!(text.get_cursor().selection.unwrap().x, 0);
    assert_eq!(text.get_cursor().selection.unwrap().y, 0);
}

#[test]
fn process_selection_beginning_of_line() {
    let mut text = create_text("abc abc");
    text.move_to_end_of_line(Selection::NotSelect);
    text.move_to_beginning_of_line(Selection::Select);

    assert_eq!(text.get_cursor().selection.unwrap().x, 7);
    assert_eq!(text.get_cursor().selection.unwrap().y, 0);
}
