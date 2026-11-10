mod gpu;

use crate::Result;
use std::num::NonZeroU32;
use wgpu::{
    PipelineCompilationOptions, RenderPassColorAttachment, RenderPassDescriptor,
    TextureViewDescriptor,
    util::{BufferInitDescriptor, DeviceExt},
    wgt::WgpuHasDisplayHandle,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::NoUninit)]
pub struct GpuVertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl GpuVertex {
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: core::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
        ],
    };
}

const TRIANGLE: [GpuVertex; 3] = [
    GpuVertex {
        position: [-0.5, -0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    GpuVertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    GpuVertex {
        position: [0.0, 0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

const TRIANGLE_INDEX: [u16; 3] = [0, 1, 2];

pub struct Engine {
    gpu: gpu::GpuContext,
    triangle_render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl Engine {
    pub fn new<T>(display: T, width: NonZeroU32, height: NonZeroU32) -> Result<Self>
    where
        T: WgpuHasDisplayHandle + Into<wgpu::SurfaceTarget<'static>> + Clone,
    {
        let gpu = pollster::block_on(gpu::GpuContext::new(display, width, height))?;

        let triangle_pipeline_layout =
            gpu.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("triangle_pipeline_layout"),
                    bind_group_layouts: &[],
                    immediate_size: 0,
                });

        let vertex_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(&TRIANGLE),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(&TRIANGLE_INDEX),
            usage: wgpu::BufferUsages::INDEX,
        });

        let shader_module = gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("shader_module"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("../../res/target/triangle.wgsl").into(),
                ),
            });

        let triangle_render_pipeline =
            gpu.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("triangle_render_pipeline"),
                    layout: Some(&triangle_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader_module,
                        entry_point: Some("vs_main"),
                        compilation_options: PipelineCompilationOptions::default(),
                        buffers: &[Some(GpuVertex::LAYOUT)],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleStrip,
                        strip_index_format: Some(wgpu::IndexFormat::Uint16),
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        ..Default::default()
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    fragment: Some(wgpu::FragmentState {
                        module: &shader_module,
                        entry_point: Some("fs_main"),
                        compilation_options: PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: gpu.config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    multiview_mask: None,
                    cache: None,
                });

        Ok(Self {
            gpu,
            triangle_render_pipeline,
            vertex_buffer,
            index_buffer,
        })
    }

    pub fn render_frame(&self) {
        let Some((surface_texture, reconfig_surface)) = self.gpu.acquire_surface() else {
            log::warn!("Could not obtain surface texture, skipping frame...");
            return;
        };

        let surface_view = surface_texture.texture.create_view(&TextureViewDescriptor {
            label: Some("surface_view"),
            format: Some(self.gpu.config.format),
            dimension: Some(wgpu::TextureViewDimension::D2),
            usage: Some(self.gpu.config.usage),
            ..Default::default()
        });

        let mut command_encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &surface_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.3,
                        g: 0.5,
                        b: 0.4,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        render_pass.set_pipeline(&self.triangle_render_pipeline);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..3, 0, 0..1);

        drop(render_pass);

        self.gpu.queue.submit([command_encoder.finish()]);
        self.gpu.queue.present(surface_texture);

        if reconfig_surface {
            self.gpu.configure_surface();
        }
    }

    pub fn resize_surface(&mut self, width: NonZeroU32, height: NonZeroU32) {
        self.gpu.configure_surface_with_size(width, height);
    }
}
