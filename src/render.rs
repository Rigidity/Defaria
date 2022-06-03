use crate::chunk::Chunk;
use crate::key::Key;
use crate::{AssetHandles, ClientRegistry};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

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
    pub generate_mesh: fn(position: (f32, f32, f32), texture: Rect<f32>) -> MeshFragment,
}

pub fn create_world(
    mut commands: Commands,
    client_registry: Res<ClientRegistry<'static>>,
    asset_handles: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_atlas = texture_atlases
        .get(asset_handles.block_texture_atlas.clone())
        .unwrap();

    let chunk = Chunk {
        blocks: [Key {
            namespace: "defaria",
            name: "arrow",
        }; 4096],
        has_changed: false,
    };

    let mut mesh_fragments: Vec<MeshFragment> = vec![];

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let i = x * 256 + y * 16 + z;

                let block_model = client_registry
                    .block_models
                    .get(&chunk.blocks[i])
                    .expect("Unregistered block model.");

                let vendor_handle = asset_server.get_handle(block_model.texture);
                let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
                let texture_position = texture_atlas.textures[vendor_index];

                let texture = Rect {
                    top: texture_position.min.y / texture_atlas.size.y,
                    left: texture_position.min.x / texture_atlas.size.x,
                    right: texture_position.max.x / texture_atlas.size.x,
                    bottom: texture_position.max.y / texture_atlas.size.y,
                };

                mesh_fragments.push((block_model.generate_mesh)(
                    (x as f32, y as f32, z as f32),
                    texture,
                ));
            }
        }
    }

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(build_mesh(aggregate_mesh_fragments(mesh_fragments))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_atlas.texture.clone()),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.7,
    });
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

pub fn generate_block(position: (f32, f32, f32), textures: [Rect<f32>; 6]) -> MeshFragment {
    aggregate_mesh_fragments(vec![
        generate_block_face(BlockFace::Front, position, textures[0]),
        generate_block_face(BlockFace::Back, position, textures[1]),
        generate_block_face(BlockFace::Left, position, textures[2]),
        generate_block_face(BlockFace::Right, position, textures[3]),
        generate_block_face(BlockFace::Top, position, textures[4]),
        generate_block_face(BlockFace::Bottom, position, textures[5]),
    ])
}

pub fn generate_block_face(
    face: BlockFace,
    (x, y, z): (f32, f32, f32),
    texture: Rect<f32>,
) -> MeshFragment {
    MeshFragment {
        vertices: match face {
            BlockFace::Front => vec![
                Vertex {
                    position: [x - 0.5, y + 0.5, z + 0.5],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [texture.left, texture.top],
                },
                Vertex {
                    position: [x - 0.5, y - 0.5, z + 0.5],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [x + 0.5, y - 0.5, z + 0.5],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [texture.right, texture.bottom],
                },
                Vertex {
                    position: [x + 0.5, y + 0.5, z + 0.5],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [texture.right, texture.top],
                },
            ],
            BlockFace::Back => vec![
                Vertex {
                    position: [x + 0.5, y - 0.5, z - 0.5],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [x - 0.5, y - 0.5, z - 0.5],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [texture.right, texture.bottom],
                },
                Vertex {
                    position: [x - 0.5, y + 0.5, z - 0.5],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [texture.right, texture.top],
                },
                Vertex {
                    position: [x + 0.5, y + 0.5, z - 0.5],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [texture.left, texture.top],
                },
            ],
            BlockFace::Left => vec![
                Vertex {
                    position: [x - 0.5, y + 0.5, z - 0.5],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [texture.left, texture.top],
                },
                Vertex {
                    position: [x - 0.5, y - 0.5, z - 0.5],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [x - 0.5, y - 0.5, z + 0.5],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [texture.right, texture.bottom],
                },
                Vertex {
                    position: [x - 0.5, y + 0.5, z + 0.5],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [texture.right, texture.top],
                },
            ],
            BlockFace::Right => vec![
                Vertex {
                    position: [x + 0.5, y - 0.5, z + 0.5],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [x + 0.5, y - 0.5, z - 0.5],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [texture.right, texture.bottom],
                },
                Vertex {
                    position: [x + 0.5, y + 0.5, z - 0.5],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [texture.right, texture.top],
                },
                Vertex {
                    position: [x + 0.5, y + 0.5, z + 0.5],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [texture.left, texture.top],
                },
            ],
            BlockFace::Top => vec![
                Vertex {
                    position: [x + 0.5, y + 0.5, z - 0.5],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [texture.right, texture.top],
                },
                Vertex {
                    position: [x - 0.5, y + 0.5, z - 0.5],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [texture.left, texture.top],
                },
                Vertex {
                    position: [x - 0.5, y + 0.5, z + 0.5],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [x + 0.5, y + 0.5, z + 0.5],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [texture.right, texture.bottom],
                },
            ],
            BlockFace::Bottom => vec![
                Vertex {
                    position: [x - 0.5, y - 0.5, z + 0.5],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [texture.right, texture.top],
                },
                Vertex {
                    position: [x - 0.5, y - 0.5, z - 0.5],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [texture.right, texture.bottom],
                },
                Vertex {
                    position: [x + 0.5, y - 0.5, z - 0.5],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [texture.left, texture.bottom],
                },
                Vertex {
                    position: [x + 0.5, y - 0.5, z + 0.5],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [texture.left, texture.top],
                },
            ],
        },
        indices: vec![0, 1, 2, 2, 3, 0],
    }
}
