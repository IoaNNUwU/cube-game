use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[cfg(feature = "derive-display")]
use strum::Display;
use crate::MaxCount;

#[cfg_attr(feature = "derive-display", derive(Display))]
#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum LiquidBlockItem {
    #[default]
    Water,
    Lava,
    Honey,
}

impl MaxCount for LiquidBlockItem {
    fn max_count(&self) -> u16 {
        1
    }
}