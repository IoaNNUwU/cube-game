use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[cfg(feature = "derive-display")]
use strum::Display;

#[cfg_attr(feature = "derive-display", derive(Display))]
#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum SolidBlock {
    Stone(CommonBlockAttrs),
    Dirt(CommonBlockAttrs),
    StoneBricks(CommonBlockAttrs),
    UnbreakableStone,
    PlainsGrass(CommonBlockAttrs),
    PlainsLog(CommonBlockAttrs, LogBlockAttrs),
    PlainsLeaves,
}

impl Default for SolidBlock {
    fn default() -> Self {
        SolidBlock::Stone(CommonBlockAttrs::default())
    }
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct CommonBlockAttrs {
    pub transform: Transformation,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct LogBlockAttrs {
    pub direction: LogDirection,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum LogDirection { X, #[default] Y, Z }

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Transformation {
    #[default]
    None,
    Half(HalfBlockTransformation),
    Stairs(StairsTransformation),
    Torch(TorchTransformation),
    Fence(FenceAttrs),
    Wall(WallAttrs),
}

#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum HalfBlockTransformation {
    DownHalfSolid,
    UpperHalfSolid,
    PositiveXHalfSolid,
    NegativeXHalfSolid,
    PositiveZHalfSolid,
    NegativeZHalfSolid,
}

#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum StairsTransformation {
    Normal(StairsDirection),
    Flipped(StairsDirection),
}

#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum StairsDirection {
    EmptyPartTowardsPositiveX,
    EmptyPartTowardsNegativeX,
    EmptyPartTowardsPositiveZ,
    EmptyPartTowardsNegativeZ,

    EmptyBitTowardsPosXPosZ,
    EmptyBitTowardsNegXNegZ,
    EmptyBitTowardsNegXPosZ,
    EmptyBitTowardsPosXNegZ,

    FilledBitTowardsPosXPosZ,
    FilledBitTowardsNegXNegZ,
    FilledBitTowardsNegXPosZ,
    FilledBitTowardsPosXNegZ,
}

#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum TorchTransformation {
    OnNorthWall,
    OnSouthWall,
    OnEastWall,
    OnWestWall,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct FenceAttrs {
    pub positive_x: Connection,
    pub positive_z: Connection,
    pub negative_x: Connection,
    pub negative_z: Connection,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct WallAttrs {
    pub positive_x: Connection,
    pub positive_z: Connection,
    pub negative_x: Connection,
    pub negative_z: Connection,
    pub upper: Connection,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Connection {
    #[default]
    Disconnected,
    Connected,
}