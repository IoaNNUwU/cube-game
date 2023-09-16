pub mod torch {
    use serde::{Deserialize, Serialize};
    use strum::EnumIter;

    #[derive(Default, Clone, Hash, Debug)]
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    #[derive(Serialize, Deserialize, EnumIter)]
    pub enum Brightness {
        None,

        Darkest,
        Darker,
        Dark,

        DarkestNormal,
        DarkerNormal,
        #[default] Normal,
        BrighterNormal,
        BrightestNormal,

        Bright,
        Brighter,
        Brightest,
    }

    #[derive(Default, Clone, Hash, Debug)]
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    #[derive(Serialize, Deserialize, EnumIter)]
    pub enum TorchType {
        #[default]
        Coal,
        Amethyst,
        Diamond,
    }
}