pub use block;

mod replace;
pub use replace::*;

mod weather;
pub use weather::*;

pub use chunk::*;
pub use chunk::biome::*;

use serde::{Deserialize, Serialize};

pub const WORLD_HEIGHT_CHUNKS: usize = 32;
pub const WORLD_HEIGHT_BLOCKS: usize = WORLD_HEIGHT_CHUNKS * CHUNK_HEIGHT;

pub const WORLD_WIDTH_CHUNKS: usize = 3_000_000;
pub const WORLD_WIDTH_BLOCKS: usize = WORLD_WIDTH_CHUNKS * CHUNK_WIDTH;

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ChunkPosInWorld {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl From<(u32, u32, u32)> for ChunkPosInWorld {
    fn from(value: (u32, u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<ChunkPosInWorld> for (u32, u32, u32) {
    fn from(value: ChunkPosInWorld) -> Self {
        (value.x, value.y, value.z)
    }
}