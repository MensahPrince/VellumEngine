// src/input.rs
use winit::event::{WindowEvent, ElementState, KeyEvent};
use winit::keyboard::PhysicalKey; // FIXED: Changed to PhysicalKey
use std::collections::HashSet;

pub struct InputManager {
    keys_pressed: HashSet<PhysicalKey>, // FIXED: Changed from NamedKey to PhysicalKey
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { 
                event: KeyEvent { physical_key, state, .. }, // FIXED: Changed from logical_key to physical_key
                .. 
            } => {
                match state {
                    ElementState::Pressed => {
                        self.keys_pressed.insert(*physical_key);
                    }
                    ElementState::Released => {
                        self.keys_pressed.remove(physical_key);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn is_key_pressed(&self, key: PhysicalKey) -> bool { // FIXED: Changed parameter type
        self.keys_pressed.contains(&key)
    }
}