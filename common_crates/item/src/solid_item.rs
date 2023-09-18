use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[cfg(feature = "derive-display")]
use strum::Display;

use crate::MaxCount;

#[cfg_attr(feature = "derive-display", derive(Display))]
#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum SolidBlockItem {
    #[default]
    Stone,
    Dirt,
    StoneBricks,
    UnbreakableStone,
    PlainsGrass,
    PlainsLog,
    PlainsLeaves,
}

impl MaxCount for SolidBlockItem {
    fn max_count(&self) -> u16 {
        9_999
    }
}