use serde::{Deserialize, Serialize};
use strum::EnumIter;
use block_item::torch::{Brightness, TorchType};

#[cfg(feature = "derive-display")]
use strum::Display;

#[cfg_attr(feature = "derive-display", derive(Display))]
#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum FancyBlock {
    Torch(TorchItemAttrs),
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct TorchItemAttrs {
    pub brightness: Brightness,
    pub torch_type: TorchType,
}