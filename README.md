# Taskbar_interface

A Rust library to communicate with the desktop taskbar, featuring:

- Show a progress indicator in your app taskbar button.
- Highlight your app in the taskbar if it requires urgent attention.

Currently, we only support Windows and Linux, although we would appreciate help to support other platforms(e.g. macOS).

## Usage

Pretty simple, add this to your `Cargo.toml`:

```toml
[dependencies]
taskbar_interface = "0.1"
```

then you only need to plug in this library using the `RawWindowHandle` provide by some other library that you use to create
the window, here is an example for winit:
```rust
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
                indicator.set_progress(progress).unwrap();
            }
            _ => (),
        }
    });
}
```
![Progress](https://user-images.githubusercontent.com/58484439/131411439-8d6c372e-71d9-411c-84b2-d14d04221433.PNG)

And this one is for glutin:
```rust
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
                indicator.needs_attention(start.elapsed().as_secs() % 10 <= 5).unwrap();
            }
            _ => (),
        }
    });
}
```
![Attention](https://user-images.githubusercontent.com/58484439/131412856-298e9fd9-238d-4781-b2c6-e49ff4413beb.PNG)

Currently, there is no way to use this library if your framework/library don't provide you a `RawWindowHandle`.

## Linux support

Currently, the system-side of what this library aims to do is very immature. We did our best but except for Cinnamon desktop
and maybe some others that also make use of `libxapps`. For the majority including KDE Plasma, Plain dock(Elementary OS) and
DockbarX(XFCE, MATE and legacy gnome2) is required to use the "Unity protocol" that includes as a major flaw, that you need
to specific the `.desktop` file of your app. So your app must be property installed in the system, not portable or downloaded
and ran.

This library aims to support all Linux desktops if you find a desktop *that supports* some features listed above and this not work 
in it, don't doubt to open an issue!.
