use std::{io::Read, ptr};

use crate::cursor;

use cursor::{Cursor, Point};
use memchr::memchr_iter;

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
    pub history: Vec<UndoPoint>,
    pub index: usize,
    pub last_added: bool,
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

fn next_word<'a, I>(iter: &mut I) -> i64
where
    I: Iterator<Item = char>,
{
    let mut count: i64 = 0;

    match iter.next() {
        Some(x) => {
            if !x.is_alphanumeric() {
                count += 1;
                while let Some(e) = iter.next() {
                    if e == '\n' {
                        return count;
                    }

                    if e.is_alphanumeric() {
                        break;
                    } else {
                        count += 1;
                    }
                }
            }
        }
        None => {}
    }

    count += iter.take_while(|x| x.is_alphanumeric()).count() as i64;

    return count + 1 as i64;
}

pub enum DeleteDirection {
    Forward,
    Back,
}

struct RemoveCrlf<T: std::io::Read> {
    reader: T,
    matches: Vec<usize>,
}

impl<T: std::io::Read> RemoveCrlf<T> {
    pub fn new(reader: T) -> RemoveCrlf<T> {
        return RemoveCrlf {
            reader,
            matches: vec![],
        };
    }
}

pub fn remove_crlf_from_buff(buf: &mut [u8], matches: &[usize]) {
    let b = buf.as_mut_ptr();
    for i in 0..matches.len() - 1 {
        let current = matches[i] as isize;
        let next = matches[i + 1] as isize;
        unsafe {
            ptr::copy(
                b.offset(current + 1),
                b.offset(current - i as isize),
                (next - current) as usize,
            )
        }
    }
}

impl<T: std::io::Read> Read for RemoveCrlf<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let res = self.reader.read(buf)?;
        let s = &mut buf[..res];

        self.matches.clear();
        self.matches.extend(memchr_iter('\r' as u8, s));
        self.matches.push(res);
        remove_crlf_from_buff(s, &self.matches);

        return Ok(res - (self.matches.len() - 1));
    }
}

#[derive(Clone)]
pub enum Selection {
    Select,
    NotSelect,
}

impl Text {
    pub fn new<T: std::io::Read>(reader: T) -> Text {
        let initial_text = ropey::Rope::from_reader(RemoveCrlf::new(reader)).unwrap();
        let p = UndoPoint {
            text: initial_text,
            cursor: Cursor::new(),
        };
        return Text {
            history: vec![p],
            index: 0,
            last_added: false,
        };
    }

    #[allow(dead_code)]
    pub fn get_string(&self) -> String {
        self.history[self.index].text.to_string()
    }

    pub fn get_cursor(&mut self) -> &mut Cursor {
        &mut self.history[self.index].cursor
    }

    pub fn current_point(&mut self) -> &mut UndoPoint {
        &mut self.history[self.index]
    }

    pub fn get_text(&self) -> &ropey::Rope {
        &self.history[self.index].text
    }

    fn soft_undo_point(&mut self) {
        if !self.last_added {
            self.add_undo_point();
        }

        self.last_added = true;
    }

    pub fn insert_text(&mut self, str: &str) {
        if str.len() > 1 || str.chars().nth(0).unwrap().is_whitespace() {
            self.add_undo_point();
        } else {
            self.soft_undo_point();
        }

        if let Some(range) = self.selection_range() {
            let UndoPoint { text, cursor } = self.current_point();

            text.remove(range.clone());
            text.insert(range.start, str);
            cursor.position = Point::from_char(range.start + str.chars().count(), text);
            cursor.selection = None
        } else {
            let UndoPoint { text, cursor } = self.current_point();
            let start_idx = cursor.position.to_char(text);

            text.insert(start_idx, str);

            let end_idx = start_idx + str.chars().count();
            cursor.position = Point::from_char(end_idx, text);
            cursor.remembered_x = cursor.position.x;
        }
    }

    pub fn delete_text(&mut self, key: DeleteDirection) {
        fn char_to_delete_next(text: &mut ropey::Rope, cursor: &mut Cursor) -> Option<usize> {
            let start_idx = cursor.position.to_char(text);

            if start_idx < text.len_chars() {
                return Some(start_idx);
            }
            None
        }

        fn char_to_delete_previous(text: &mut ropey::Rope, cursor: &mut Cursor) -> Option<usize> {
            let start_idx = cursor.position.to_char(text);

            if start_idx > 0 {
                return Some(start_idx - 1);
            }
            return None;
        }

        if let Some(range) = self.selection_range() {
            self.add_undo_point();

            let UndoPoint { text, cursor } = self.current_point();
            text.remove(range.clone());
            cursor.position = Point::from_char(range.start, text);
            cursor.selection = None;
        } else {
            let char_to_delete = match key {
                DeleteDirection::Forward => {
                    let UndoPoint { text, cursor } = self.current_point();
                    char_to_delete_next(text, cursor)
                }
                DeleteDirection::Back => {
                    let UndoPoint { text, cursor } = self.current_point();
                    char_to_delete_previous(text, cursor)
                }
            };

            if let Some(idx) = char_to_delete {
                if self.current_point().text.char(idx) == '\n' {
                    self.add_undo_point();
                } else {
                    self.soft_undo_point();
                }
                let point = self.current_point();
                point.cursor.position = Point::from_char(idx, &point.text);
                point.text.remove(idx..(idx + 1));
            }
        }
    }

