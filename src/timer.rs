use std::time::Instant;

pub struct Timer {
    timer: Instant,
    text: &'static str,
}

impl Timer {
    #[allow(dead_code)]
    pub fn new(text: &'static str) -> Timer {
        Timer {
            timer: Instant::now(),
            text,
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!(
            "{}, {}",
            self.text,
            self.timer.elapsed().as_secs_f64() * 1000.0
        );
    }
}

#[macro_export]
macro_rules! timer {
    ($text:expr) => {
        // use crate::timer::Timer;
        // let _t = Timer::new($text);
    };
}
