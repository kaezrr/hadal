mod obj;

use wgpu::BufferUsages;
use wgpu::util::DeviceExt;

pub use crate::loaders::obj::load_model_from_obj;
use crate::model::Material;
use crate::model::Mesh;
use crate::model::Model;
use crate::model::ModelVertex;
use crate::model::PropertiesUniform;
use crate::texture::Texture;

pub struct UnloadedModel {
    meshes: Vec<UnloadedMesh>,
    materials: Vec<UnloadedMaterial>,
}

pub struct UnloadedMesh {
    name: String,
    vertices: Vec<ModelVertex>,
    indices: Vec<u32>,
    material_id: Option<usize>,
}

pub struct UnloadedMaterial {
    name: String,
    diffuse_texture: Option<Vec<u8>>,
    normal_texture: Option<Vec<u8>>,
    properties: PropertiesUniform,
}

impl UnloadedModel {
    pub fn initialize(
        self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        material_layout: &wgpu::BindGroupLayout,
    ) -> anyhow::Result<Model> {
        let meshes = self
            .meshes
            .into_iter()
            .map(|mesh| mesh.initialize(device))
            .collect();

        let materials = self
            .materials
            .into_iter()
            .map(|material| material.initialize(device, queue, material_layout))
            .collect::<anyhow::Result<_>>()?;

        Ok(Model { meshes, materials })
    }
}

impl UnloadedMesh {
    pub fn initialize(self, device: &wgpu::Device) -> Mesh {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("vertex_buffer: {}", self.name)),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("index_buffer: {}", self.name)),
            contents: bytemuck::cast_slice(&self.indices),
            usage: BufferUsages::INDEX,
        });

        Mesh {
            name: self.name,
            vertex_buffer,
            index_buffer,
            num_indices: self.indices.len() as u32,
            material_id: self.material_id,
        }
    }
}

impl UnloadedMaterial {
    pub fn initialize(
        self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
    ) -> anyhow::Result<Material> {
        let diffuse_texture = self
            .diffuse_texture
            .map(|bytes| {
                Texture::from_bytes(
                    device,
                    queue,
                    &bytes,
                    wgpu::TextureFormat::Rgba8UnormSrgb,
                    Some(&format!("Diffuse Texture: {}", self.name)),
                )
            })
            .transpose()?;

        let normal_texture = self
            .normal_texture
            .map(|bytes| {
                Texture::from_bytes(
                    device,
                    queue,
                    &bytes,
                    wgpu::TextureFormat::Rgba8Unorm,
                    Some(&format!("Normal Texture: {}", self.name)),
                )
            })
            .transpose()?;

        Ok(Material::new(
            device,
            queue,
            self.name,
            self.properties,
            layout,
            diffuse_texture,
            normal_texture,
        ))
    }
}
