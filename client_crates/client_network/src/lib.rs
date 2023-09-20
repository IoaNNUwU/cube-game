use std::collections::VecDeque;
use bevy::prelude::*;
use common_network::RenetClientPlugin;
use common_network::{DefaultChannel, RenetClient};
use common_network::NetcodeClientPlugin;
use protocol::client2server::Client2ServerPacket;
use protocol::server2client::Server2ClientPacket;

mod config;

pub struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        let (client, transport) = config::config_client(None, None);

        app
            .init_resource::<IncomingS2CPacketsQueue>()
            .init_resource::<SendC2SPacketQueue>()

            .insert_resource(client)
            .insert_resource(transport)

            .add_plugins((
                RenetClientPlugin,
                NetcodeClientPlugin,
            ))
            .add_systems(PreUpdate, (
                add_s2c_packets_to_queue_on_message_from_server,
            ))
            .add_systems(PostUpdate, (
                send_c2s_packets_from_queue,
            ));
    }
}

const CHANNEL: DefaultChannel = DefaultChannel::ReliableOrdered;

#[derive(Default, Resource)]
pub struct IncomingS2CPacketsQueue {
    coming_from_server_queue: Vec<Server2ClientPacket>,
}

impl IncomingS2CPacketsQueue {
    pub fn receive(&mut self) -> Option<Server2ClientPacket> {
        self.coming_from_server_queue.pop()
    }

    pub fn incoming_packets(&mut self) -> impl Iterator<Item = Server2ClientPacket> + '_ {
        self.into_iter()
    }
}

impl Iterator for IncomingS2CPacketsQueue {
    type Item = Server2ClientPacket;

    fn next(&mut self) -> Option<Self::Item> {
        self.receive()
    }
}

fn add_s2c_packets_to_queue_on_message_from_server(
    mut renet_client: ResMut<RenetClient>,
    mut incoming_packets_queue: ResMut<IncomingS2CPacketsQueue>,
) {
    while let Some(packet) = renet_client.receive_message(CHANNEL) {
        if let Ok(packet) = protocol::deserialize::<Server2ClientPacket>(packet.as_ref()) {
            incoming_packets_queue.coming_from_server_queue.push(packet);
        };
    }
}

#[derive(Default, Resource)]
pub struct SendC2SPacketQueue {
    send_to_server_queue: VecDeque<Client2ServerPacket>,
}

impl SendC2SPacketQueue {
    pub fn send(&mut self, packet: Client2ServerPacket) {
        self.send_to_server_queue.push_front(packet)
    }
}

fn send_c2s_packets_from_queue(
    mut renet_client: ResMut<RenetClient>,
    mut send_packets_queue: ResMut<SendC2SPacketQueue>,
) {
    while let Some(packet) = send_packets_queue.send_to_server_queue.pop_back() {
        let message = protocol::serialize(&packet);
        renet_client.send_message(CHANNEL, message);
    }
}