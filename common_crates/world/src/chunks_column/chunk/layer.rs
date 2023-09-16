use block::BlockState;

use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};

pub const CHUNK_WIDTH: usize = 32;

/// # Layer of the chunk
///
/// [Index (0, 0)](Index) returns block state with lowest xz in that chunks_column
/// ```rust
/// use block::BlockState;
/// use world::chunk::ChunkLayer;
///
/// let layer = ChunkLayer::default();
/// let block_at_lowest_xz = &layer[(0, 0)];
/// ```
#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ChunkLayer {
    layer: [[BlockState; CHUNK_WIDTH]; CHUNK_WIDTH],
}

impl ChunkLayer {
    pub const EMPTY: Self = todo!();

    pub const fn new(layer: [[BlockState; CHUNK_WIDTH]; CHUNK_WIDTH]) -> Self {
        Self { layer }
    }

    pub fn of(block_state: BlockState) -> Self {
        Self::from_fn(|_| {
            block_state.clone()
        })
    }

    pub fn from_fn<F: FnMut(PosInLayer) -> BlockState>(mut pos_in_layer_to_block_state_fn: F) -> Self {
        Self {
            layer: std::array::from_fn(|x| {
                std::array::from_fn(|z| {
                    pos_in_layer_to_block_state_fn(PosInLayer::new(x as u8, z as u8))
                })
            }),
        }
    }
}

impl Index<PosInLayer> for ChunkLayer {
    type Output = BlockState;

    fn index(&self, pos_in_layer: PosInLayer) -> &Self::Output {
        self.index((pos_in_layer.x as usize, pos_in_layer.z as usize))
    }
}

impl IndexMut<PosInLayer> for ChunkLayer {
    fn index_mut(&mut self, pos_in_layer: PosInLayer) -> &mut Self::Output {
        self.index_mut((pos_in_layer.x as usize, pos_in_layer.z as usize))
    }
}

impl Index<(usize, usize)> for ChunkLayer {
    type Output = BlockState;

    fn index(&self, xz: (usize, usize)) -> &Self::Output {
        &self.layer[xz.0][xz.1]
    }
}

impl IndexMut<(usize, usize)> for ChunkLayer {
    fn index_mut(&mut self, xz: (usize, usize)) -> &mut Self::Output {
        &mut self.layer[xz.0][xz.1]
    }
}

/// [CHUNK_WIDTH] are in u8 range
#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct PosInLayer {
    pub x: u8,
    pub z: u8,
}

impl PosInLayer {
    /// Unchecked variant of [PosInLayer::new_checked]
    pub fn new(x: u8, z: u8) -> Self {
        Self { x, z }
    }

    /// Returns None if x or z <= [CHUNK_WIDTH]
    pub fn new_checked(x: u8, z: u8) -> Option<Self> {
        if (x as usize <= CHUNK_WIDTH) && (z as usize <= CHUNK_WIDTH) {
            Some(Self::new(x, z))
        } else {
            None
        }
    }
}

mod test {
    use block::BlockState;
    use block::solid_block::{CommonBlockAttrs, SolidBlock};
    use super::ChunkLayer;

    #[test]
    fn chunk_layer_from_fn_test() {
        let layer = ChunkLayer::from_fn(|pos_in_layer| {
            if pos_in_layer.x % 2 == 0 {
                BlockState::Solid(SolidBlock::Stone(CommonBlockAttrs::default()))
            } else {
                BlockState::Air
            }
        });

        assert_eq!(layer[(0, 0)], BlockState::Solid(SolidBlock::Stone(CommonBlockAttrs::default())));
        assert_eq!(layer[(1, 0)], BlockState::Air);
    }
}