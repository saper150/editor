extern crate gl;

extern crate glfw;

mod app;
mod check_error;
mod cursor;
mod editor_action;
mod font;
mod matrix;
mod offset_of;
mod process_keyboard;
mod program;
mod rect;
mod render;
mod scroll;
mod shaders;
mod task_executor;
mod text;
mod timer;

fn main() {
    let program = program::Program::new();
    program.run();
}

#[cfg(test)]
mod app_test;
#[cfg(test)]
mod text_test;
