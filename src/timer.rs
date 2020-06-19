
use std::time::{Instant};


pub struct Timer {
	timer: Instant,
}

impl Timer {
	#[allow(dead_code)]
	pub fn new() -> Timer {
		Timer {
			timer: Instant::now()
		}
	}
}

impl Drop for Timer {

	fn drop(&mut self) {
		println!("elapsed: {}", self.timer.elapsed().as_secs_f64() * 1000.0 );
	}

}
