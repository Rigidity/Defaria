use bevy::{prelude::*, utils::HashMap};

use crate::{
    chunk::{Chunk, CHUNK_SIZE},
    game::AssetHandles,
    key::Key,
    player::Player,
    registry::ClientRegistry,
    render::{aggregate_mesh_fragments, build_mesh, MeshFragment},
    utils::world_to_chunk,
};

#[derive(Default)]
pub struct World {
    pub chunks: HashMap<IVec3, Entity>,
    pub max_size: IVec3,
}

#[derive(Default)]
pub struct ChunkGenerator {
    pub radius: u32,
}

pub fn build_chunks(
    mut commands: Commands,
    world: Res<World>,
    client_registry: Res<ClientRegistry<'static>>,
    asset_handles: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunks: Query<(Entity, &mut Chunk<'static>, Option<&Handle<Mesh>>)>,
) {
    let texture_atlas = texture_atlases
        .get(asset_handles.block_texture_atlas.clone())
        .unwrap();

    for (entity, mut chunk, mesh_handle) in chunks.iter_mut() {
        if !chunk.has_changed {
            continue;
        }

        let mut mesh_fragments: Vec<MeshFragment> = vec![];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let i = x * CHUNK_SIZE * CHUNK_SIZE + y * CHUNK_SIZE + z;

                    if let Some(block) = chunk.blocks[i] {
                        let block_model = client_registry
                            .block_models
                            .get(&block)
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
                            &world,
                            IVec3::new(x as i32, y as i32, z as i32),
                            texture,
                        ));
                    }
                }
            }
        }

        let new_mesh = build_mesh(aggregate_mesh_fragments(mesh_fragments));

        if let Some(mesh_handle) = mesh_handle {
            if let Some(mesh) = meshes.get_mut(mesh_handle) {
                *mesh = new_mesh;
            }
        } else {
            commands.entity(entity).insert_bundle(PbrBundle {
                mesh: meshes.add(new_mesh),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(texture_atlas.texture.clone()),
                    ..default()
                }),
                transform: Transform::from_xyz(
                    chunk.position.x as f32 * CHUNK_SIZE as f32,
                    chunk.position.y as f32 * CHUNK_SIZE as f32,
                    chunk.position.z as f32 * CHUNK_SIZE as f32,
                ),
                ..default()
            });
        }

        chunk.has_changed = false;
    }
}

pub fn generate_chunks(
    mut commands: Commands,
    mut world: ResMut<World>,
    chunk_generator: Res<ChunkGenerator>,
    players: Query<&Transform, With<Player>>,
) {
    for player_transform in players.iter() {
        let chunk_position = world_to_chunk(player_transform.translation);

        let radius = chunk_generator.radius as i32 - 1;

        for x in (chunk_position.x - radius)..(chunk_position.x + radius + 1) {
            for y in (chunk_position.y - radius)..(chunk_position.y + radius + 1) {
                for z in (chunk_position.z - radius)..(chunk_position.z + radius + 1) {
                    let position = IVec3::new(x, y, z);

                    if world.chunks.contains_key(&position) {
                        continue;
                    }

                    let distance = (((x - chunk_position.x).pow(2)
                        + (y - chunk_position.y).pow(2)
                        + (z - chunk_position.z).pow(2))
                        as f32)
                        .sqrt();

                    if distance > radius as f32 {
                        continue;
                    }

                    let mut chunk = Chunk {
                        position,
                        blocks: [None; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
                        has_changed: true,
                    };

                    for x in 0..CHUNK_SIZE {
                        for y in 0..CHUNK_SIZE {
                            for z in 0..CHUNK_SIZE {
                                let world_position = IVec3::new(
                                    position.x * CHUNK_SIZE as i32 + x as i32,
                                    position.y * CHUNK_SIZE as i32 + y as i32,
                                    position.z * CHUNK_SIZE as i32 + z as i32,
                                );

                                if world_position.y < 16 {
                                    chunk.blocks[x * 256 + y * 16 + z] = Some(Key {
                                        namespace: "defaria",
                                        name: "arrow",
                                    });
                                }
                            }
                        }
                    }

                    let entity = commands.spawn().insert(chunk).id();

                    world.chunks.insert(position, entity);
                }
            }
        }
    }
}

pub fn create_world(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.7,
    });
}
