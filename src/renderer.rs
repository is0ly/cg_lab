use std::sync::Arc;

use wgpu::{util::DeviceExt, PipelineCompilationOptions};
use winit::{
    dpi::PhysicalSize,
    event_loop::OwnedDisplayHandle,
    window::{Window, WindowId},
};

use crate::render_data::{ColoredMesh, ColoredVertex};

pub struct Renderer {
    instance: wgpu::Instance,
    window: Arc<Window>,

    device: wgpu::Device,
    queue: wgpu::Queue,

    size: PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,

    canvas_format: wgpu::TextureFormat,
    canvas_texture: wgpu::Texture,
    canvas_view: wgpu::TextureView,
    canvas_sampler: wgpu::Sampler,
    canvas_bind_group_layout: wgpu::BindGroupLayout,
    canvas_bind_group: wgpu::BindGroup,
    canvas_initialized: bool,

    render_pipeline: wgpu::RenderPipeline,
    present_pipeline: wgpu::RenderPipeline,

    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    index_count: u32,
}

impl Renderer {
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(display),
        ));

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .expect("Failed to find a suitable GPU adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("Failed to create device and queue");

        let size = window.inner_size();

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = capabilities.formats[0];
        let canvas_format = surface_format.add_srgb_suffix();

        let colored_mesh_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("colored mesh shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/colored_mesh.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("colored mesh render pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &colored_mesh_shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[Some(ColoredVertex::LAYOUT)],
            },
            fragment: Some(wgpu::FragmentState {
                module: &colored_mesh_shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: canvas_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let (canvas_texture, canvas_view) =
            Self::create_canvas_texture(&device, size, canvas_format);

        let canvas_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("canvas sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let canvas_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("canvas bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let canvas_bind_group = Self::create_canvas_bind_group(
            &device,
            &canvas_bind_group_layout,
            &canvas_view,
            &canvas_sampler,
        );

        let present_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("present texture shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/present_texture.wgsl").into()),
        });

        let present_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("present texture pipeline layout"),
                bind_group_layouts: &[Some(&canvas_bind_group_layout)],
                immediate_size: 0,
            });

        let present_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("present texture pipeline"),
            layout: Some(&present_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &present_shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &present_shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: canvas_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let renderer = Self {
            instance,
            window,
            device,
            queue,
            size,
            surface,
            surface_format,

            canvas_format,
            canvas_texture,
            canvas_view,
            canvas_sampler,
            canvas_bind_group_layout,
            canvas_bind_group,
            canvas_initialized: false,

            render_pipeline,
            present_pipeline,

            vertex_buffer: None,
            index_buffer: None,
            index_count: 0,
        };

        renderer.configure_surface();

        renderer
    }

    pub fn window_id(&self) -> WindowId {
        self.window.id()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn request_canvas_clear(&mut self) {
        self.canvas_initialized = false;
    }

    pub fn upload_colored_mesh(&mut self, mesh: &ColoredMesh) {
        if mesh.index_count() == 0 {
            self.vertex_buffer = None;
            self.index_buffer = None;
            self.index_count = 0;
            return;
        }

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("colored mesh vertex buffer"),
                contents: bytemuck::cast_slice(mesh.vertices()),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("colored mesh index buffer"),
                contents: bytemuck::cast_slice(mesh.indices()),
                usage: wgpu::BufferUsages::INDEX,
            });

        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.index_count = mesh.index_count();
    }

    fn configure_surface(&self) {
        if self.size.width == 0 || self.size.height == 0 {
            return;
        }

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };

        self.surface.configure(&self.device, &surface_config);
    }

    fn create_canvas_texture(
        device: &wgpu::Device,
        size: PhysicalSize<u32>,
        format: wgpu::TextureFormat,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("canvas texture"),
            size: wgpu::Extent3d {
                width: size.width.max(1),
                height: size.height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        (texture, view)
    }

    fn create_canvas_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("canvas bind group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }

    fn recreate_canvas(&mut self) {
        let (canvas_texture, canvas_view) =
            Self::create_canvas_texture(&self.device, self.size, self.canvas_format);

        let canvas_bind_group = Self::create_canvas_bind_group(
            &self.device,
            &self.canvas_bind_group_layout,
            &canvas_view,
            &self.canvas_sampler,
        );

        self.canvas_texture = canvas_texture;
        self.canvas_view = canvas_view;
        self.canvas_bind_group = canvas_bind_group;
        self.canvas_initialized = false;
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        self.size = new_size;
        self.configure_surface();
        self.recreate_canvas();
    }

    pub fn render(&mut self) {
        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,

            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => {
                return;
            }

            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                drop(texture);
                self.configure_surface();
                return;
            }

            wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure_surface();
                return;
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface = self
                    .instance
                    .create_surface(self.window.clone())
                    .expect("Failed to recreate surface");

                self.configure_surface();
                return;
            }

            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("Validation errors should panic before this point");
            }
        };

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("main render encoder"),
            });

        {
            let canvas_load_op = if self.canvas_initialized {
                wgpu::LoadOp::Load
            } else {
                wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.02,
                    g: 0.02,
                    b: 0.05,
                    a: 1.0,
                })
            };

            let mut canvas_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.canvas_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: canvas_load_op,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            if self.index_count > 0
                && let (Some(vertex_buffer), Some(index_buffer)) =
                    (&self.vertex_buffer, &self.index_buffer)
            {
                canvas_pass.set_pipeline(&self.render_pipeline);
                canvas_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                canvas_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                canvas_pass.draw_indexed(0..self.index_count, 0, 0..1);
            }
        }

        self.canvas_initialized = true;

        {
            let mut present_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("present texture render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            present_pass.set_pipeline(&self.present_pipeline);
            present_pass.set_bind_group(0, &self.canvas_bind_group, &[]);
            present_pass.draw(0..3, 0..1);
        }

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        self.queue.present(surface_texture);
    }
}
