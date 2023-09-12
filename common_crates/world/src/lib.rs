use std::collections::HashMap;
use crate::chunk::{ChunkPos2, WorldChunk};

pub mod chunk;

pub const WORLD_WIDTH_CHUNKS: usize = 3_000_000;
pub const WORLD_WIDTH_BLOCKS: usize = WORLD_WIDTH_CHUNKS * chunk::CHUNK_WIDTH;

pub struct World {
    pub map: HashMap<ChunkPos2, WorldChunk>
}
