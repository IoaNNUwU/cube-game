use serde::{Deserialize, Serialize};
use strum::EnumIter;
use block_item::torch::{Brightness, TorchType};

#[cfg(feature = "derive-display")]
use strum::Display;

#[cfg_attr(feature = "derive-display", derive(Display))]
#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum FancyBlockItem {
    Torch(TorchAttrs),
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