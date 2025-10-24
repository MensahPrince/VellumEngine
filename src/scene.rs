// src/scene.rs
use wgpu::util::DeviceExt;

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 2],
}

#[derive(Clone)]
pub struct Entity {
    vertices: Vec<Vertex>,
    position: [f32; 2],
}

pub struct Scene {
    entities: Vec<Entity>,
    vertex_buffer: Option<wgpu::Buffer>,
}

impl Scene {
    pub fn new() -> Self {
        let triangle = Entity {
            vertices: vec![
                Vertex { position: [0.0, 0.5] },
                Vertex { position: [-0.5, -0.5] },
                Vertex { position: [0.5, -0.5] },
            ],
            position: [0.0, 0.0],
        };
        Self {
            entities: vec![triangle],
            vertex_buffer: None,
        }
    }

    pub fn initialize_buffer(&mut self, device: &wgpu::Device) {
        let vertices: Vec<Vertex> = self.entities.iter()
            .flat_map(|entity| {
                entity.vertices.iter().map(move |v| Vertex {
                    position: [v.position[0] + entity.position[0], v.position[1] + entity.position[1]]
                })
            })
            .collect();
        
        self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }));
    }

    pub fn vertex_buffer(&self) -> Option<&wgpu::Buffer> {
        self.vertex_buffer.as_ref()
    }

    pub fn vertex_count(&self) -> u32 {
        self.entities.iter().map(|e| e.vertices.len() as u32).sum()
    }

    pub fn update(&mut self, delta_time: f64) {
        if !self.entities.is_empty() {
            self.entities[0].position[0] += (delta_time * 0.5) as f32; // Move at 0.5 units/sec
        }
    }
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}