use serde::{Deserialize, Serialize};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Clone, Hash, Debug)]
#[derive(Serialize, Deserialize)]
pub struct WorldBlockPosition {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl WorldBlockPosition {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
}

#[derive(PartialOrd, PartialEq)]
#[derive(Default, Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WorldPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
