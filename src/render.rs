use crate::app;
use crate::rect::rect_renderer::create_rect;
use crate::matrix;
use crate::cursor::Point;

use crate::glfw::Context;

use matrix::Matrix;
use app::App;

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


fn render_selection(app: &mut App, projection: &Matrix) {
    let height = app.font_renderer.advance_height;

    if app.selection.is_none() {
        return;
    }

    let selection = app.selection.unwrap();

    let (start, end) = if selection.y == app.cursor.position.y {
        if selection.x >= app.cursor.position.x {
            (app.cursor.position, selection)
        } else {
            (selection, app.cursor.position)
        }
    } else {
        if selection.y > app.cursor.position.y {
            (app.cursor.position, selection)
        } else {
            (selection, app.cursor.position)
        }
    };

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
                app.text.line(start.y as usize).len_chars() as i64 - start.x,
            ),
            height,
            [0.5, 0.5, 0.5],
        ));

        for (i, l) in app
            .text
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

fn clamp_scroll(app: &mut App) {
    let visible_range = app::visible_range(app);
    if app.cursor.position.y > visible_range.end as i64 - 2 {
        app.scroll.1 =
            app.cursor.position.y + 2 - ((visible_range.end - visible_range.start) as i64);
    }

    if app.cursor.position.y < visible_range.start as i64 {
        app.scroll.1 = app.cursor.position.y;
    }
}

fn mvp_matrix(app: &App) -> Matrix {
    let screen_scroll = app.scroll.1 as f32 * app.font_renderer.advance_height;
    matrix::mul(&app.projection, &matrix::translate(0.0, screen_scroll, 0.0))
}

fn render_cursor(app: &mut App, mvp: &matrix::Matrix) {
    let width = 2.0;
    let height = app.font_renderer.advance_height;
    let screen_pos = grid_to_screen(app, app.cursor.position);

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

        clamp_scroll(app);

        let mvp = mvp_matrix(app);
        render_selection(app, &mvp);
        render_cursor(app, &mvp);

        app.font_renderer
            .render(&app.text, app::visible_range(app), &mvp);

        app.window.swap_buffers();
        app.should_rerender = false;
    }
}