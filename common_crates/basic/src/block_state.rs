pub mod transformation;

use strum::EnumIter;

use transformation::Transformation;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum SolidBlock {
    #[default]
    Stone,
    Dirt,
    StoneBricks,
    PlainsGrass,
    PlainsLog,
    PlainsLeaves,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum LiquidBlock {
    #[default]
    Water,
    Lava,
    Honey,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum FancyBlockType {
    #[default]
    Torch,
    Fence,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct BlockState {
    pub block_type: BlockType,
    pub transformation: Transformation,
    pub data: Option<BlockData>,
}

impl BlockState {
    pub const EMPTY: Self = BlockState::with_type(BlockType::Air);

    pub const fn with_type(block_type: BlockType) -> Self {
        BlockState {
            block_type,
            transformation: Transformation::None,
            data: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        matches!(self.block_type, BlockType::Air)
    }
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct BlockData(Box<()>);

mod test {
    use crate::block::{BlockState, BlockType};

    #[test]
    fn block_state_emptiness_test() {
        let air_state = BlockState {
            block_type: Default::default(),
            transformation: Default::default(),
            data: None,
        };
        let full_state = BlockState {
            block_type: BlockType::Stone,
            transformation: Default::default(),
            data: None,
        };

        assert!(air_state.is_empty());
        assert!(BlockState::EMPTY.is_empty());

        assert!(!full_state.is_empty());
    }
}
