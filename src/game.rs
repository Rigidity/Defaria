use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::text::TextPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    Ingame,
}

#[derive(Default)]
pub struct AssetHandles {
    block_textures: Vec<HandleUntyped>,
}

pub fn load_assets(mut asset_handles: ResMut<AssetHandles>, asset_server: Res<AssetServer>) {
    asset_handles.block_textures = asset_server.load_folder("blocks").expect("Could not find block textures folder.");
}

pub fn check_assets(
    mut game_state: ResMut<State<GameState>>,
    asset_handles: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(
        asset_handles.block_textures
            .iter()
            .map(|handle| handle.id)
    ) {
        game_state.set(GameState::Ingame).unwrap();
    }
}

pub fn setup_assets(
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &asset_handles.block_textures {
        let handle = handle.typed_weak();
        let texture = textures.get(&handle).expect("Atlas texture is not an image.");
        texture_atlas_builder.add_texture(handle, texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let texture_atlas_texture = texture_atlas.texture.clone();
    let vendor_handle = asset_server.get_handle("blocks/dirt.png");
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_atlas_texture),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        ..default()
    });*/
}
