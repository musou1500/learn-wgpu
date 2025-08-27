use wgpu::util::DeviceExt as _;

use crate::{
    model::{self, Vertex},
    render_pipeline::create_render_pipeline,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    _padding: u32,
    pub color: [f32; 3],
    _padding2: u32,
}

impl LightUniform {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self {
            position,
            _padding: 0,
            color,
            _padding2: 0,
        }
    }
}

pub struct Light {
    pub uniform: LightUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Light {
    pub fn new(
        device: &wgpu::Device,
        uniform: LightUniform,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let buffer: wgpu::Buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        });

        let shader = wgpu::ShaderModuleDescriptor {
            source: wgpu::ShaderSource::Wgsl(include_str!("./light.wgsl").into()),
            label: Some("Light Shader"),
        };
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Light Pipeline Layout"),
            bind_group_layouts: &[camera_bind_group_layout, &bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = create_render_pipeline(
            device,
            &layout,
            color_format,
            depth_format,
            &[model::ModelVertex::desc()],
            shader,
            false, // Light doesn't need wireframe typically
        );

        Self {
            uniform,
            buffer,
            bind_group,
            bind_group_layout,
            render_pipeline,
        }
    }
}
