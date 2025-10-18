# VellumEngine Roadmap

## Overview
VellumEngine is a lightweight 2D graphics engine built in Rust using winit and wgpu. Goal: Provide a modular, performant base for 2D games/apps with easy extensibility. Target platforms: Desktop (Windows/Linux/Mac), Web (WASM).

Current Status: Basic window + red triangle rendering (MVP).

## Phase 1: Core Foundation (1-2 weeks)
- **Milestone**: Stable rendering pipeline with basic primitives.
- **Tasks**:
  - Refactor code into modules (graphics, window, renderer).
  - Replace hardcoded triangle with vertex buffer for dynamic shapes (e.g., draw rects/circles).
  - Implement efficient redraw (event-driven, vsync).
  - Add basic input handling (keyboard/mouse).
  - Error handling and logging.
- **Success Criteria**: Render multiple colored shapes; handle window resize/close without crashes.

## Phase 2: 2D Rendering Features (2-4 weeks)
- **Milestone**: Support for sprites, textures, and basic transformations.
- **Tasks**:
  - Add texture loading and sampling (use `image` crate).
  - Implement orthographic camera (uniforms for zoom/pan).
  - Sprite batching for performance (single draw call for multiple sprites).
  - Basic blending (alpha transparency).
  - Simple shader effects (e.g., tinting).
- **Success Criteria**: Load and render textured sprites; animate position/scale.

## Phase 3: Scene and Asset Management (3-4 weeks)
- **Milestone**: Entity-based scene graph.
- **Tasks**:
  - Introduce ECS-like system (entities with components: Transform, Sprite).
  - Async asset loader (textures, shaders, fonts via `rusttype` or `glyph_brush`).
  - Scene hierarchy (parent-child transforms).
  - Basic UI elements (buttons, text).
- **Success Criteria**: Build a simple scene with interactive elements (e.g., clickable sprites).

## Phase 4: Advanced Features and Optimization (4-6 weeks)
- **Milestone**: Production-ready engine.
- **Tasks**:
  - Performance: Instancing, GPU queries, profiling.
  - Audio integration (e.g., `rodio` for sound).
  - Physics plugin hook (integrate with `rapier2d`).
  - Multi-platform testing (WASM build).
  - Documentation and examples (e.g., a demo game).
- **Success Criteria**: Run a full 2D game prototype (e.g., pong) at 60+ FPS.

## Phase 5: Release and Community (Ongoing)
- **Milestone**: Open-source v1.0.
- **Tasks**:
  - Publish to crates.io.
  - Add CI/CD (tests, builds).
  - Gather feedback; add requested features (e.g., particle systems).
- **Success Criteria**: Usable by others; positive GitHub stars/issues.

## Risks and Dependencies
- Dependencies: Keep minimal (winit, wgpu, image, etc.). Avoid heavy frameworks like Bevy.
- Risks: WebGPU adoption (browser support); performance on low-end hardware.
- Metrics: Track FPS, draw calls, memory usage.

This roadmap is flexible—adjust based on feedback or priorities.