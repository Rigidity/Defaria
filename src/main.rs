extern crate core;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::Camera3d;
use registry::ClientRegistry;

use crate::game::{
    check_assets, load_assets, load_block_models, load_blocks, setup_assets, AssetHandles,
    GameState,
};
use crate::player::{create_player, grab_mouse, manage_mouse, move_camera, rotate_camera};
use crate::registry::Registry;
use crate::render::create_world;

mod chunk;
mod game;
mod key;
mod player;
mod registry;
mod render;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.8, 0.9)))
        .insert_resource(WindowDescriptor {
            width: 900.0,
            height: 600.0,
            ..default()
        })
        .insert_resource(AssetHandles::default())
        .insert_resource(Registry::default())
        .insert_resource(ClientRegistry::default())
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Loading)
        .add_system_set(
            SystemSet::on_enter(GameState::Loading)
                .with_system(load_assets)
                .with_system(load_blocks)
                .with_system(load_block_models),
        )
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_assets))
        .add_system_set(
            SystemSet::on_enter(GameState::Ingame)
                .with_system(setup_assets)
                .with_system(grab_mouse.after(setup_assets))
                .with_system(create_player.after(setup_assets))
                .with_system(create_world.after(setup_assets)),
        )
        .add_system(manage_mouse)
        .add_system(rotate_camera)
        .add_system(move_camera.after(setup_assets))
        .run();
}
