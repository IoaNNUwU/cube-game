use serde::{Deserialize, Serialize};
use strum::EnumIter;
use block_item::torch::{Brightness, TorchType};

#[cfg(feature = "derive-display")]
use strum::Display;
use crate::MaxCount;

#[cfg_attr(feature = "derive-display", derive(Display))]
#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum FancyBlockItem {
    Torch(TorchAttrs),
}

impl MaxCount for FancyBlockItem {
    fn max_count(&self) -> u16 {
        99
    }
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct TorchAttrs {
    pub placing: TorchPlacing,
    pub brightness: Brightness,
    pub torch_type: TorchType,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum TorchPlacing {
    #[default]
    BlockBelow,
    OnPositiveXWall,
    OnPositiveZWall,
    OnNegativeXWall,
    OnNegativeZWall,
}