use winit::{
    application::ApplicationHandler,  // Trait for handling application lifecycle events
    event::WindowEvent,  // Enum for window-specific events like resize or close
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow},  // Components for managing the event loop: ActiveEventLoop for current state, EventLoop for creation, ControlFlow for loop behavior
    window::{Window, WindowId, WindowAttributes},  // Window struct, its ID, and attributes for customization
};
use wgpu::{Instance, SurfaceConfiguration};  // wgpu types: Instance for GPU context, SurfaceConfiguration for render surface setup
use std::sync::Arc;  // Atomic Reference Counting for thread-safe shared ownership, used for Window

// Main application struct that holds all graphics and window state
// Uses Option for fields to allow lazy initialization during runtime
struct VellumApp {
    window: Option<Arc<Window>>,  // Optional shared window handle
    device: Option<wgpu::Device>,  // Optional GPU device for creating resources
    queue: Option<wgpu::Queue>,  // Optional command queue for submitting GPU work
    surface: Option<wgpu::Surface<'static>>,  // Optional render surface tied to the window
    config: Option<SurfaceConfiguration>,  // Optional configuration for the surface (size, format, etc.)
    render_pipeline: Option<wgpu::RenderPipeline>,  // Optional pre-configured render pipeline for drawing
}

impl VellumApp {
    // Constructor: Initializes the struct with all fields set to None
    fn new() -> Self {
        Self {
            window: None,
            device: None,
            queue: None,
            surface: None,
            config: None,
            render_pipeline: None,
        }
    }

    // Asynchronous function to set up wgpu graphics context
    // Takes a shared window reference and initializes GPU resources
    async fn initialize_graphics(&mut self, window: Arc<Window>) {
        // Create a wgpu instance with all backends enabled for broad compatibility
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // Debugging: List all available GPU adapters to help diagnose hardware issues
        println!("Enumerating adapters:");
        for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
            let info = adapter.get_info();
            println!("  - {} ({:?})", info.name, info.backend);
        }
        
        // Create a surface for rendering, bound to the provided window
        let surface = instance.create_surface(window.clone()).unwrap();
        
        // Request a GPU adapter compatible with the surface (prefers low power)
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await;
        
        // Fallback: If no compatible adapter found, request one without surface check, forcing fallback if needed
        let adapter = match adapter {
            Some(adapter) => adapter,
            None => {
                eprintln!("Warning: No adapter found with surface compatibility, trying without...");
                instance
                    .request_adapter(&wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::LowPower,
                        compatible_surface: None,
                        force_fallback_adapter: true,
                    })
                    .await
                    .expect("Failed to find any suitable GPU adapter. Please ensure your graphics drivers are up to date.")
            }
        };
        
        // Print info about the selected adapter for debugging
        let info = adapter.get_info();
        println!("Using adapter: {} ({:?})", info.name, info.backend);
            
        // Request a logical device and queue from the adapter with minimal requirements
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            .unwrap();

        // Query surface capabilities to determine supported formats and modes
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];  // Pick the first supported format
        // Configure the surface with window size, format, and other settings
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,  // Allow rendering to this texture
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,  // VSync mode for smooth presentation
            alpha_mode: surface_caps.alpha_modes[0],  // First supported alpha mode
            view_formats: vec![],
            desired_maximum_frame_latency: 2,  // Buffer up to 2 frames
        };
        surface.configure(&device, &config);

        // Load and compile the WGSL shader module from file
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        // Create an empty pipeline layout (no bindings or constants)
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        // Create the render pipeline, defining vertex/fragment stages and output format
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",  // Vertex shader entry point
                buffers: &[],  // No vertex buffers (hardcoded in shader)
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",  // Fragment shader entry point
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: None,  // No blending
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),  // Default triangle list topology
            depth_stencil: None,  // No depth/stencil buffer
            multisample: wgpu::MultisampleState::default(),  // No multisampling
            multiview: None,
        });

        // Assign initialized resources to self
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);
        self.config = Some(config);
        self.render_pipeline = Some(render_pipeline);
    }

    // Function to perform a single render frame
    fn render(&mut self) {
        // Early exit if any required state is missing
        let Some(surface) = &self.surface else { return };
        let Some(device) = &self.device else { return };
        let Some(queue) = &self.queue else { return };
        let Some(config) = &self.config else { return };
        let Some(render_pipeline) = &self.render_pipeline else { return };

        // Get the next texture to render to; handle errors like surface loss
        let output = match surface.get_current_texture() {
            Ok(output) => output,
            Err(wgpu::SurfaceError::Lost) => {
                surface.configure(device, config);  // Reconfigure on loss
                return;
            }
            Err(_) => return,
        };
        
        // Create a view of the output texture
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        // Create a command encoder for recording GPU commands
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });
        
        // Scope for render pass
        {
            // Begin a render pass: Clear to black, no depth
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            // Set the pipeline and draw the triangle (3 vertices, 1 instance)
            render_pass.set_pipeline(render_pipeline);
            render_pass.draw(0..3, 0..1);
        }
        
        // Submit the encoded commands to the queue and present the frame
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

impl ApplicationHandler for VellumApp {
    // Called when the app is resumed (e.g., start or focus regain)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            // Create window attributes: Title and initial size
            let window_attributes = WindowAttributes::default()
                .with_title("VellumEngine")
                .with_inner_size(winit::dpi::PhysicalSize::new(800, 600));
            
            // Create and wrap the window in Arc for sharing
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            
            let window_clone = window.clone();
            self.window = Some(window);
            
            // Block on async graphics init
            pollster::block_on(self.initialize_graphics(window_clone));
        }
    }

    // Handle window events like close, resize, or redraw requests
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();  // Exit the event loop on close
            }
            WindowEvent::Resized(size) => {
                // Update surface config on resize, ensure min size 1x1
                if let (Some(surface), Some(device), Some(config)) = 
                    (&self.surface, &self.device, &mut self.config) {
                    config.width = size.width.max(1);
                    config.height = size.height.max(1);
                    surface.configure(device, config);
                }
                // Request a redraw after resize
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                // Perform render and request next frame
                self.render();
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}  // Ignore other events
        }
    }

    // Called before the event loop idles; used to trigger continuous redraws
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

// Entry point: Set up event loop and run the app
fn main() {
    // Create the event loop
    let event_loop = EventLoop::new().unwrap();
    // Set to poll mode for continuous event processing
    event_loop.set_control_flow(ControlFlow::Poll);
    
    // Create the app instance
    let mut app = VellumApp::new();
    // Run the event loop with the app handler
    let _ = event_loop.run_app(&mut app);
}