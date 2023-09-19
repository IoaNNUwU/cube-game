use serde::{Deserialize, Serialize};

pub mod solid_item;
pub mod liquid_item;
pub mod fancy_item;
pub mod super_fancy_item;

pub use solid_item::SolidBlockItem;
pub use fancy_item::FancyBlockItem;
pub use liquid_item::LiquidBlockItem;
pub use super_fancy_item::SuperFancyBlockItem;

#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ItemStack {
    pub item: Item,
    pub count: u16,
}

impl Default for ItemStack {
    fn default() -> Self {
        let default_item = Item::default();
        Self {
            count: default_item.max_count(),
            item: default_item,
        }
    }
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

impl MaxCount for Item {
    fn max_count(&self) -> u16 {
        match self {
            Item::None => 0,
            Item::Block(block) => block.max_count(),
            Item::Liquid(liquid) => liquid.max_count(),
            Item::Fancy(fancy) => fancy.max_count(),
            Item::SuperFancy(super_fancy) => super_fancy.max_count(),
            Item::Instrument(_) => 1,
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::Block(SolidBlockItem::default())
    }
}

pub trait MaxCount {
    fn max_count(&self) -> u16;
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