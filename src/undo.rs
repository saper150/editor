use crate::app;
use crate::cursor;

use app::{App};
use cursor::{Cursor, Point};

#[derive(Clone, Debug)]
pub struct UndoPoint {
    pub text: ropey::Rope,
    pub cursor: Cursor,
}

pub struct UndoState {
    pub history: Vec<UndoPoint>,
    pub index: usize,
}

impl UndoState {
    pub fn new(initial: ropey::Rope) -> UndoState {
        UndoState {
            history: vec![UndoPoint {
                text: initial,
                cursor: Cursor::new(),
            }],
            index: 1,
        }
    }
}

fn create_undo_from_current(app: &App) -> UndoPoint {
    UndoPoint {
        text: app.text.clone(),
        cursor: app.cursor.clone(),
    }
}

fn apply_undo(app: &mut App, point: UndoPoint) {
    app.text = point.text;
    app.cursor = point.cursor;
}

fn push(app: &mut App) {
    if app.undo.history.len() > 1
        && app.undo.history.last().unwrap().cursor.position.y == app.cursor.position.y
        && (app.undo.history.last().unwrap().cursor.position.x - app.cursor.position.x).abs() == 1
    {
        let undo = create_undo_from_current(app);
        let last = app.undo.history.last_mut().unwrap();
        *last = undo;
    } else {
        app.undo.history.push(create_undo_from_current(app));
    }
    app.undo.index = app.undo.history.len();
    app.selection = None;
}

fn undo_init(app: &mut App) {
    app.undo.history.truncate(app.undo.index);
    if app.undo.history.len() == 1 {
        let point = app.undo.history.last_mut().unwrap();
        point.cursor = app.cursor;
    }
}

pub fn delete_text(app: &mut App, range: std::ops::Range<usize>) {
    undo_init(app);
    app.text.remove(range.clone());

    app.cursor.position = Point::from_char(range.start.clone(), &app.text);
    app.cursor.remembered_x = app.cursor.position.x;

    push(app);
}

pub fn insert_text(app: &mut App, text: &str) {
    undo_init(app);

    let start_idx =
        app.cursor.position.to_char(&app.text);

    app.text.insert(start_idx, text);
    let end_idx = start_idx + text.chars().count();

    app.cursor.position = Point::from_char(end_idx, &app.text);
    app.cursor.remembered_x = app.cursor.position.x;

    push(app);
}

// pub fn replace_text(app: &mut App, range: &Range, text: &str) {
//     let start_idx = app.text.line_to_char(range.start_line) + range.start_column;
//     let mut end_idx = app.text.line_to_char(range.end_line) + range.end_column;

//     app.text.remove(start_idx..end_idx);
//     app.text.insert(start_idx, text);

//     end_idx = start_idx + text.chars().count();

//     let end_line = app.text.char_to_line(end_idx);

//     let end_column = end_idx - app.text.line_to_char(end_line);
//     app.cursor.position.x = end_column as i64;
//     app.cursor.position.y = end_line as i64;
//     app.cursor.remembered_x = app.cursor.position.x;

// }

pub fn back(app: &mut App) {
    if app.undo.index <= 1 {
        return;
    }
    apply_undo(app, app.undo.history[app.undo.index - 2].clone());
    app.undo.index -= 1;
}

pub fn forward(app: &mut App) {
    if app.undo.index < app.undo.history.len() {
        apply_undo(app, app.undo.history[app.undo.index].clone());
        app.undo.index += 1;
    }
}
