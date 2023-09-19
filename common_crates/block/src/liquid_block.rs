use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[cfg(feature = "derive-display")]
use strum::Display;

#[cfg_attr(feature = "derive-display", derive(Display))]
#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize, EnumIter)]
pub enum LiquidBlock {
    Water(LiquidAttrs),
    Lava(LiquidAttrs),
    Honey(LiquidAttrs),
}

impl Default for LiquidBlock {
    fn default() -> Self {
        Self::Water(LiquidAttrs::default())
    }
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct LiquidAttrs {
    pub fill: Fill,
    pub flow: FlowDirection,
}

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum FlowDirection {
    #[default]
    NoFlow,
    TowardsPositiveX,
    TowardsNegativeX,
    TowardsPositiveZ,
    TowardsNegativeZ,
}

#[derive(Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Fill(pub u8);

impl Default for Fill {
    fn default() -> Self {
        Self(u8::MAX)
    }
}
