#![cfg(test)]
use block::{BlockState, SolidBlock, solid};
use block::solid_block::CommonBlockAttrs;

#[test]
fn zero_args_test() {
    assert_eq!(
        BlockState::Solid(SolidBlock::UnbreakableStone),
        solid!(UnbreakableStone),
    );
    assert_eq!(
        BlockState::Solid(SolidBlock::UnbreakableStone),
        solid!(SolidBlock::UnbreakableStone),
    );
}

#[test]
fn one_arg_test() {
    assert_eq!(
        BlockState::Solid(SolidBlock::Stone(CommonBlockAttrs::default())),
        solid!(Stone),
    );
    assert_eq!(
        BlockState::Solid(SolidBlock::Stone(CommonBlockAttrs::default())),
        solid!(SolidBlock::Stone),
    );
}

#[test]
fn more_arg_test() {
    assert_eq!(
        BlockState::Solid(SolidBlock::PlainsLog(Default::default(), Default::default())),
        solid!(SolidBlock::PlainsLog),
    );
    assert_eq!(
        BlockState::Solid(SolidBlock::PlainsLog(Default::default(), Default::default())),
        solid!(PlainsLog),
    );
}