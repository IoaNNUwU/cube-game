use client2server::FromClientPacket;
use server2client::FromServerPacket;

pub mod client2server;
pub mod server2client;

use serde::{Deserialize, Serialize};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Packet {
    FromServer(FromServerPacket),
    FromClient(FromClientPacket),
}
