use std::time::Instant;

use raw_window_handle::HasRawWindowHandle;
use taskbar_interface::TaskbarInterface;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut indicator = TaskbarInterface::new(window.raw_window_handle()).unwrap();
    #[cfg(all(unix, not(target_os = "macos")))]
    let _ = indicator.set_unity_app_uri("application://myapp.desktop");
    let start = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                let progress = start.elapsed().as_secs_f64().fract();
                indicator.set_progress(progress);
            }
            _ => (),
        }
    });
}
