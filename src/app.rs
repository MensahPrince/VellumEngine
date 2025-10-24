// src/app.rs
use crate::{window::WindowManager, renderer::Renderer, game_loop::GameLoop, input::InputManager};
use winit::{
    application::ApplicationHandler,
    event_loop::ActiveEventLoop,
    window::WindowId,
    event::WindowEvent,
    keyboard::{KeyCode, PhysicalKey}, // FIXED: Changed imports for key handling
};

pub struct VellumApp {
    window_manager: WindowManager,
    renderer: Renderer,
    game_loop: GameLoop,
    input_manager: InputManager,
}

impl VellumApp {
    pub fn new() -> Self {
        Self {
            window_manager: WindowManager::new(),
            renderer: Renderer::new(),
            game_loop: GameLoop::new(60.0),
            input_manager: InputManager::new(),
        }
    }
}

impl ApplicationHandler for VellumApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_manager.window.is_none() {
            if let Err(e) = self.window_manager.create_window(event_loop) {
                log::error!("Failed to create window: {}", e);
                event_loop.exit();
                return;
            }
            if let Some(window) = &self.window_manager.window {
                if let Err(e) = pollster::block_on(self.renderer.initialize(window.clone())) {
                    log::error!("Failed to initialize renderer: {}", e);
                    event_loop.exit();
                    return;
                }
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        self.input_manager.handle_event(&event);
        match event {
            WindowEvent::Resized(size) => {
                self.renderer.resize(size.width, size.height);
                self.window_manager.handle_window_event(event_loop, id, event);
            }
            _ => self.window_manager.handle_window_event(event_loop, id, event),
        }

        // FIXED: Changed from NamedKey::W to KeyCode::KeyW
        if self.input_manager.is_key_pressed(PhysicalKey::Code(KeyCode::KeyW)) {
            log::info!("W key is pressed!");
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) { // FIXED: Added underscore
        let (delta_time, update_count) = self.game_loop.tick();
        for _ in 0..update_count {
            self.renderer.scene.update(delta_time);
            if let Some(device) = &self.renderer.device {
                self.renderer.scene.initialize_buffer(device);
            }
        }
        log::info!("Delta time: {:.4}ms, Updates: {}", delta_time * 1000.0, update_count);
        self.renderer.render();
        self.window_manager.request_redraw();
    }
}