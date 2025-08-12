use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
    .with_title("New Window")
    .build(&event_loop)
    .unwrap();
}
