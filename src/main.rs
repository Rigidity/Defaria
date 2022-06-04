extern crate core;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::Camera3d;
use game::run_ingame;
use registry::ClientRegistry;
use world::{build_chunks, generate_chunks};

use crate::game::{
    check_assets, load_assets, load_block_models, load_blocks, AssetHandles, GameState,
};
use crate::player::{create_player, grab_mouse, manage_mouse, move_camera, rotate_camera};
use crate::registry::Registry;
use crate::world::create_world;

mod chunk;
mod game;
mod key;
mod player;
mod registry;
mod render;
mod utils;
mod world;

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
                .with_system(grab_mouse)
                .with_system(create_player)
                .with_system(create_world),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_ingame)
                .with_system(manage_mouse)
                .with_system(rotate_camera)
                .with_system(move_camera)
                .with_system(generate_chunks)
                .with_system(build_chunks),
        )
        .run();
}
