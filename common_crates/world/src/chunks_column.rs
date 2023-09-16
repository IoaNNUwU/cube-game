use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};
use block::BlockState;

use chunk::Chunk;
use crate::chunk::PosInChunk;
use crate::CHUNK_HEIGHT;

pub const WORLD_HEIGHT_CHUNKS: usize = 32;
pub const WORLD_HEIGHT_BLOCKS: usize = WORLD_HEIGHT_CHUNKS * CHUNK_HEIGHT;

pub(crate) mod chunk;

/// [Index 0](Index) gives lowest chunk in column
/// ```rust
/// use world::ColumnOfChunks;
///
/// let world_chunk = ColumnOfChunks::default();
/// let lowest_chunk = &world_chunk[0];
/// ```
#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ColumnOfChunks {
    chunks: [Chunk; WORLD_HEIGHT_CHUNKS],
}

impl ColumnOfChunks {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_fn<F: FnMut(PosInColumnOfChunks) -> BlockState>(mut pos_in_chunk_column_to_block_state_fn: F) -> Self {
        Self {
            chunks: std::array::from_fn(|chunk_number| {
                Chunk::from_fn(|pos_in_chunk| {
                    pos_in_chunk_column_to_block_state_fn(
                        PosInColumnOfChunks::from_pos_in_chunk(pos_in_chunk, chunk_number)
                    )
                })
            }),
        }
    }

    pub fn from_fn_chunk<F: FnMut(usize) -> Chunk>(mut chunk_number_to_chunk_fn: F) -> Self {
        Self {
            chunks: std::array::from_fn(|chunk_number| {
                chunk_number_to_chunk_fn(chunk_number)
            }),
        }
    }
}

impl Index<usize> for ColumnOfChunks {
    type Output = Chunk;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chunks[index]
    }
}

impl IndexMut<usize> for ColumnOfChunks {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.chunks[index]
    }
}

impl Index<PosInColumnOfChunks> for ColumnOfChunks {
    type Output = BlockState;

    fn index(&self, pos_in_column: PosInColumnOfChunks) -> &Self::Output {
        &self.chunks[pos_in_column.chunk_number()][pos_in_column.as_pos_in_chunk()]
    }
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct PosInColumnOfChunks {
    pub x: u8,
    pub y: u16,
    pub z: u8,
}

impl PosInColumnOfChunks {
    pub fn from_pos_in_chunk(pos_in_chunk: PosInChunk, chunk_number: usize) -> Self {
        Self {
            x: pos_in_chunk.x,
            y: pos_in_chunk.y as u16 + (CHUNK_HEIGHT * chunk_number) as u16,
            z: pos_in_chunk.z,
        }
    }

    pub fn chunk_number(&self) -> usize {
        (self.y / 16) as usize
    }

    pub fn as_pos_in_chunk(&self) -> PosInChunk {
        PosInChunk::new(self.x, (self.y % 16) as u8, self.z)
    }
}

mod test {
    use block::BlockState;
    use block::solid_block::{CommonBlockAttrs, SolidBlock};
    use crate::ColumnOfChunks;
    use crate::replace::Replace;

    #[test]
    fn chunk_column_test() {
        let mut chunks_column = ColumnOfChunks::default();
        let lowest_block_with_lowest_xz = &mut chunks_column[0][0][(0, 0)];

        assert_eq!(lowest_block_with_lowest_xz, &BlockState::Air);

        lowest_block_with_lowest_xz.replace(BlockState::Solid(SolidBlock::Stone(CommonBlockAttrs::default())));

        assert_eq!(lowest_block_with_lowest_xz, &BlockState::Solid(SolidBlock::Stone(CommonBlockAttrs::default())));
    }
}