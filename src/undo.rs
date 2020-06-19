use crate::app;

use app::{App, CursorPosition};

#[derive(Clone, Debug)]
pub struct UndoPoint {
    pub text: ropey::Rope,
    pub cursor_position: CursorPosition,
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
                cursor_position: CursorPosition {
                    x: 0,
                    y: 0,
                    remembered_x: 0,
                },
            }],
            index: 1,
        }
    }
}

pub struct Range {
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
}

impl Range {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Range {
        Range {
            start_line: start.0,
            start_column: start.1,

            end_line: end.0,
            end_column: end.1,
        }
    }
}

fn create_undo_from_current(app: &App) -> UndoPoint {
    UndoPoint {
        text: app.text.clone(),
        cursor_position: app.cursor_position.clone(),
    }
}

fn apply_undo(app: &mut App, point: UndoPoint) {
    app.text = point.text;
    app.cursor_position = point.cursor_position;
}

fn push(app: &mut App) {
    if app.undo.history.len() > 1
        && app.undo.history.last().unwrap().cursor_position.y == app.cursor_position.y
        && (app.undo.history.last().unwrap().cursor_position.x - app.cursor_position.x).abs() == 1
    {
        let undo = create_undo_from_current(app);
        let last = app.undo.history.last_mut().unwrap();
        *last = undo;
    } else {
        app.undo.history.push(create_undo_from_current(app));
    }
    app.undo.index = app.undo.history.len();
}

fn undo_init(app: &mut App) {
    app.undo.history.truncate(app.undo.index);
    if app.undo.history.len() == 1 {
        let point = app.undo.history.last_mut().unwrap();
        point.cursor_position = app.cursor_position;
    }
}

pub fn delete_text(app: &mut App, range: &Range) {
    undo_init(app);
    let start_idx = app.text.line_to_char(range.start_line) + range.start_column;
    let end_idx = app.text.line_to_char(range.end_line) + range.end_column;

    app.text.remove(start_idx..end_idx);

    app.cursor_position.x = range.start_column as i64;
    app.cursor_position.y = range.start_line as i64;
    app.cursor_position.remembered_x = app.cursor_position.x;
    push(app);
}

pub fn insert_text(app: &mut App, text: &str) {
    undo_init(app);
    let start_idx =
        app.text.line_to_char(app.cursor_position.y as usize) + app.cursor_position.x as usize;

    app.text.insert(start_idx, text);

    let end_idx = start_idx + text.chars().count();

    let end_line = app.text.char_to_line(end_idx);

    let end_column = end_idx - app.text.line_to_char(end_line);
    app.cursor_position.x = end_column as i64;
    app.cursor_position.y = end_line as i64;
    app.cursor_position.remembered_x = app.cursor_position.x;
    push(app);
}

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
