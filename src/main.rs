// src/main.rs
mod window;
mod renderer;
mod game_loop;
mod input;
mod scene;
mod app;

use winit::event_loop::{EventLoop, ControlFlow};
use app::VellumApp;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = VellumApp::new();
    let _ = event_loop.run_app(&mut app);
}