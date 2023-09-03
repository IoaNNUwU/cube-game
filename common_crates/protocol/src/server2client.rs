use basic::block::BlockState;

use basic::position::WorldBlockPosition;

use serde::{Deserialize, Serialize};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FromServerPacket {
    HandShake(FromServerHandShake),
    SetBlock(FromServerPlaceBlock),
}

#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FromServerHandShake {
    pub server_name: String,
}

#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FromServerPlaceBlock {
    pub block_state: BlockState,
    pub position: WorldBlockPosition,
}