    pub fn add_undo_point(&mut self) {
        self.last_added = true;
        let undo_point = self.current_point().clone();
        self.history.truncate(self.index + 1);
        self.history.push(undo_point);
        self.index = self.history.len() - 1;
    }

    pub fn undo(&mut self) {
        self.last_added = false;
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn redo(&mut self) {
        self.index = (self.index + 1).min(self.history.len() - 1);
    }

    fn process_selection(&mut self, selection: Selection) {
        let cursor = self.get_cursor();
        match selection {
            Selection::Select => {
                cursor.selection = cursor.selection.or(Some(cursor.position.clone()))
            }
            Selection::NotSelect => cursor.selection = None,
        }
    }

    pub fn move_cursor(&mut self, by: i64, selection: Selection) {
        self.process_selection(selection);
        let UndoPoint { cursor, text } = self.current_point();
        let idx = clamp(
            cursor.position.to_char(text) as i64 + by,
            0,
            text.len_chars() as i64,
        );
        cursor.position = Point::from_char(idx as usize, text);
        cursor.remembered_x = cursor.position.x
    }

    pub fn move_cursor_y(&mut self, by: i64, selection: Selection) {
        self.process_selection(selection);
        let UndoPoint { cursor, text } = self.current_point();
        let cursor_idx = cursor.position.to_char(text);
        let line_idx = clamp(
            text.char_to_line(cursor_idx) as i64 + by,
            0,
            text.len_lines() as i64 - 1,
        ) as usize;

        let is_last_line = line_idx + 1 == text.len_lines();

        let max = text.line(line_idx).len_chars() - if is_last_line { 0 } else { 1 };

        let new_idx =
            text.line_to_char(line_idx) + clamp(cursor.remembered_x, 0, max as i64) as usize;

        cursor.position = Point::from_char(new_idx, text);
    }

    pub fn move_to_next_word(&mut self, selection: Selection) {
        self.process_selection(selection.clone());

        let UndoPoint { cursor, text } = self.current_point();

        let idx = cursor.position.to_char(text);
        let move_by = next_word(&mut text.chars_at(idx).into_iter());
        self.move_cursor(move_by, selection);
    }

    pub fn move_to_prev_word(&mut self, selection: Selection) {
        self.process_selection(selection.clone());

        let UndoPoint { cursor, text } = self.current_point();

        let idx = cursor.position.to_char(text);

        let mut iter = BackwardIterator {
            src: &mut text.chars_at(idx).into_iter(),
        };

        let move_by = next_word(&mut iter);
        self.move_cursor(-move_by, selection);
    }

    pub fn move_to_end_of_line(&mut self, selection: Selection) {
        self.process_selection(selection);

        let UndoPoint { cursor, text } = self.current_point();

        let is_last_line = cursor.position.y + 1 == text.len_lines() as i64;

        cursor.position.x = text.line(cursor.position.y as usize).len_chars() as i64;

        if !is_last_line {
            cursor.position.x -= 1;
        }
        cursor.remembered_x = cursor.position.x;
    }

    pub fn move_to_beginning_of_line(&mut self, selection: Selection) {
        self.process_selection(selection);
        let UndoPoint { cursor, .. } = self.current_point();
        cursor.position.x = 0;
        cursor.remembered_x = 0;
    }

    pub fn move_to_end(&mut self, selection: Selection) {
        self.process_selection(selection);
        let UndoPoint { cursor, text } = self.current_point();

        cursor.position = Point::from_char(text.len_chars(), text);
    }

    pub fn move_to_begging(&mut self, selection: Selection) {
        self.process_selection(selection);
        let UndoPoint { cursor, .. } = self.current_point();

        cursor.position = Point { x: 0, y: 0 }
    }

    pub fn get_selection_str(&mut self) -> Option<String> {
        let e = self.selection_range()?;
        let UndoPoint { text, .. } = self.current_point();
        Some(text.slice(e).to_string())
    }

    pub fn selection_range(&mut self) -> Option<std::ops::Range<usize>> {
        let UndoPoint { cursor, text } = self.current_point();
        let a = cursor.position.to_char(text);
        let b = cursor.selection?.to_char(text);
        Some(a.min(b)..b.max(a))
    }

    pub fn get_current_line(&mut self) -> String {
        let UndoPoint { cursor, text } = self.current_point();
        return text.line(cursor.position.y as usize).to_string();
    }

    pub fn remove_selection(&mut self) -> Option<String> {
        let str = self.get_selection_str();
        let range = self.selection_range()?;
        self.add_undo_point();

        let UndoPoint { text, .. } = self.current_point();
        text.remove(range);

        return str;
    }

    pub fn remove_current_line(&mut self) -> String {
        let line_to_remove = self.get_current_line();
        if line_to_remove.len() > 0 {
            self.add_undo_point();
        }

        let UndoPoint { cursor, text } = self.current_point();
        cursor.position.x = 0;
        cursor.remembered_x = 0;
        let start_idx = Point::to_char(&cursor.position, text);
        let end_idx = start_idx + text.line(cursor.position.y as usize).len_chars();

        text.remove(start_idx..end_idx);
        return line_to_remove;
    }
}
