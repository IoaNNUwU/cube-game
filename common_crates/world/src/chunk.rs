use serde::{Deserialize, Serialize};
use basic::block::BlockState;
use biome::Biome;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;

pub const WORLD_HEIGHT_CHUNKS: usize = 32;
pub const WORLD_HEIGHT_BLOCKS: usize = WORLD_HEIGHT_CHUNKS * CHUNK_HEIGHT;

pub mod biome;

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub biome: Biome,
    pub blocks: [[[BlockState; CHUNK_WIDTH]; CHUNK_WIDTH]; CHUNK_HEIGHT],
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ChunkPos2 {
    x: u32,
    y: u32,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ChunkPos3 {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct WorldChunk {
    pub chunks: [Chunk; WORLD_HEIGHT_CHUNKS]
}
