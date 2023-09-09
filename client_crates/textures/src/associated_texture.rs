use basic::block::BlockType;

pub const BLOCKS_PATH: &str = "blocks";

pub trait AssociatedTexture {
    fn texture_path(&self) -> &str;
}

impl AssociatedTexture for BlockType {
    fn texture_path(&self) -> &str {
        match self {
            BlockType::Air => "blocks/air.png",
            BlockType::Dirt => "blocks/dirt.png",
            BlockType::Stone => "blocks/stone.png",
            BlockType::StoneBricks => "blocks/stone_bricks.png",
            BlockType::PlainsGrass => "blocks/plains_grass.png",
            BlockType::PlainsLog => "blocks/plains_log.png",
            BlockType::PlainsLeaves => "blocks/plains_leaves.png",
        }
    }
}