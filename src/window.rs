// src/window.rs
use winit::{
    event::WindowEvent,
    event_loop::{ActiveEventLoop},
    window::{Window, WindowAttributes, WindowId},
};
use std::sync::Arc;

pub struct WindowManager {
    pub window: Option<Arc<Window>>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self { window: None }
    }

    pub fn create_window(&mut self, event_loop: &ActiveEventLoop) -> Result<(), winit::error::OsError> {
        let window_attributes = WindowAttributes::default()
            .with_title("VellumEngine")
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600));
        let window = Arc::new(event_loop.create_window(window_attributes)?);
        self.window = Some(window);
        Ok(())
    }

    pub fn handle_window_event(&self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    pub fn request_redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}