use std::collections::VecDeque;
use bevy::prelude::{Bundle, Component, Entity, Resource};
use common_world::{Chunk, ChunkPosInWorld};
use protocol::server2client::Server2ClientPacket;

#[derive(Resource)]
pub struct ServerWorld(pub bevy::utils::HashMap<ChunkPosInWorld, Option<Entity>>);

#[derive(Component)]
pub struct ChunkComponent(pub Chunk);

#[derive(Component)]
pub struct ChunkPosInWorldComponent(pub ChunkPosInWorld);

#[derive(Component)]
pub struct ViewerPacketQueue(pub VecDeque<Server2ClientPacket>);

#[derive(Component)]
pub struct NewPlayerPacketQueue(pub VecDeque<Server2ClientPacket>);

#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk: ChunkComponent,
    pub chunk_pos_in_world: ChunkPosInWorldComponent,
    pub viewer_packet_queue: ViewerPacketQueue,
    pub new_player_packet_queue: NewPlayerPacketQueue,
}