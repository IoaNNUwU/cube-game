#![allow(dead_code)]

pub mod transformation;

use transformation::Transformation;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum BlockType {
    #[default]
    Air,
    Dirt,
    GrassBlock,
    Stone,
    StoneBricks,
    Log,
    Leaves,
}

#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct BlockState {
    pub block_type: BlockType,
    pub transformation: Transformation,
    pub data: Option<BlockData>,
}

#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct BlockData(Box<()>);
