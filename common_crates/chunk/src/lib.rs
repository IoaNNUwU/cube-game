include!(concat!(env!("OUT_DIR"), "/empty_chunk_macro_generated.rs"));

pub mod biome;

mod layer;

pub use layer::*;

use block::BlockState;

use std::ops::{Index, IndexMut};
use serde::{Serialize, Deserialize};

use biome::Biome;

pub const CHUNK_HEIGHT: usize = 32;

/// [Index 0](Index) gives lowest layer in chunk
/// ```rust
/// use chunk::{Chunk, ChunkLayer};
/// use block::BlockState;
///
/// let chunk = Chunk::default();
/// let lowest_layer: &ChunkLayer = &chunk[0];
/// let lowest_block_with_min_xz: &BlockState = &chunk[0][(0,0)];
/// ```
#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub biome: Biome,
    pub layers: [ChunkLayer; CHUNK_HEIGHT],
}

impl Chunk {
    pub const EMPTY: Self = Self {
        biome: Biome::Void,
        layers: empty_chunk!(),
    };

    pub fn of(block_state: BlockState) -> Self {
        Self::from_fn(|_| block_state.clone())
    }

    pub fn from_fn<F: FnMut(PosInChunk) -> BlockState>(mut pos_in_chunk_to_block_state_fn: F) -> Self {
        Self {
            biome: Default::default(),
            layers: std::array::from_fn(|y| {
                ChunkLayer::from_fn(|pos_in_layer| {
                    pos_in_chunk_to_block_state_fn(PosInChunk::from_pos_in_layer(pos_in_layer, y as u8))
                })
            }),
        }
    }

    pub fn from_fn_layer<F: FnMut(usize) -> ChunkLayer>(mut layer_number_to_layer_fn: F) -> Self {
        Self {
            biome: Default::default(),
            layers: std::array::from_fn(|y| {
                layer_number_to_layer_fn(y)
            }),
        }
    }

    pub fn block_at(&self, chunk_x: usize, chunk_y: usize, chunk_z: usize) -> &BlockState {
        &self.layers[chunk_y][(chunk_x, chunk_z)]
    }

    pub fn block_at_mut(&mut self, chunk_x: usize, chunk_y: usize, chunk_z: usize) -> &mut BlockState {
        &mut self.layers[chunk_y][(chunk_x, chunk_z)]
    }
}

impl Index<PosInChunk> for Chunk {
    type Output = BlockState;

    fn index(&self, pos_in_chunk: PosInChunk) -> &Self::Output {
        &self.layers[pos_in_chunk.y as usize][pos_in_chunk.as_pos_in_layer()]
    }
}

impl IndexMut<PosInChunk> for Chunk {
    fn index_mut(&mut self, pos_in_chunk: PosInChunk) -> &mut Self::Output {
        &mut self.layers[pos_in_chunk.y as usize][pos_in_chunk.as_pos_in_layer()]
    }
}

impl Index<usize> for Chunk {
    type Output = ChunkLayer;

    fn index(&self, index: usize) -> &Self::Output {
        &self.layers[index]
    }
}

impl IndexMut<usize> for Chunk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.layers[index]
    }
}

/// [CHUNK_WIDTH] are in u8 range
/// [CHUNK_HEIGHT] are in u8 range
#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct PosInChunk {
    pub x: u8,
    pub y: u8,
    // TODO check performance of XZY vs XYZ
    pub z: u8,
}

impl PosInChunk {
    /// Unchecked variant of [PosInChunk::new_checked]
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }

    /// Returns None if x or z > [CHUNK_WIDTH]
    /// Returns None if y > [CHUNK_HEIGHT]
    pub fn new_checked(x: u8, y: u8, z: u8) -> Option<Self> {
        if (x as usize <= CHUNK_WIDTH) && (z as usize <= CHUNK_WIDTH) {
            if y as usize <= CHUNK_HEIGHT {
                Some(Self::new(x, y, z))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn from_pos_in_layer(pos_in_layer: PosInLayer, y: u8) -> Self {
        Self {
            x: pos_in_layer.x,
            y,
            z: pos_in_layer.z,
        }
    }

    pub fn as_pos_in_layer(&self) -> PosInLayer {
        PosInLayer::new(self.x, self.z)
    }
}

#[cfg(test)]
mod test {
    use block::BlockState;
    use block::solid_block::CommonBlockAttrs;
    use block::solid_block::SolidBlock;
    use crate::{PosInChunk, Chunk};

    #[test]
    fn chunk_from_fn_test() {
        let chunk = Chunk::from_fn(|PosInChunk { x, y, z }| {
            if x == y && x == z {
                BlockState::Solid(SolidBlock::Stone(CommonBlockAttrs::default()))
            } else {
                BlockState::Air
            }
        });

        assert_eq!(chunk[0][(0, 0)], BlockState::Solid(SolidBlock::Stone(CommonBlockAttrs::default())));
        assert_eq!(chunk[1][(1, 1)], BlockState::Solid(SolidBlock::Stone(CommonBlockAttrs::default())));

        assert_eq!(chunk[1][(1, 2)], BlockState::Air);
    }
}