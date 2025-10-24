// src/main.rs
mod window;
mod renderer;
mod app;

use winit::event_loop::{EventLoop, ControlFlow};
use app::VellumApp;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = VellumApp::new();
    let _ = event_loop.run_app(&mut app);
}