use basic::block::BlockState;
use basic::position::WorldBlockPosition;

use serde::{Deserialize, Serialize};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FromClientPacket {
    HandShake(FromClientHandShake),
    SetBlock(FromClientPlaceBlock),
}

#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FromClientHandShake {
    pub player_name: String,
}

#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FromClientPlaceBlock {
    pub block_state: BlockState,
    pub position: WorldBlockPosition,
}
