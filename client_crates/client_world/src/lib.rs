use bevy::prelude::*;
use common_world::{Chunk, ChunkPosInWorld};

#[derive(Resource)]
pub struct ClientWorld(pub bevy::utils::HashMap<ChunkPosInWorld, Option<Entity>>);

#[derive(Component)]
pub struct ChunkComponent(pub Chunk);

#[derive(Component)]
pub struct ChunkPosInWorldComponent(pub ChunkPosInWorld);

#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk: ChunkComponent,
    pub chunk_pos_in_world: ChunkPosInWorldComponent,
}