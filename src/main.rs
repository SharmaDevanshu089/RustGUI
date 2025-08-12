use winit::{event::{self, Event}, event_loop::{self, EventLoop}, window::WindowBuilder};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
    .with_title("New Window")
    .build(&event_loop)
    .unwrap();
    event_loop.run(move | event , elwt| 
        {
            
        }).unwrap();
}
