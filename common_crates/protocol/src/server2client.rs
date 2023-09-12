use basic::block::BlockState;
use basic::position::*;

use serde::{Deserialize, Serialize};

#[derive(PartialOrd, PartialEq)]
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Server2ClientPacket {
    HandShake(S2cHandShake),
    SetBlock(S2cSetBlock),
    ClientMoves(S2cPlayerMove),
    Message(S2cChatMessage),
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct S2cHandShake {
    pub server_name: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct S2cSetBlock {
    pub block_state: BlockState,
    pub position: WorldBlockPosition,
}

#[derive(PartialOrd, PartialEq)]
#[derive(Default, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct S2cPlayerMove {
    pub name: String,
    pub new_position: WorldPosition,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct S2cChatMessage {
    pub message: String,
}