mod player;
mod game;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::Camera3d;
use crate::game::{AssetHandles, check_assets, GameState, load_assets, setup_assets};
use crate::player::{create_player, grab_mouse, manage_mouse, move_camera, rotate_camera};

#[derive(Component)]
struct Chunk {
    blocks: [Vec<Block>; 4096],
}

struct Block;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.8, 0.9)))
        .insert_resource(WindowDescriptor {
            width: 900.0,
            height: 600.0,
            ..default()
        })
        .insert_resource(AssetHandles::default())
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Loading)
        .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_assets))
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_assets))
        .add_system_set(SystemSet::on_enter(GameState::Ingame)
            .with_system(setup_assets)
            .with_system(grab_mouse.after(setup_assets))
            .with_system(create_player.after(setup_assets))
            .with_system(create_world.after(setup_assets))
        )
        .add_system(manage_mouse)
        .add_system(rotate_camera)
        .add_system(move_camera.after(setup_assets))
        .run();
}

pub fn create_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        ..default()
    });
}