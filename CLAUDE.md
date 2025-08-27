# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust graphics application built with `wgpu` and `winit` that renders 3D models with lighting and skybox support. The project demonstrates modern WebGPU-based 3D rendering in Rust with camera controls, instanced rendering, and shader-based graphics.

## Build and Run Commands

- **Build the project**: `cargo build`
- **Run the application**: `cargo run` (uses `src/bin/main.rs` as the main executable)
- **Build for release**: `cargo build --release`
- **Run with release optimizations**: `cargo run --release`

## Architecture Overview

The application follows a modular graphics rendering architecture:

### Core Components

- **`src/state.rs`**: Contains the `WindowState` struct - the main application state managing the wgpu surface, device, camera, models, and render pipelines
- **`src/bin/main.rs`**: Application entry point with winit event loop handling
- **`src/camera.rs`**: First-person camera system with projection matrix and controller
- **`src/model.rs`**: 3D model loading, vertex definitions, and rendering traits
- **`src/texture.rs`**: Texture loading and management
- **`src/light.rs`**: Lighting system with uniforms and render pipeline
- **`src/resources.rs`**: Resource loading utilities for models and textures
- **`src/render_pipeline.rs`**: Render pipeline creation utilities

### Rendering Pipeline Structure

The application uses multiple render passes:
1. Light rendering pass (for light visualization)
2. Main object rendering pass (instanced cube rendering)
3. Skybox rendering pass (cube-mapped environment)

### Key Features

- **Instanced Rendering**: Renders 100 cubes (10x10 grid) with individual transformations
- **PBR Lighting**: Point light with rotating animation
- **Skybox**: Cube-mapped environment using equirectangular-to-cubemap conversion
- **Camera Controls**: WASD movement, mouse look, and scroll wheel zoom
- **Asset Loading**: OBJ model loading with material support

### WGSL Shaders

- **`shader.wgsl`**: Main vertex/fragment shader for 3D models
- **`sky.wgsl`**: Skybox rendering shader
- **`src/light.wgsl`**: Light visualization shader
- **`equirectangular.wgsl`**: Equirectangular to cubemap conversion compute shader

### Dependencies

Key external crates:
- `wgpu`: Modern graphics API abstraction
- `winit`: Cross-platform window handling
- `cgmath`: Linear algebra and 3D math
- `tobj`: OBJ file loading
- `image`: Image loading and processing
- `bytemuck`: Safe casting between POD types

## Development Notes

- The application targets Rust edition 2024
- Uses async model loading with `pollster::block_on`
- Implements proper depth testing and buffer management
- Color changes based on mouse cursor position for interactive feedback
- All shaders are embedded at compile time using `include_str!` and `include_wgsl!`