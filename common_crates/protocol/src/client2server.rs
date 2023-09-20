use math::Quat;
use block::BlockState;
use basic::position::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Client2ServerPacket {
    Ping,

    Connect(C2SConnect),

    Disconnect(C2SDisconnect),

    PlaceBlock(C2SPlaceBlock),

    ClientMoves(C2SPlayerMove),

    Message(C2SChatMessage),
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct C2SConnect {
    pub player_name: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct C2SDisconnect {
    pub reason: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct C2SPlaceBlock {
    pub block_state: BlockState,
    pub position: WorldBlockPosition,
}

#[derive(Default, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct C2SPlayerMove {
    pub world_pos: WorldPosition,
    pub rotation: Quat,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct C2SChatMessage {
    pub message: String,
}