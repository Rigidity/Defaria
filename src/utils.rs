use bevy::prelude::*;

use crate::chunk::CHUNK_SIZE;

pub fn world_to_chunk(position: Vec3) -> IVec3 {
    IVec3::new(
        (position.x / CHUNK_SIZE as f32).floor() as i32,
        (position.y / CHUNK_SIZE as f32).floor() as i32,
        (position.z / CHUNK_SIZE as f32).floor() as i32,
    )
}
