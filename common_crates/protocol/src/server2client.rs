use math::Quat;
use block::BlockState;
use basic::position::*;

use serde::{Deserialize, Serialize};
use chunk::Chunk;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Server2ClientPacket {
    Ping(S2CPing),

    AllowJoin,
    Kick(S2CKick),

    PlaceBlock(S2CPlaceBlock),
    PlaceChunk(S2CPlaceChunk),

    ClientMoves(S2cPlayerMove),

    Message(C2CChatMessage),
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct S2CPing {
    pub server_name: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct S2CKick {
    pub reason: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct S2CPlaceBlock {
    pub block_state: BlockState,
    pub position: WorldBlockPosition,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct S2CPlaceChunk {
    pub column: Box<Chunk>,
    pub position: WorldBlockPosition,
}

#[derive(Default, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct S2cPlayerMove {
    pub id: u64,
    pub world_pos: WorldPosition,
    pub rotation: Quat,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct C2CChatMessage {
    pub message: String,
}