use crate::app;

use app::App;

pub fn process_char(app: &mut App, char: &char) {
    let line_idx = app.text.line_to_char(app.cursor_position.y as usize);
    app.text
        .insert_char(line_idx + app.cursor_position.x as usize, char.clone());
    app.cursor_position.x += 1;
    app.cursor_position.remembered_x = app.cursor_position.x;

    app.should_rerender = true;
}

pub fn process_keyboard(
    app: &mut App,
    key: &glfw::Key,
    _scancode: &i32,
    action: &glfw::Action,
    _modifiers: &glfw::Modifiers,
) {
    if *action == glfw::Action::Release {
        return;
    }

    let cursor = &mut app.cursor_position;
    let text = &mut app.text;

    match key {
        glfw::Key::Enter => {
            let line_idx = text.line_to_char(cursor.y);
            text.insert_char(line_idx + cursor.x, '\n');
            cursor.y += 1;
            cursor.x = 0;
            cursor.remembered_x = 0;
            app.should_rerender = true;
        }
        glfw::Key::Tab => {
            let line_idx = text.line_to_char(cursor.y);
            text.insert_char(line_idx + cursor.x, '\t');
            cursor.x += 1;
            cursor.remembered_x = cursor.x;
            app.should_rerender = true;
        }
        glfw::Key::Backspace => {
            if cursor.x != 0 || cursor.y != 0 {
                let old_position = cursor.clone();

                if cursor.x == 0 {
                    if cursor.y > 0 {
                        let p = text.line(cursor.y - 1).len_chars();
                        cursor.x = p - 1;
                        cursor.y -= 1;
                    }
                } else {
                    cursor.x -= 1;
                }

                let line_idx = text.line_to_char(old_position.y);
                cursor.remembered_x = cursor.x;
                text.remove(line_idx + old_position.x - 1..line_idx + old_position.x);
            }

            app.should_rerender = true;
        }
        glfw::Key::Delete => {
            let line_idx = text.line_to_char(cursor.y);

            if text.line(cursor.y).len_chars() > cursor.x {
                text.remove(line_idx + cursor.x..line_idx + cursor.x + 1);
            }
        }
        glfw::Key::Left => {
            if cursor.x > 0 {
                cursor.x -= 1;
                cursor.remembered_x = cursor.x;
                app.should_rerender = true;
            } else if cursor.y > 0 {
                cursor.y -= 1;
                cursor.x = text.line(cursor.y).len_chars() - 1;
                cursor.remembered_x = cursor.x;
            }
        }
        glfw::Key::Right => {
            if cursor.x + 1 < text.line(cursor.y).len_chars() {
                cursor.x += 1;
                cursor.remembered_x = cursor.x;
                app.should_rerender = true;
            } else if cursor.y + 1 < text.len_lines() {
                cursor.x = 0;
                cursor.remembered_x = cursor.x;
                cursor.y += 1;
            }
        }
        glfw::Key::Up => {
            if cursor.y > 0 {
                cursor.y -= 1;
                let line_len = text.line(cursor.y).len_chars() - 1;

                if cursor.remembered_x > line_len {
                    cursor.x = line_len;
                } else {
                    cursor.x = cursor.remembered_x
                }

                app.should_rerender = true;
            }
        }
        glfw::Key::Down => {
            if cursor.y + 1 < text.len_lines() {
                cursor.y += 1;

                let line_len = text.line(cursor.y).len_chars();

                if cursor.remembered_x + 1 > line_len {
                    if line_len == 0 {
                        // last line might not have any chars
                        cursor.x = 0
                    } else {
                        cursor.x = line_len - 1;
                    }
                } else {
                    cursor.x = cursor.remembered_x
                }

                app.should_rerender = true;
            }
        }
        glfw::Key::End => {
            cursor.x = text.line(cursor.y).len_chars() - 1;
            cursor.remembered_x = cursor.x;
            app.should_rerender = true;
        }
        glfw::Key::Home => {
            cursor.x = 0;
            cursor.remembered_x = cursor.x;
            app.should_rerender = true;
		}
		glfw::Key::PageDown => {
			app.scroll.1 += 50.0;
		}
		glfw::Key::PageUp => {
			app.scroll.1 -= 50.0;
		}
        _ => {}
    }
    app.should_rerender = true;
}
