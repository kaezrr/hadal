use glam::Mat4;
use wgpu::util::DeviceExt;

use crate::model::{Mesh, Model, Scene};

#[derive(Debug)]
pub struct GpuScene {
    pub models: Vec<GpuModel>,
}

impl GpuScene {
    pub fn new(scene: &Scene, device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let models = scene
            .models
            .iter()
            .map(|model| GpuModel::new(model, device, layout))
            .collect();

        Self { models }
    }
}

#[derive(Debug)]
pub struct GpuModel {
    pub meshes: Vec<GpuMesh>,
    pub transform: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl GpuModel {
    const TRANSFORM_BINDING: u32 = 0;

    pub fn new(model: &Model, device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let meshes = model
            .meshes
            .iter()
            .map(|mesh| GpuMesh::new(mesh, device))
            .collect();

        let transform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: model.label.as_deref(),
            contents: bytemuck::bytes_of(&model.transform.matrix()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: model.label.as_deref(),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: Self::TRANSFORM_BINDING,
                resource: transform.as_entire_binding(),
            }],
        });

        Self {
            meshes,
            transform,
            bind_group,
        }
    }

    pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("GpuModel::bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: Self::TRANSFORM_BINDING,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(core::mem::size_of::<Mat4>() as u64),
                },
                count: None,
            }],
        })
    }
}

#[derive(Debug)]
pub struct GpuMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl GpuMesh {
    fn new(mesh: &Mesh, device: &wgpu::Device) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: mesh.label.as_deref(),
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: mesh.label.as_deref(),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: mesh.indices.len() as u32,
        }
    }
}
