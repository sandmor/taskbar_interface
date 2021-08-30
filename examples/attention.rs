use std::time::Instant;

use glutin::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};
use raw_window_handle::HasRawWindowHandle;
use taskbar_interface::TaskbarInterface;

fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();
    let mut indicator =
        TaskbarInterface::new(windowed_context.window().raw_window_handle()).unwrap();
    #[cfg(all(unix, not(target_os = "macos")))]
    let _ = indicator.set_unity_app_uri("application://myapp.desktop");

    let start = Instant::now();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: _,
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                indicator.needs_attention(start.elapsed().as_secs() % 10 <= 5);
            }
            _ => (),
        }
    });
}
