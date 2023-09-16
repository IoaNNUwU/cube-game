use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use strum::EnumIter;

#[cfg(feature = "derive-display")]
use strum::Display;

#[cfg_attr(feature = "derive-display", derive(Display))]
#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum SuperFancyBlock {
    Chest(Box<ChestAttr>),
    CraftingTable(Box<CraftingTableAttr>),
}

#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ChestAttr {
    pub inventory: ItemStorage<18, 6>,
}

impl Default for ChestAttr {
    fn default() -> Self {
        Self {
            inventory: ItemStorage {
                items: std::array::from_fn(|_| std::array::from_fn(|_| Item)),
            },
        }
    }
}

#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct ItemStorage<const COLUMNS: usize, const ROWS: usize> {
    #[serde_as(as = "[[_; COLUMNS]; ROWS]")]
    items: [[Item; COLUMNS]; ROWS],
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Item;

#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct CraftingTableAttr {
    pub inv: ItemStorage<5, 5>,
}

impl Default for CraftingTableAttr {
    fn default() -> Self {
        Self {
            inv: ItemStorage {
                items: std::array::from_fn(|_| std::array::from_fn(|_| Item)),
            },
        }
    }
}