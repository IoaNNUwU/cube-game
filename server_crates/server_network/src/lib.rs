use std::collections::VecDeque;
use bevy::prelude::*;

use bevy_renet::renet::{DefaultChannel, RenetServer};
use bevy_renet::RenetServerPlugin;
use bevy_renet::transport::NetcodeServerPlugin;
use protocol::client2server::Client2ServerPacket;
use protocol::server2client::Server2ClientPacket;

mod config;

pub struct ServerNetworkPlugin;

impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        let (server, transport) = config::config_server(None);

        app
            .init_resource::<IncomingC2SPacketsQueue>()
            .init_resource::<SendS2CPacketsQueue>()

            .insert_resource(server)
            .insert_resource(transport)

            .add_plugins((
                RenetServerPlugin,
                NetcodeServerPlugin,
            ))
            .add_systems(PreUpdate, (
                add_c2s_packet_to_queue_on_message_from_client,
            ))
            .add_systems(PostUpdate, (
                send_s2c_packets_from_queue,
            ));
    }
}

const CHANNEL: DefaultChannel = DefaultChannel::ReliableOrdered;

#[derive(Default, Resource)]
pub struct IncomingC2SPacketsQueue {
    coming_from_clients_queue: VecDeque<Client2ServerMessage>,
}

impl IncomingC2SPacketsQueue {
    pub fn receive(&mut self) -> Option<Client2ServerMessage> {
        self.coming_from_clients_queue.pop_back()
    }

    pub fn incoming_packets(&mut self) -> impl Iterator<Item=Client2ServerMessage> + '_ {
        self.into_iter()
    }
}

impl Iterator for IncomingC2SPacketsQueue {
    type Item = Client2ServerMessage;

    fn next(&mut self) -> Option<Self::Item> {
        self.receive()
    }
}

pub struct Client2ServerMessage {
    pub id: u64,
    pub packet: Client2ServerPacket,
}

fn add_c2s_packet_to_queue_on_message_from_client(
    mut renet_server: ResMut<RenetServer>,
    mut clients2server_connection: ResMut<IncomingC2SPacketsQueue>,
) {
    for id in renet_server.clients_id() {
        while let Some(packet) = renet_server.receive_message(id, CHANNEL) {
            if let Ok(packet) = protocol::deserialize::<Client2ServerPacket>(packet.as_ref()) {
                let message = Client2ServerMessage { id, packet };
                clients2server_connection.coming_from_clients_queue.push_back(message);
            }
        }
    }
}

#[derive(Default, Resource)]
pub struct SendS2CPacketsQueue {
    send_to_clients_queue: VecDeque<Server2ClientMessage>,
    send_all_queue: VecDeque<Server2ClientPacket>,
}

impl SendS2CPacketsQueue {
    pub fn send(&mut self, message: Server2ClientMessage) {
        self.send_to_clients_queue.push_back(message);
    }

    pub fn send_all(&mut self, packet: Server2ClientPacket) {
        self.send_all_queue.push_back(packet);
    }
}

pub struct Server2ClientMessage {
    pub client_id: u64,
    pub packet: Server2ClientPacket,
}

fn send_s2c_packets_from_queue(
    mut renet_server: ResMut<RenetServer>,
    mut send_packets_queue: ResMut<SendS2CPacketsQueue>,
) {
    while let Some(s2c_message) = send_packets_queue.send_to_clients_queue.pop_back() {
        let message = protocol::serialize(&s2c_message.packet);
        renet_server.send_message(s2c_message.client_id, CHANNEL, message);
    }
    while let Some(packet) = send_packets_queue.send_all_queue.pop_back() {
        let message = protocol::serialize(&packet);

        for id in renet_server.clients_id() {
            renet_server.send_message(id, CHANNEL, message.clone());
        }
    }
}