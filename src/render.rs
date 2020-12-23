use crate::app;
use crate::cursor::Point;
use crate::matrix;
use crate::rect::rect_renderer::create_rect;

use crate::glfw::Context;

use app::App;
use matrix::Matrix;

fn x_to_screen(app: &App, x: i64) -> f32 {
    let char_width = app.font_renderer.char_width;
    x as f32 * char_width
}

fn y_to_screen(app: &App, y: i64) -> f32 {
    let height = app.font_renderer.advance_height;
    ((y + 2) as f32 * height) - app.font_renderer.ascender
}

fn grid_to_screen(app: &App, pos: Point) -> (f32, f32) {
    (x_to_screen(app, pos.x), y_to_screen(app, pos.y))
}

fn render_selection(app: &mut App, projection: &Matrix, range: std::ops::Range<usize>) {
    let height = app.font_renderer.advance_height;

    if app.text.get_cursor().selection.is_none() {
        return;
    }

    let selection = app.text.get_cursor().selection.unwrap();

    let (mut start, mut end) = if selection.y == app.text.get_cursor().position.y {
        if selection.x >= app.text.get_cursor().position.x {
            (app.text.get_cursor().position, selection)
        } else {
            (selection, app.text.get_cursor().position)
        }
    } else {
        if selection.y > app.text.get_cursor().position.y {
            (app.text.get_cursor().position, selection)
        } else {
            (selection, app.text.get_cursor().position)
        }
    };

    start.y = start.y.max(range.start as i64);
    end.y = end.y.min(range.end as i64);

    let mut v = Vec::new();

    let start_screen = grid_to_screen(app, start);
    if start.y == end.y {
        v.push(create_rect(
            start_screen.0,
            start_screen.1,
            x_to_screen(app, end.x - start.x),
            height,
            [0.5, 0.5, 0.5],
        ));
    } else {
        v.push(create_rect(
            start_screen.0,
            start_screen.1,
            x_to_screen(
                app,
                app.text.get_text().line(start.y as usize).len_chars() as i64 - start.x,
            ),
            height,
            [0.5, 0.5, 0.5],
        ));

        for (i, l) in app
            .text
            .get_text()
            .lines_at(start.y as usize + 1)
            .take((end.y - start.y) as usize - 1)
            .enumerate()
        {
            let line = i as i64 + start.y + 1;

            v.push(create_rect(
                0.0,
                y_to_screen(app, line),
                x_to_screen(app, l.len_chars() as i64),
                height,
                [0.5, 0.5, 0.5],
            ));
        }

        let end_screen = grid_to_screen(app, end);

        v.push(create_rect(
            0.0,
            end_screen.1,
            end_screen.0,
            height,
            [0.5, 0.5, 0.5],
        ));
    };

    app.rect_renderer.render(&v, &projection);
}

fn mvp_matrix(app: &App) -> Matrix {
    let screen_scroll = app.scroll.current_scroll.y * app.font_renderer.advance_height;
    let x_scroll = app.scroll.current_scroll.x * app.font_renderer.char_width;
    matrix::mul(
        &app.projection,
        &matrix::translate(-x_scroll, screen_scroll, 0.0),
    )
}

fn render_cursor(app: &mut App, mvp: &matrix::Matrix) {
    let width = 2.0;
    let height = app.font_renderer.advance_height;

    let cursor_position = app.text.get_cursor().position;
    let screen_pos = grid_to_screen(app, cursor_position);

    app.rect_renderer.render(
        &vec![create_rect(
            screen_pos.0,
            screen_pos.1,
            width,
            height,
            [1.0, 1.0, 1.0],
        )],
        &mvp,
    );
}

pub fn render_app(app: &mut App) {
    if app.should_rerender {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let visible_range = app::visible_range(app, app.scroll.current_scroll.y);

        let mvp = mvp_matrix(app);
        render_selection(app, &mvp, visible_range.clone());
        render_cursor(app, &mvp);

        app.font_renderer
            .render(&app.text.get_text(), visible_range, &mvp);

        app.window.swap_buffers();
        app.should_rerender = false;
    }
}
