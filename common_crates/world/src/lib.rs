use std::collections::hash_map::Entry;
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
use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};

pub const WORLD_WIDTH_CHUNKS: usize = 3_000_000;
pub const WORLD_WIDTH_BLOCKS: usize = WORLD_WIDTH_CHUNKS * CHUNK_WIDTH;

#[derive(Default, Clone, Debug)]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct World(HashMap<ChunkPosInWorld, Entity>);

impl Index<ChunkPosInWorld> for World {
    type Output = ColumnOfChunks;

    fn index(&self, index: ChunkPosInWorld) -> &Self::Output {
        &self.0[&index]
    }
}

impl IndexMut<ChunkPosInWorld> for World {
    fn index_mut(&mut self, index: ChunkPosInWorld) -> &mut Self::Output {
        self.0.entry(index).or_default()
    }
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ChunkPosInWorld {
    x: u32,
    y: u32,
}