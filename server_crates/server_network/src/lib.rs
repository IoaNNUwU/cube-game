use bevy::prelude::*;

use bevy_renet::renet::{DefaultChannel, RenetServer, ServerEvent};
use bevy_renet::RenetServerPlugin;
use bevy_renet::transport::NetcodeServerPlugin;
use protocol::client2server::Client2ServerPacket;
use protocol::server2client::{Server2ClientPacket, S2cHandShake};

mod config;

pub struct ServerNetworkPlugin;

impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        let (server, transport) = config::config_server(None);

        app
            .init_resource::<Events<Client2ServerEvent>>()
            .init_resource::<Events<Server2ClientEvent>>()

            .insert_resource(server)
            .insert_resource(transport)

            .add_plugins((
                RenetServerPlugin,
                NetcodeServerPlugin,
            ))
            .add_systems(PreUpdate, (
                send_c2s_events_on_message_from_client,
                send_message_to_client_on_s2c_events,
                send_handshake_on_client_connected_to_startup_things,
            ));
    }
}

/// __Events that are coming to server from clients__
///
/// Usually we should listen for them like this
/// ```rust
/// use bevy::prelude::EventReader;
/// use protocol::client2server::Client2ServerPacket;
/// use server_network::Client2ServerEvent;
///
/// fn system(
///     mut c2s_events: EventReader<Client2ServerEvent>
/// ) {
///     for event in c2s_events.iter() {
///         match &event.packet {
///             Client2ServerPacket::HandShake(p) => {
///                 println!("HandShake packet from {}#{}",
///                     p.player_name,
///                     event.client_id
///                 );
///             }
///             _ => {}
///         };
///     }
/// }
/// ```
/// see [Server2ClientEvent] for the example of combined usage
#[derive(Event)]
pub struct Client2ServerEvent {
    pub client_id: u64,
    pub packet: Client2ServerPacket,
}

/// # Events server is sending to clients
///
/// Usually we should send them like this
///
/// ```rust
/// use bevy::prelude::EventWriter;
/// use protocol::client2server::Client2ServerPacket;
/// use protocol::server2client::{Server2ClientPacket, S2cHandShake};
/// use server_network::Server2ClientEvent;
///
/// fn system(
///     mut s2c_events: EventWriter<Server2ClientEvent>
/// ) {
///     s2c_events.send(Server2ClientEvent {
///         client_id: 0,
///         packet: Server2ClientPacket::HandShake(S2cHandShake {
///             server_name: "Test Craft".to_string(),
///         }),
///     });
/// }
/// ```
/// Combined usage of [Server2ClientEvent] & [Client2ServerEvent] allows client-server interaction in ECS style:
///
/// ```rust
/// use bevy::prelude::{EventReader, EventWriter};
/// use protocol::client2server::Client2ServerPacket;
/// use protocol::server2client::{Server2ClientPacket, S2cHandShake, S2cChatMessage};
/// use server_network::{Client2ServerEvent, Server2ClientEvent};
///
/// fn system(
///     mut c2s_events: EventReader<Client2ServerEvent>,
///     mut s2c_events: EventWriter<Server2ClientEvent>,
/// ) {
///     for incoming_event in c2s_events.iter() {
///         match &incoming_event.packet {
///             Client2ServerPacket::HandShake(p) => {
///                 s2c_events.send(Server2ClientEvent {
///                     client_id: incoming_event.client_id,
///                     packet: Server2ClientPacket::Message(S2cChatMessage {
///                         message: format!("Hello, {}", p.player_name),
///                     }),
///                 });
///             },
///         _ => {}
///         }
///     }
/// }
/// ```
#[derive(Event)]
pub struct Server2ClientEvent {
    pub client_id: u64,
    pub packet: Server2ClientPacket,
}

fn send_c2s_events_on_message_from_client(
    mut server: ResMut<RenetServer>,
    mut receive_packet_from_client_events: EventWriter<Client2ServerEvent>,
) {
    for client_id in server.clients_id() {
        while let Some(packet) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            if let Ok(packet) = protocol::deserialize::<Client2ServerPacket>(packet.as_ref()) {
                receive_packet_from_client_events.send(Client2ServerEvent { client_id, packet });
            };
        }
    }
}

fn send_message_to_client_on_s2c_events(
    mut server: ResMut<RenetServer>,
    mut send_message_to_client_events: EventReader<Server2ClientEvent>,
) {
    for event in send_message_to_client_events.iter() {
        let client_id = event.client_id;
        let message = protocol::serialize(&event.packet);

        if server.is_connected(client_id) {
            server.send_message(client_id, DefaultChannel::ReliableOrdered, message);
        };
    }
}

fn send_handshake_on_client_connected_to_startup_things(
    mut server: ResMut<RenetServer>,
    mut join_events: EventReader<ServerEvent>,
) {
    for event in join_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client connected {}", client_id);

                let message = protocol::serialize(
                    &Server2ClientPacket::HandShake(S2cHandShake {
                        server_name: "Eblocraft".to_string(),
                    }));

                server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason: _reason } => {
                println!("Client disconnected {}", client_id);
            }
        }
    }
}