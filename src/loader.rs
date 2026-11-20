use std::path::Path;

use glam::{Mat4, Vec3};
use gltf::{Primitive, mesh::util::ReadColors};

use crate::model::{Mesh, Model, Scene, Transform, Vertex};

pub fn load_gltf(path: impl AsRef<Path>) -> crate::Result<Vec<Scene>> {
    let (document, buffers, _images) = gltf::import(path)?;

    Ok(document
        .scenes()
        .map(|scene| {
            let mut models = Vec::new();

            scene.nodes().for_each(|root_node| {
                visit_node(&root_node, &mut models, &Mat4::IDENTITY, &buffers);
            });

            Scene {
                label: scene.name().map(String::from),
                models,
            }
        })
        .collect())
}

fn visit_node(
    node: &gltf::Node,
    models: &mut Vec<Model>,
    parent_transform: &Mat4,
    buffers: &[gltf::buffer::Data],
) {
    let matrix = parent_transform * Mat4::from_cols_array_2d(&node.transform().matrix());

    if let Some(mesh) = node.mesh() {
        let meshes = visit_mesh(&mesh, buffers);
        models.push(Model {
            label: mesh.name().map(String::from),
            meshes,
            transform: Transform::from_matrix(matrix),
        });
    }

    node.children().for_each(|child| {
        visit_node(&child, models, &matrix, buffers);
    });
}

fn visit_mesh(mesh: &gltf::Mesh, buffers: &[gltf::buffer::Data]) -> Vec<Mesh> {
    mesh.primitives()
        .map(|primitive| {
            assert_eq!(primitive.mode(), gltf::mesh::Mode::Triangles); // Only can draw triangles for now

            let props = read_primitive(&primitive, buffers);

            debug_assert_eq!(props.colors.len(), props.positions.len());

            let vertices: Vec<Vertex> = props
                .positions
                .into_iter()
                .zip(props.colors)
                .map(|(position, color)| Vertex { position, color })
                .collect();

            Mesh {
                label: mesh
                    .name()
                    .map(|x| format!("{x}-primitive-{}", primitive.index())),
                vertices,
                indices: props.indices,
            }
        })
        .collect()
}

fn read_primitive(
    primitive: &Primitive,
    buffers: &[gltf::buffer::Data],
) -> PrimitiveVertexProperties {
    let reader = primitive.reader(|buffer| buffers.get(buffer.index()).map(|v| &**v));

    let positions = reader
        .read_positions()
        .into_iter()
        .flatten()
        .map(Vec3::from_array)
        .collect::<Vec<_>>();

    let indices = reader.read_indices().map_or_else(
        || (0..positions.len() as u32).collect(),
        |x| x.into_u32().collect(),
    );

    let colors = reader
        .read_colors(0)
        .map(ReadColors::into_rgb_f32)
        .map_or_else(
            || vec![Vec3::ONE; positions.len()],
            |x| x.map(Vec3::from_array).collect(),
        );

    PrimitiveVertexProperties {
        positions,
        indices,
        colors,
    }
}

struct PrimitiveVertexProperties {
    positions: Vec<Vec3>,
    indices: Vec<u32>,
    colors: Vec<Vec3>,
}
