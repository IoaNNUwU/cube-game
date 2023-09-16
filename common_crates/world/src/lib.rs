#![feature(inline_const)]

pub use chunk::CHUNK_WIDTH;
pub use chunk::CHUNK_HEIGHT;

pub use chunks_column::WORLD_HEIGHT_CHUNKS;
pub use chunks_column::WORLD_HEIGHT_BLOCKS;

mod chunks_column;
pub use chunks_column::ColumnOfChunks;

pub use block;

pub mod chunk {
    pub use super::chunks_column::chunk::*;
}

pub mod replace;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub const WORLD_WIDTH_CHUNKS: usize = 3_000_000;
pub const WORLD_WIDTH_BLOCKS: usize = WORLD_WIDTH_CHUNKS * CHUNK_WIDTH;

pub struct World {
    pub map: HashMap<ChunkPos2, ColumnOfChunks>
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
