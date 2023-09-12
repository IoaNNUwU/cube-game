use basic::block::BlockState;
use basic::position::*;

use serde::{Deserialize, Serialize};

#[derive(PartialOrd, PartialEq)]
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Client2ServerPacket {
    HandShake(C2sHandShake),
    SetBlock(C2sSetBlock),
    ClientMoves(C2sPlayerMove),
    Message(C2sChatMessage),
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct C2sHandShake {
    pub player_name: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct C2sSetBlock {
    pub block_state: BlockState,
    pub position: WorldBlockPosition,
}

#[derive(PartialOrd, PartialEq)]
#[derive(Default, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct C2sPlayerMove {
    pub name: String,
    pub new_position: WorldPosition,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct C2sChatMessage {
    pub message: String,
}