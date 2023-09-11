use basic::block::BlockState;
use basic::position::*;

use serde::{Deserialize, Serialize};

#[derive(PartialOrd, PartialEq)]
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Client2ServerPacket {
    HandShake(ClientGivesHandshake),
    SetBlock(ClientPlacesBlock),
    ClientMoves(ClientMoves)
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct ClientGivesHandshake {
    pub player_name: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Default, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct ClientPlacesBlock {
    pub block_state: BlockState,
    pub position: WorldBlockPosition,
}

#[derive(PartialOrd, PartialEq)]
#[derive(Default, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ClientMoves {
    pub name: String,
    pub new_position: WorldPosition,
}
