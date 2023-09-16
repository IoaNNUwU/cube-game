#![cfg(test)]

use strum::IntoEnumIterator;
use block::fancy_block::FancyBlock;
use block::liquid_block::LiquidBlock;
use block::solid_block::SolidBlock;
use item::fancy_item::FancyBlockItem;
use item::liquid_item::LiquidBlockItem;
use item::solid_item::SolidBlockItem;
use item::super_fancy_item::SuperFancyBlockItem;

#[test]
fn fancy_block_item_test() {
    let block_names: Vec<String> = FancyBlock::iter()
        .map(|block| block.to_string())
        .collect();

    let item_names: Vec<String> = FancyBlockItem::iter()
        .map(|item| item.to_string())
        .collect();

    assert_eq!(block_names, item_names);
}

#[test]
fn solid_block_item_test() {
    let block_names: Vec<String> = SolidBlock::iter()
        .map(|block| block.to_string())
        .collect();

    let item_names: Vec<String> = SolidBlockItem::iter()
        .map(|item| item.to_string())
        .collect();

    assert_eq!(block_names, item_names);
}

#[test]
fn super_fancy_block_item_test() {
    let block_names: Vec<String> = SuperFancyBlockItem::iter()
        .map(|block| block.to_string())
        .collect();

    let item_names: Vec<String> = SuperFancyBlockItem::iter()
        .map(|item| item.to_string())
        .collect();

    assert_eq!(block_names, item_names);
}

#[test]
fn liquid_block_item_test() {
    let block_names: Vec<String> = LiquidBlock::iter()
        .map(|block| block.to_string())
        .collect();

    let item_names: Vec<String> = LiquidBlockItem::iter()
        .map(|item| item.to_string())
        .collect();

    assert_eq!(block_names, item_names);
}