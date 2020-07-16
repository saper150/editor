use crate::app;
use crate::cursor;

use app::App;
use cursor::{Cursor, Point};

#[derive(Clone, Debug)]
pub struct UndoPoint {
    pub text: ropey::Rope,
    pub cursor: Cursor,
    pub selection: Option<Point>,
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
                selection: None,
            }],
            index: 1,
        }
    }
}

fn create_undo_from_current(app: &App) -> UndoPoint {
    UndoPoint {
        text: app.text.clone(),
        cursor: app.cursor.clone(),
        selection: app.selection.clone(),
    }
}

fn apply_undo(app: &mut App, point: UndoPoint) {
    app.text = point.text;
    app.cursor = point.cursor;
    app.selection = point.selection;
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
        point.selection = app.selection;
    }
}

pub fn delete_text(app: &mut App, range: std::ops::Range<usize>) {
    undo_init(app);
    app.text.remove(range.clone());

    app.cursor.position = Point::from_char(range.start.clone(), &app.text);
    app.cursor.remembered_x = app.cursor.position.x;

    push(app);
}

fn selection_range(app: &App) -> std::ops::Range<usize> {
    let a = app.cursor.position.to_char(&app.text);
    let b = app.selection.unwrap().to_char(&app.text);
    a.min(b)..b.max(a)
}

pub fn insert_text(app: &mut App, text: &str) {
    undo_init(app);

    if app.selection.is_some() {
        let range = selection_range(app);
        app.text.remove(selection_range(app));
        app.cursor.position = Point::from_char(range.start.clone(), &app.text);
    }

    let start_idx = app.cursor.position.to_char(&app.text);

    app.text.insert(start_idx, text);
    let end_idx = start_idx + text.chars().count();

    app.cursor.position = Point::from_char(end_idx, &app.text);
    app.cursor.remembered_x = app.cursor.position.x;

    push(app);
}

pub fn back(app: &mut App) {
    if app.undo.index <= 1 {
        return;
    }

    let v: Vec<Option<Point>> = app.undo.history.iter().map(|x| x.selection).collect();
    app.undo.index -= 1;
}

pub fn forward(app: &mut App) {
    if app.undo.index < app.undo.history.len() {
        apply_undo(app, app.undo.history[app.undo.index].clone());
        app.undo.index += 1;
    }
}
