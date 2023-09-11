use basic::block::BlockState;
use basic::position::*;

use serde::{Deserialize, Serialize};

#[derive(PartialOrd, PartialEq)]
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Server2ClientPacket {
    HandShake(ServerGivesHandshake),
    SetBlock(ServerPlacesBlock),
    ClientMoves(ServerMovesPlayer),
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct ServerGivesHandshake {
    pub server_name: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct ServerPlacesBlock {
    pub block_state: BlockState,
    pub position: WorldBlockPosition,
}

#[derive(PartialOrd, PartialEq)]
#[derive(Default, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ServerMovesPlayer {
    pub name: String,
    pub new_position: WorldPosition,
}