use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[cfg(feature = "derive-display")]
use strum::Display;

#[cfg_attr(feature = "derive-display", derive(Display))]
#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum SuperFancyBlockItem {
    Chest,
    CraftingTable,
}
