pub mod transformation;

use transformation::Transformation;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum BlockType {
    #[default]
    Air,
    Dirt,
    Stone,
    StoneBricks,
    PlainsGrass,
    PlainsLog,
    PlainsLeaves,
}

pub const BLOCK_TYPES: [BlockType; 7] = [
    BlockType::Air,
    BlockType::Dirt,
    BlockType::Stone,
    BlockType::StoneBricks,
    BlockType::PlainsGrass,
    BlockType::PlainsLog,
    BlockType::PlainsLeaves,
];

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct BlockState {
    pub block_type: BlockType,
    pub transformation: Transformation,
    pub data: Option<BlockData>,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct BlockData(Box<()>);
