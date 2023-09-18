use heck::{ToPascalCase, ToSnakeCase};
use item::fancy_item::FancyBlockItem;
use item::Item;
use item::liquid_item::LiquidBlockItem;
use item::solid_item::SolidBlockItem;
use item::super_fancy_item::SuperFancyBlockItem;

pub trait AssociatedTexture {
    fn texture_path(&self) -> Option<Box<str>>;
}

impl AssociatedTexture for Item {
    fn texture_path(&self) -> Option<Box<str>> {
        match self {
            Item::None => None,
            Item::Block(solid) => solid.texture_path(),
            Item::Liquid(liquid) => liquid.texture_path(),
            Item::Fancy(fancy) => fancy.texture_path(),
            Item::SuperFancy(super_fancy) => super_fancy.texture_path(),
            Item::Instrument(inst) =>
                Some(format!("instrument/{:?}.png", inst).to_snake_case().into_boxed_str()),
        }
    }
}

impl AssociatedTexture for SolidBlockItem {
    fn texture_path(&self) -> Option<Box<str>> {
        Some(format!("blocks/{}.png", self.to_string().to_snake_case()).into_boxed_str())
    }
}

impl AssociatedTexture for LiquidBlockItem {
    fn texture_path(&self) -> Option<Box<str>> {
        Some(format!("liquids/{}.png", self.to_string().to_snake_case()).into_boxed_str())
    }
}

impl AssociatedTexture for FancyBlockItem {
    fn texture_path(&self) -> Option<Box<str>> {
        Some(format!("fancy_blocks/{}.png", self.to_string().to_snake_case()).into_boxed_str())
    }
}

impl AssociatedTexture for SuperFancyBlockItem {
    fn texture_path(&self) -> Option<Box<str>> {
        Some(format!("super_fancy_blocks/{}.png", self.to_string().to_snake_case()).into_boxed_str())
    }
}

#[cfg(test)]
mod test {
    use item::solid_item::SolidBlockItem;
    use crate::AssociatedTexture;

    #[test]
    fn test() {
        println!("{}", &*SolidBlockItem::Stone.texture_path().unwrap());
    }
}