use serde::{Deserialize, Serialize};

pub mod solid_item;
pub mod liquid_item;
pub mod fancy_item;
pub mod super_fancy_item;

use solid_item::SolidBlockItem;
use fancy_item::FancyBlockItem;
use liquid_item::LiquidBlockItem;
use super_fancy_item::SuperFancyBlockItem;

#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ItemStack {
    pub item: Item,
    pub count: u16,
}

#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Item {
    None,
    Block(SolidBlockItem),
    Liquid(LiquidBlockItem),
    Fancy(FancyBlockItem),
    SuperFancy(SuperFancyBlockItem),
    Instrument(SwordAttrs),
}

impl Default for Item {
    fn default() -> Self {
        Self::Block(SolidBlockItem::default())
    }
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct SwordAttrs {
    pub max_durability: Durability,
    pub durability: Durability,

}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Durability(pub u32);