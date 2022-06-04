use crate::key::Key;
use bevy::prelude::*;

pub const CHUNK_SIZE: usize = 16;

#[derive(Component)]
pub struct Chunk<'a> {
    pub position: IVec3,
    pub blocks: [Option<Key<'a>>; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
    pub has_changed: bool,
}

pub struct Block {
    pub solid: bool,
}
