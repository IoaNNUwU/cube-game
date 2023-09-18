#![feature(inline_const)]

pub use chunk::CHUNK_WIDTH;
pub use chunk::CHUNK_HEIGHT;

pub use chunks_column::WORLD_HEIGHT_CHUNKS;
pub use chunks_column::WORLD_HEIGHT_BLOCKS;

mod chunks_column;

pub use chunks_column::ColumnOfChunks;
pub use chunks_column::weather::Weather;

pub use block;

pub mod chunk {
    pub use super::chunks_column::chunk::*;
}

pub mod replace;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub const WORLD_WIDTH_CHUNKS: usize = 3_000_000;
pub const WORLD_WIDTH_BLOCKS: usize = WORLD_WIDTH_CHUNKS * CHUNK_WIDTH;

#[derive(Default, Clone, Debug)]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct World {
    pub map: HashMap<ChunkPosInWorld, ColumnOfChunks>
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ChunkPosInWorld {
    x: u32,
    y: u32,
}