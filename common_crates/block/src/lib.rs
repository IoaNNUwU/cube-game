use serde::{Deserialize, Serialize};

pub mod solid_block;
pub mod liquid_block;
pub mod fancy_block;
pub mod super_fancy_block;

use solid_block::SolidBlock;
use liquid_block::LiquidBlock;
use fancy_block::FancyBlock;
use super_fancy_block::SuperFancyBlock;

#[cfg(feature = "derive-display")]
use strum::Display;

#[cfg_attr(feature = "derive-display", derive(Display))]
#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum BlockState {
    #[default]
    Air,
    Solid(SolidBlock),
    Liquid(LiquidBlock),
    Fancy(FancyBlock),
    SuperFancy(SuperFancyBlock),
}

mod test {
    
}