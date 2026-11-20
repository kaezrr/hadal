use glam::{Mat4, Quat, Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::NoUninit)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
}

impl Vertex {
    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: core::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
        ],
    };
}

#[derive(Debug)]
pub struct Scene {
    pub label: Option<String>,
    pub models: Vec<Model>,
}

#[derive(Debug)]
pub struct Model {
    pub label: Option<String>,
    pub meshes: Vec<Mesh>,
    pub transform: Transform,
}

#[derive(Debug)]
pub struct Mesh {
    pub label: Option<String>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub const IDENTITY: Self = Self {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    pub fn matrix(&self) -> Mat4 {
        glam::Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    pub fn from_matrix(mat: Mat4) -> Self {
        let (scale, rotation, position) = mat.to_scale_rotation_translation();

        Self {
            position,
            rotation,
            scale,
        }
    }
}
