use bevy::prelude::*;

use crate::{Camera3d, MouseMotion};

#[derive(Component, Default)]
pub struct Player {
    pub pitch: f32,
    pub yaw: f32,
}

pub fn grab_mouse(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    lock_mouse(window);
}

pub fn create_player(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Player { ..default() });
}

pub fn manage_mouse(
    mut windows: ResMut<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if keyboard_input.just_pressed(KeyCode::Escape) {
        unlock_mouse(window);
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        lock_mouse(window);
    }
}

pub fn rotate_camera(
    mut windows: ResMut<Windows>,
    mut camera: Query<(&mut Player, &mut Transform), With<Camera3d>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let speed = 0.06;

    let window = windows.get_primary_mut().unwrap();

    for (mut player, mut camera_transform) in camera.iter_mut() {
        for event in mouse_motion.iter() {
            if window.cursor_locked() {
                player.pitch -= (speed * event.delta.y).to_radians();
                player.yaw -= (speed * event.delta.x).to_radians();
            }
        }

        player.pitch = player
            .pitch
            .clamp(-90.0f32.to_radians(), 90.0f32.to_radians());

        let y_rotation = Quat::from_axis_angle(Vec3::Y, player.yaw);
        let x_rotation = Quat::from_axis_angle(Vec3::X, player.pitch);

        camera_transform.rotation = y_rotation * x_rotation;
    }
}

pub fn move_camera(
    time: ResMut<Time>,
    windows: Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
) {
    let speed = 6.0 * time.delta_seconds();

    let window = windows.get_primary().unwrap();

    for mut camera_transform in camera.iter_mut() {
        let mut velocity = Vec3::ZERO;

        let local_z = camera_transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let strafe = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keyboard_input.get_pressed() {
            if window.cursor_locked() {
                match key {
                    KeyCode::W => velocity += forward,
                    KeyCode::S => velocity -= forward,
                    KeyCode::A => velocity -= strafe,
                    KeyCode::D => velocity += strafe,
                    KeyCode::Space => velocity += Vec3::Y,
                    KeyCode::LShift => velocity -= Vec3::Y,
                    _ => {}
                }
            }
        }

        velocity = velocity.normalize_or_zero();

        camera_transform.translation += velocity * speed;
    }
}

fn lock_mouse(window: &mut Window) {
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);

    window.set_cursor_position(Vec2::new(window.width() / 2.0, window.height() / 2.0));
}

fn unlock_mouse(window: &mut Window) {
    window.set_cursor_visibility(true);
    window.set_cursor_lock_mode(false);
}
