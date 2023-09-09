use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum Transformation {
    #[default]
    None,
    Half(HalfBlockTransformation),
    Stairs(StairsTransformation),
    Torch(TorchTransformation),
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum HalfBlockTransformation {
    #[default]
    DownHalfSolid,
    UpperHalfSolid,
    NorthHalfSolid,
    SouthHalfSolid,
    EastHalfSolid,
    WestHalfSolid,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum StairsTransformation {
    #[default]
    LowerNorth,
    LowerSouth,
    LowerEast,
    LowerWest,
    FlippedLowerNorth,
    FlippedLowerSouth,
    FlippedLowerEast,
    FlippedLowerWest,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum TorchTransformation {
    #[default]
    OnNorthWall,
    OnSouthWall,
    OnEastWall,
    OnWestWall,
}
