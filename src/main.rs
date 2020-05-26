extern crate gl;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

mod matrix;
mod renderer;
mod shaders;
mod font;

pub struct App {
    renderer: renderer::Renderer,
}

impl App {
    fn new(size: glutin::dpi::PhysicalSize<u32>) -> App {
        return App {
            renderer: renderer::Renderer::new(size),
        };
    }
}

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Megalodon");

    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 6)))
        .build_windowed(wb, &el)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

    let mut app = App::new(windowed_context.window().inner_size());


    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    app.renderer.on_resize(physical_size);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { .. } => {
                    windowed_context.window().request_redraw();
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                app.renderer.render();

                // app.render(&args, &mut gl);
                // println!("{}", now.elapsed().as_secs_f64() * 1000.0);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
