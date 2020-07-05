

#[derive(Copy, Clone, Debug)]
pub struct Point {
	pub x: i64,
    pub y: i64,
}

impl Point {

	pub fn to_char(&self, text: &ropey::Rope) -> usize {
		text.line_to_char(self.y as usize) + self.x as usize
	}

	pub fn from_char(char: usize, text: &ropey::Rope) -> Point {
		let line = text.char_to_line(char);
		let line_idx = text.line_to_char(line);

		Point {
			y: line as i64,
			x: (char as usize - line_idx) as i64,
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Cursor {
    pub position: Point,
    pub remembered_x: i64,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
			position: Point { x: 0, y: 0 },
            remembered_x: 0,
        }
	}

}
