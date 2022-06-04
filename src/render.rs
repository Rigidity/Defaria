use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use crate::world::World;

#[derive(Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texcoord: [f32; 2],
}

#[derive(Debug)]
pub struct MeshFragment {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Copy, Clone)]
pub enum BlockFace {
    Front,
    Back,
    Top,
    Bottom,
    Left,
    Right,
}

pub struct BlockModel<'a> {
    pub texture: &'a str,
    pub generate_mesh: fn(world: &World, position: IVec3, texture: Rect<f32>) -> MeshFragment,
}

pub fn build_mesh(mesh_fragment: MeshFragment) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut texcoords = Vec::new();

    for vertex in mesh_fragment.vertices {
        positions.push(vertex.position);
        normals.push(vertex.normal);
        texcoords.push(vertex.texcoord);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, texcoords);

    mesh.set_indices(Some(Indices::U32(mesh_fragment.indices)));

    mesh
}

pub fn aggregate_mesh_fragments(mesh_fragments: Vec<MeshFragment>) -> MeshFragment {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut next_index = 0;

    for mesh_fragment in mesh_fragments {
        let first_index = next_index;

        for vertex in mesh_fragment.vertices {
            next_index += 1;

            vertices.push(vertex);
        }

        for index in mesh_fragment.indices {
            indices.push(first_index + index);
        }
    }

    MeshFragment { vertices, indices }
}

pub fn generate_block(position: IVec3, textures: [Option<Rect<f32>>; 6]) -> MeshFragment {
    let mut block_faces: Vec<MeshFragment> = vec![];

    for (i, block_face) in [
        BlockFace::Front,
        BlockFace::Back,
        BlockFace::Left,
        BlockFace::Right,
        BlockFace::Top,
        BlockFace::Bottom,
    ]
    .iter()
    .enumerate()
    {
        if let Some(texture) = textures[i] {
            block_faces.push(generate_block_face(*block_face, position, texture));
        }
    }

    aggregate_mesh_fragments(block_faces)
}

pub fn generate_block_face(face: BlockFace, position: IVec3, texture: Rect<f32>) -> MeshFragment {
    let left = position.x as f32;
    let right = left + 1.0;
    let bottom = position.y as f32;
    let top = bottom + 1.0;
    let back = position.z as f32;
    let front = back + 1.0;

    MeshFragment {
        vertices: match face {
            BlockFace::Front => vec![
                Vertex {
                    position: [left, top, front],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [texture.left, texture.top],
                },
                Vertex {
                    position: [left, bottom, front],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [right, bottom, front],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [texture.right, texture.bottom],
                },
                Vertex {
                    position: [right, top, front],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [texture.right, texture.top],
                },
            ],
            BlockFace::Back => vec![
                Vertex {
                    position: [right, bottom, back],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [left, bottom, back],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [texture.right, texture.bottom],
                },
                Vertex {
                    position: [left, top, back],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [texture.right, texture.top],
                },
                Vertex {
                    position: [right, top, back],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [texture.left, texture.top],
                },
            ],
            BlockFace::Left => vec![
                Vertex {
                    position: [left, top, back],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [texture.left, texture.top],
                },
                Vertex {
                    position: [left, bottom, back],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [left, bottom, front],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [texture.right, texture.bottom],
                },
                Vertex {
                    position: [left, top, front],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [texture.right, texture.top],
                },
            ],
            BlockFace::Right => vec![
                Vertex {
                    position: [right, bottom, front],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [right, bottom, back],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [texture.right, texture.bottom],
                },
                Vertex {
                    position: [right, top, back],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [texture.right, texture.top],
                },
                Vertex {
                    position: [right, top, front],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [texture.left, texture.top],
                },
            ],
            BlockFace::Top => vec![
                Vertex {
                    position: [right, top, back],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [texture.right, texture.top],
                },
                Vertex {
                    position: [left, top, back],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [texture.left, texture.top],
                },
                Vertex {
                    position: [left, top, front],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [right, top, front],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [texture.right, texture.bottom],
                },
            ],
            BlockFace::Bottom => vec![
                Vertex {
                    position: [left, bottom, front],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [texture.right, texture.top],
                },
                Vertex {
                    position: [left, bottom, back],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [texture.right, texture.bottom],
                },
                Vertex {
                    position: [right, bottom, back],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [right, bottom, front],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [texture.left, texture.top],
                },
            ],
        },
        indices: vec![0, 1, 2, 2, 3, 0],
    }
}
