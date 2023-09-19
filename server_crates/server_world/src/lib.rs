use std::collections::VecDeque;
use bevy::prelude::{Component, Entity, Resource};
use bevy::utils::HashMap;
use common_world::ChunkPosInWorld;
use protocol::server2client::Server2ClientPacket;

#[derive(Resource)]
pub struct ServerWorld(pub HashMap<ChunkPosInWorld, Entity>);

#[derive(Component)]
pub struct ChunkComponent {
    pub pos_in_world: ChunkPosInWorld,
    packet_queue: VecDeque<Server2ClientPacket>
}