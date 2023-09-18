use std::collections::VecDeque;
use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bevy_renet::transport::NetcodeClientPlugin;
use protocol::client2server::Client2ServerPacket;
use protocol::server2client::Server2ClientPacket;

mod config;

pub struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        let (client, transport) = config::config_client(None, None);

        app
            .init_resource::<ReceivePacketsFromServerQueue>()
            .init_resource::<SendPacketsToServerQueue>()

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
pub struct ReceivePacketsFromServerQueue {
    coming_from_server_queue: VecDeque<Server2ClientPacket>,
}

impl ReceivePacketsFromServerQueue {
    pub fn receive(&mut self) -> Option<Server2ClientPacket> {
        self.coming_from_server_queue.pop_back()
    }

    pub fn incoming_packets(&mut self) -> impl Iterator<Item = Server2ClientPacket> + '_ {
        self.into_iter()
    }
}

impl Iterator for ReceivePacketsFromServerQueue {
    type Item = Server2ClientPacket;

    fn next(&mut self) -> Option<Self::Item> {
        self.receive()
    }
}

fn add_s2c_packets_to_queue_on_message_from_server(
    mut renet_client: ResMut<RenetClient>,
    mut incoming_packets_queue: ResMut<ReceivePacketsFromServerQueue>,
) {
    while let Some(packet) = renet_client.receive_message(CHANNEL) {
        if let Ok(packet) = protocol::deserialize::<Server2ClientPacket>(packet.as_ref()) {
            incoming_packets_queue.coming_from_server_queue.push_back(packet);
        };
    }
}

#[derive(Default, Resource)]
pub struct SendPacketsToServerQueue {
    send_to_server_queue: VecDeque<Client2ServerPacket>,
}

impl SendPacketsToServerQueue {
    pub fn send(&mut self, packet: Client2ServerPacket) {
        self.send_to_server_queue.push_front(packet)
    }
}

fn send_c2s_packets_from_queue(
    mut renet_client: ResMut<RenetClient>,
    mut send_packets_queue: ResMut<SendPacketsToServerQueue>,
) {
    while let Some(packet) = send_packets_queue.send_to_server_queue.pop_back() {
        let message = protocol::serialize(&packet);
        renet_client.send_message(CHANNEL, message);
    }
}