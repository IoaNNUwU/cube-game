use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Biome {
    #[default]
    Void,
    Plains,
    Forest,
    Snowy,
}