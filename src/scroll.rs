fn lerp(lower: f32, upper: f32, by: f32) -> f32 {
    lower * (1.0 - by) + upper * by
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Scroll {
    pub base_scroll: PointF,
    pub current_scroll: PointF,
    pub target_scroll: PointF,
    pub animation_time: f32,
    pub started_animating: bool,
}

impl Scroll {
    pub fn new() -> Scroll {
        Scroll {
            base_scroll: PointF { x: 0.0, y: 0.0 },
            current_scroll: PointF { x: 0.0, y: 0.0 },
            target_scroll: PointF { x: 0.0, y: 0.0 },
            animation_time: 0.0,
            started_animating: false,
        }
    }
}

pub fn scroll_to(scroll: &mut Scroll, y: f32) {
    if (scroll.current_scroll.y - y).abs() <= 2.0 {
        scroll.current_scroll.y = y;
        scroll.target_scroll = scroll.current_scroll;
        scroll.base_scroll = scroll.current_scroll;
        scroll.animation_time = 0.0;
    } else {
        scroll.target_scroll.y = y;
        scroll.base_scroll = scroll.current_scroll;
        scroll.animation_time = 0.0;
    }
}

pub fn scroll_to_x(scroll: &mut Scroll, x: f32) {
    scroll.current_scroll.x = x
}

pub fn advance_scroll(scroll: &mut Scroll, time: f32) -> bool {
    if scroll.base_scroll == scroll.target_scroll {
        return false;
    }

    let target_time = 0.1f32;

    if scroll.started_animating {
        scroll.animation_time += time;
    } else {
        scroll.started_animating = true;
    }

    if scroll.animation_time >= target_time {
        scroll.current_scroll = scroll.target_scroll;
        scroll.base_scroll = scroll.target_scroll;
        scroll.animation_time = 0.0;
        scroll.started_animating = false;
    } else {
        scroll.current_scroll.y = lerp(
            scroll.base_scroll.y,
            scroll.target_scroll.y,
            scroll.animation_time / target_time,
        );
    }

    return true;
}
