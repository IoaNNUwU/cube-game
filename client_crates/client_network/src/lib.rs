use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bevy_renet::transport::NetcodeClientPlugin;
use protocol::client2server::{Client2ServerPacket, C2sHandShake};
use protocol::server2client::Server2ClientPacket;

mod config;

pub struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        let (client, transport) = config::config_client(None, None);

        app
            .init_resource::<Events<Client2ServerEvent>>()
            .init_resource::<Events<Server2ClientEvent>>()

            .insert_resource(client)
            .insert_resource(transport)

            .add_plugins((
                RenetClientPlugin,
                NetcodeClientPlugin,
            ))
            .add_systems(
                PreUpdate, (
                    send_s2c_events_on_message_from_server,
                    send_message_to_server_on_c2s_events,
                    send_handshake_on_message_from_server,
                ));
    }
}

#[derive(Event)]
pub struct Client2ServerEvent {
    pub packet: Client2ServerPacket,
}

#[derive(Event)]
pub struct Server2ClientEvent {
    pub packet: Server2ClientPacket,
}

fn send_s2c_events_on_message_from_server(
    mut client: ResMut<RenetClient>,
    mut receive_packet_events: EventWriter<Server2ClientEvent>,
) {
    while let Some(packet) = client.receive_message(DefaultChannel::ReliableOrdered) {
        if let Ok(packet) = protocol::deserialize::<Server2ClientPacket>(packet.as_ref()) {
            receive_packet_events.send(Server2ClientEvent { packet });
        };
    }
}

fn send_message_to_server_on_c2s_events(
    mut client: ResMut<RenetClient>,
    mut receive_packet_events: EventReader<Client2ServerEvent>,
) {
    for event in receive_packet_events.iter() {
        let message = protocol::serialize(&event.packet);
        client.send_message(DefaultChannel::ReliableOrdered, message);
    }
}

fn send_handshake_on_message_from_server(
    mut client: ResMut<RenetClient>,
    mut incoming_events: EventReader<Server2ClientEvent>,
) {
    for event in incoming_events.iter() {
        match &event.packet {
            Server2ClientPacket::HandShake(p) => {
                println!("handshake from server `{}`", p.server_name);
                let message = protocol::serialize(
                    &Client2ServerPacket::HandShake(
                        C2sHandShake {
                            player_name: "steve1".to_string(),
                        }));
                client.send_message(DefaultChannel::ReliableOrdered, message);
            }
            _ => {}
        }
    }
}