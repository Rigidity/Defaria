use crate::key::Key;
use bevy::prelude::*;

#[derive(Component)]
pub struct Chunk<'a> {
    pub blocks: [Key<'a>; 4096],
    pub has_changed: bool,
}

pub struct Block {
    pub solid: bool,
}
