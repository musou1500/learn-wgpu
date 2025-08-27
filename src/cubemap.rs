use crate::texture;

pub struct Cubemap {
    texture_format: wgpu::TextureFormat,
    equirect_layout: wgpu::BindGroupLayout,
    equirect_to_cubemap: wgpu::ComputePipeline,
}

impl Cubemap {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture_format = wgpu::TextureFormat::Rgba32Float;

        let equirect_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: texture_format,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                    },
                    count: None,
                },
            ],
            label: Some("equirect_to_cubemap_bind_group_layout"),
        });

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Equirectangular To Cubemap Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./equirectangular.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Equirect To Cubemap Pipeline Layout"),
            bind_group_layouts: &[&equirect_layout],
            push_constant_ranges: &[],
        });

        let equirect_to_cubemap =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Equirect To Cubemap Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: Some("compute_equirect_to_cubemap"),
                compilation_options: Default::default(),
                cache: None,
            });

        Self {
            texture_format,
            equirect_layout,
            equirect_to_cubemap,
        }
    }

    pub fn build(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[u8],
        dst_size: u32,
        label: &str,
    ) -> anyhow::Result<CubeTexture> {
        let texture = texture::Texture::from_bytes(device, queue, data, label, false)?;
        let dst = CubeTexture::create_2d(device, dst_size, dst_size, self.texture_format);

        let dst_view = dst.texture().create_view(&wgpu::TextureViewDescriptor {
            label: Some(label),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout: &self.equirect_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&dst_view),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&Default::default());
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some(label),
            timestamp_writes: None,
        });

        let num_workgroups = dst_size.div_ceil(16);
        pass.set_pipeline(&self.equirect_to_cubemap);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(num_workgroups, num_workgroups, 6);
        drop(pass);
        queue.submit([encoder.finish()]);
        Ok(dst)
    }
}

pub struct CubeTexture {
    texture: wgpu::Texture,
    sampler: wgpu::Sampler,
    view: wgpu::TextureView,
}

impl CubeTexture {
    pub fn create_2d(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
    ) -> Self {
        let mip_level_count = 1;
        let usage = wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::TEXTURE_BINDING;
        let mag_filter = wgpu::FilterMode::Nearest;
        let label = Some("Cubemap Texture");
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size: wgpu::Extent3d {
                width,
                height,
                // A cube has 6 sides, so we need 6 layers
                depth_or_array_layers: 6,
            },
            mip_level_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label,
            dimension: Some(wgpu::TextureViewDimension::Cube),
            array_layer_count: Some(6),
            ..Default::default()
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            sampler,
            view,
        }
    }

    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}
