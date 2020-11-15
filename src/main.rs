extern crate gl;

extern crate glfw;

mod app;
mod check_error;
mod cursor;
mod font;
mod matrix;
mod process_keyboard;
mod program;
mod rect;
mod render;
mod shaders;
mod task_executor;
mod text;
mod timer;
mod offset_of;
mod scroll;

fn main() {
    let program = program::Program::new();
    program.run();
}

#[cfg(test)]
mod app_test;
#[cfg(test)]
mod text_test;
