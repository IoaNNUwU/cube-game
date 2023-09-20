use std::collections::VecDeque;
use std::ops::Deref;
use bevy::prelude::*;

use common_network::{DefaultChannel, RenetServer, ServerEvent};
use common_network::RenetServerPlugin;
use common_network::NetcodeServerPlugin;

use protocol::client2server::{C2SDisconnect, Client2ServerPacket};
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
                add_connect_and_disconnect_message_to_queue_on_server_event,
            ))
            .add_systems(PostUpdate, (
                send_s2c_packets_from_queue,
            ));
    }
}

const CHANNEL: DefaultChannel = DefaultChannel::ReliableOrdered;

#[derive(Default, Resource)]
pub struct IncomingC2SPacketsQueue {
    coming_from_clients_queue: Vec<Client2ServerMessage>,
}

impl IncomingC2SPacketsQueue {
    pub fn receive(&mut self) -> Option<Client2ServerMessage> {
        self.coming_from_clients_queue.pop()
    }

    pub fn incoming_packets(&mut self) -> impl Iterator<Item=Client2ServerMessage> + '_ {
        self.into_iter()
    }

    /// private
    fn push(&mut self, message: Client2ServerMessage) {
        self.coming_from_clients_queue.push(message);
    }
}

impl Iterator for IncomingC2SPacketsQueue {
    type Item = Client2ServerMessage;

    fn next(&mut self) -> Option<Self::Item> {
        self.receive()
    }
}

pub struct Client2ServerMessage {
    pub client_id: u64,
    pub packet: Client2ServerPacket,
}

fn add_c2s_packet_to_queue_on_message_from_client(
    mut renet_server: ResMut<RenetServer>,
    mut clients2server_connection: ResMut<IncomingC2SPacketsQueue>,
) {
    for id in renet_server.clients_id() {
        while let Some(packet) = renet_server.receive_message(id, CHANNEL) {
            if let Ok(packet) = protocol::deserialize::<Client2ServerPacket>(packet.as_ref()) {
                let message = Client2ServerMessage { client_id: id, packet };
                clients2server_connection.push(message);
            }
        }
    }
}

fn add_connect_and_disconnect_message_to_queue_on_server_event(
    mut server_events: EventReader<ServerEvent>,
    mut clients2server_connection: ResMut<IncomingC2SPacketsQueue>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let message = Client2ServerMessage {
                    client_id: *client_id,
                    packet: Client2ServerPacket::Ping,
                };
                clients2server_connection.push(message);
            },
            ServerEvent::ClientDisconnected { client_id, reason } => {
                let message = Client2ServerMessage {
                    client_id: *client_id,
                    packet: Client2ServerPacket::Disconnect(C2SDisconnect {
                        reason: reason.to_string(),
                    })
                };
                clients2server_connection.push(message);
            }
        }
    }
}

#[derive(Default, Resource)]
pub struct SendS2CPacketsQueue {
    send_to_clients_queue: Vec<Server2ClientMessage>,
    send_all_queue: VecDeque<Server2ClientPacket>,
}

impl SendS2CPacketsQueue {
    pub fn send(&mut self, message: Server2ClientMessage) {
        self.send_to_clients_queue.push(message);
    }

    pub fn send_to(&mut self, client_id: u64, packet: Server2ClientPacket) {
        self.send_to_clients_queue.push(Server2ClientMessage { client_id, packet });
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
    while let Some(s2c_message) = send_packets_queue.send_to_clients_queue.pop() {
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