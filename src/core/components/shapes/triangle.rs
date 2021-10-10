use std::ops::Range;

use wgpu::{util::BufferInitDescriptor, PrimitiveTopology};

use crate::{
    core::components::{material::Material, maths::coordinates::Coordinates},
    rendering::{gl_representations::TexturedGlVertex, Renderable2D},
};

const INDICES: &[u16] = &[1, 0, 2];

/// Renderable 2D Triangle.
pub struct Triangle {
    pub vertices: [Coordinates; 3],
    pub uvs: Option<[Coordinates; 3]>,
    contents: [TexturedGlVertex; 3],
}

impl Triangle {
    /// Creates a new square using `length`.
    /// uvs are mandatory but this will be updated
    pub fn new(vertices: [Coordinates; 3], uvs: Option<[Coordinates; 3]>) -> Self {
        let uvs_ref = uvs.as_ref().expect("Uvs are currently mandatory, this need to be fixed");
        let contents = [
            TexturedGlVertex::from((&vertices[0], &uvs_ref[0])),
            TexturedGlVertex::from((&vertices[1], &uvs_ref[1])),
            TexturedGlVertex::from((&vertices[2], &uvs_ref[2])),
        ];
        Self { vertices, uvs, contents }
    }
}

impl Renderable2D for Triangle {
    fn vertex_buffer_descriptor(&mut self, _material: Option<&Material>) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.contents),
            usage: wgpu::BufferUsages::VERTEX,
        }
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Index Buffer"),
            contents: bytemuck::cast_slice(&INDICES),
            usage: wgpu::BufferUsages::INDEX,
        }
    }

    fn range(&self) -> Range<u32> { 0..3 as u32 }

    fn topology() -> PrimitiveTopology { wgpu::PrimitiveTopology::TriangleList }

    fn dirty(&self) -> bool { false }

    fn set_dirty(&mut self, _is_dirty: bool) {}
}
