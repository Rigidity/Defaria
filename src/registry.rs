use crate::{chunk::Block, key::Key, render::BlockModel};
use bevy::utils::HashMap;

#[derive(Default)]
pub struct Registry<'a> {
    pub blocks: HashMap<Key<'a>, Block>,
}

#[derive(Default)]
pub struct ClientRegistry<'a> {
    pub block_models: HashMap<Key<'a>, BlockModel<'a>>,
}
