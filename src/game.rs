use crate::chunk::Block;
use crate::key::Key;
use crate::render::{generate_block, BlockModel};
use crate::world::{ChunkGenerator, World};
use crate::{ClientRegistry, Registry};
use bevy::asset::LoadState;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    Ingame,
}

pub fn run_ingame(game_state: Res<State<GameState>>) -> ShouldRun {
    if *game_state.current() == GameState::Ingame {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[derive(Default)]
pub struct AssetHandles {
    block_textures: Vec<HandleUntyped>,
    pub block_texture_atlas: Handle<TextureAtlas>,
}

pub fn load_blocks(mut registry: ResMut<Registry<'static>>) {
    registry.blocks.insert(
        Key {
            namespace: "defaria",
            name: "arrow",
        },
        Block { solid: true },
    );
}

pub fn load_block_models(mut client_registry: ResMut<ClientRegistry<'static>>) {
    let mesh_generator = |world: &World, position: IVec3, texture: Rect<f32>| {
        generate_block(
            position,
            [
                Some(texture),
                Some(texture),
                Some(texture),
                Some(texture),
                Some(texture),
                Some(texture),
            ],
        )
    };

    client_registry.block_models.insert(
        Key {
            namespace: "defaria",
            name: "arrow",
        },
        BlockModel {
            texture: "blocks/arrow.png",
            generate_mesh: mesh_generator,
        },
    );
}

pub fn load_assets(mut asset_handles: ResMut<AssetHandles>, asset_server: Res<AssetServer>) {
    asset_handles.block_textures = asset_server
        .load_folder("blocks")
        .expect("Could not find block textures folder.");
}

pub fn check_assets(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut asset_handles: ResMut<AssetHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    if let LoadState::Loaded = asset_server
        .get_group_load_state(asset_handles.block_textures.iter().map(|handle| handle.id))
    {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        for handle in &asset_handles.block_textures {
            let handle = handle.clone().typed();
            let texture = textures
                .get(&handle)
                .expect("Atlas texture is not an image.");
            texture_atlas_builder.add_texture(handle, texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        asset_handles.block_texture_atlas = texture_atlases.add(texture_atlas.clone());

        commands.insert_resource(World::default());
        commands.insert_resource(ChunkGenerator { radius: 4 });

        game_state.set(GameState::Ingame).unwrap();
    }
}
