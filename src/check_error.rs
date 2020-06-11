#[macro_export]
macro_rules! check_error {
    () => {
        #[cfg(debug_assertions)]
        {
            let line = line!();
            let error;
            #[allow(unused_unsafe)]
            unsafe {
                error = gl::GetError();
            }
            if error != gl::NO_ERROR {
                let message = match error {
                    gl::INVALID_ENUM => "INVALID_ENUM",
                    gl::INVALID_VALUE => "INVALID_VALUE",
                    gl::INVALID_OPERATION => "INVALID_OPERATION",
                    gl::STACK_OVERFLOW => "STACK_OVERFLOW",
                    gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
                    gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
                    gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
                    _ => "Unknown error",
                };
                println!("file: {} error on line {} {}", file!(), line, message);
            }
        }
    };
}
