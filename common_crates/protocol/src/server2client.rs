use math::Quat;
use block::BlockState;
use basic::position::*;

use serde::{Deserialize, Serialize};
use chunk::Chunk;
use common_world::ChunkPosInWorld;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Server2ClientPacket {
    /// Answer to client's Ping packet. Provides server description in server list.
    Ping(S2CPing),

    /// State when player is connected but world isn't initialized.
    /// Used to safely change world, and signal to client he's connected,
    /// even if world isn't loaded.
    Joined,

    /// Disconnect client at any moment with a reason.
    Kick(S2CKick),

    /// Place block state at given [WorldBlockPosition]
    PlaceBlock(S2CPlaceBlock),

    /// Place [Chunk] at given [ChunkPosInWorld]
    PlaceChunk(S2CPlaceChunk),

    /// Client with given id moves at given position with given rotation.
    ClientMoves(S2cPlayerMove),

    /// Chat message
    Message(S2CChatMessage),
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
    pub position: ChunkPosInWorld,
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
pub struct S2CChatMessage {
    pub message: String,
}