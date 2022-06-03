use crate::chunk::Block;
use crate::key::Key;
use crate::render::{generate_block, BlockModel};
use crate::{ClientRegistry, Registry};
use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    Ingame,
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
    client_registry.block_models.insert(
        Key {
            namespace: "defaria",
            name: "arrow",
        },
        BlockModel {
            texture: "blocks/arrow.png",
            generate_mesh: |position, texture| {
                generate_block(
                    position,
                    [texture, texture, texture, texture, texture, texture],
                )
            },
        },
    );
}

pub fn load_assets(mut asset_handles: ResMut<AssetHandles>, asset_server: Res<AssetServer>) {
    asset_handles.block_textures = asset_server
        .load_folder("blocks")
        .expect("Could not find block textures folder.");
}

pub fn check_assets(
    mut game_state: ResMut<State<GameState>>,
    asset_handles: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded = asset_server
        .get_group_load_state(asset_handles.block_textures.iter().map(|handle| handle.id))
    {
        game_state.set(GameState::Ingame).unwrap();
    }
}

pub fn setup_assets(
    mut asset_handles: ResMut<AssetHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
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
}
