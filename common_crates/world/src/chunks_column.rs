use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};
use basic::block::{BlockState, BlockType};
use biome::Biome;

pub const CHUNK_HEIGHT: usize = 32;

pub const WORLD_HEIGHT_CHUNKS: usize = 32;
pub const WORLD_HEIGHT_BLOCKS_U8: usize = WORLD_HEIGHT_CHUNKS * CHUNK_HEIGHT;

pub mod biome;
pub mod cords;

mod layer;

pub use layer::*;

/// [Index 0](Index) gives lowest layer in world
/// ```rust
/// use world::world::Chunk;
///
/// let world = Chunk::default();
/// let lowest_layer = &world[0];
/// let lowest_block_with_min_xz = &world[0][0][0];
/// ```
#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub biome: Biome,
    pub blocks: [ChunkLayer; CHUNK_HEIGHT],
}

impl Chunk {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn of_state(block_state: BlockState) -> Self {
        Self {
            biome: Default::default(),
            blocks: std::array::from_fn(|_| ChunkLayer::of_state(block_state)),
        }
    }
}

impl Index<usize> for Chunk {
    type Output = ChunkLayer;

    fn index(&self, index: usize) -> &Self::Output {
        &self.blocks[index]
    }
}

impl IndexMut<usize> for Chunk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.blocks[index]
    }
}

/// [Index 0](Index) gives lowest world in column
/// ```rust
/// use world::world::ChunksColumn;
///
/// let world_chunk = ChunksColumn::default();
/// let lowest_chunk = &world_chunk[0];
/// ```
#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ChunksColumn {
    chunks: [Chunk; WORLD_HEIGHT_CHUNKS],
}

impl Index<usize> for ChunksColumn {
    type Output = Chunk;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chunks[index]
    }
}

impl IndexMut<usize> for ChunksColumn {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.chunks[index]
    }
}

mod test {
    use basic::block::{BlockState, BlockType};
    use crate::world::ChunkLayer;

    #[test]
    fn chunk_layer_test() {
        let layer = ChunkLayer::from_fn(|pos_in_layer| {
            if pos_in_layer.x % 2 == 0 {
                BlockState::with_type(BlockType::Stone)
            } else {
                BlockState::EMPTY
            }
        });

        assert_eq!(layer[(0, 0)], BlockState::with_type(BlockType::Stone));
        assert_eq!(layer[(1, 0)], BlockState::with_type(BlockType::Stone));
    }
}