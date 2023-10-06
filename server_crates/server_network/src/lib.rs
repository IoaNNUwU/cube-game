use bevy::prelude::*;
use common_network::transport::NetcodeServerTransport;
use derive_new::new;

use strum::IntoEnumIterator;

use std::ops::Deref;

use common_network::NetcodeServerPlugin;
use common_network::RenetServerPlugin;
use common_network::{Channel, RenetServer, ServerEvent};

use protocol::client2server::{C2SDisconnect, Client2ServerPacket};
use protocol::server2client::Server2ClientPacket;

mod server_config;

pub struct ServerNetworkPlugin;

impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        let (server, transport) = server_config::config_server_in_config_toml_file();

        app.init_resource::<IncomingC2SPacketsQueue>()
            .init_resource::<SendS2CPacketsQueue>()
            .insert_resource(server)
            .insert_resource(transport)
            .add_plugins((RenetServerPlugin, NetcodeServerPlugin))
            .add_systems(Startup, print_exposed_ip)
            .add_systems(
                PreUpdate,
                (
                    add_c2s_packet_to_queue_on_message_from_client,
                    add_connect_and_disconnect_message_to_queue_on_server_event,
                ),
            )
            .add_systems(PostUpdate, (clear_c2s_queue_after_all_messages_read,))
            .add_systems(
                PostUpdate,
                (send_s2c_packets_from_queue_and_clear_it_after,),
            );
    }
}

fn print_exposed_ip(transport: Res<NetcodeServerTransport>) {
    println!("Server started on {}", transport.addr())
}

/// ### Example bevy system
/// ```rust
/// use server_network::IncomingC2SPacketsQueue;
/// use bevy::prelude::*;
///
/// fn print_any_message_from_client(
///    mut incoming_messages: Res<IncomingC2SPacketsQueue>,
/// ) {
///    for packet in incoming_messages.iter() {
///        println!("{:?}", packet);
///    }
/// }
/// ```
/// - Note that incoming messages are aquired by Res<_> and not ResMut<_>.
/// This allows for parallel handling of incoming packets.
#[derive(Default, Resource)]
pub struct IncomingC2SPacketsQueue(Vec<Client2ServerMessage>);

impl Deref for IncomingC2SPacketsQueue {
    type Target = [Client2ServerMessage];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

#[derive(PartialEq, Debug, Clone, new)]
pub struct Client2ServerMessage {
    pub client_id: u64,
    pub packet: Client2ServerPacket,
}

fn add_c2s_packet_to_queue_on_message_from_client(
    mut renet_server: ResMut<RenetServer>,
    mut clients2server_connection: ResMut<IncomingC2SPacketsQueue>,
) {
    for client_id in renet_server.clients_id() {
        for channel in Channel::iter() {
            while let Some(packet) = renet_server.receive_message(client_id, channel) {
                if let Ok(packet) = protocol::deserialize::<Client2ServerPacket>(packet.as_ref()) {
                    let message = Client2ServerMessage { client_id, packet };
                    clients2server_connection.0.push(message);
                }
            }
        }
    }
}

fn add_connect_and_disconnect_message_to_queue_on_server_event(
    mut server_events: EventReader<ServerEvent>,
    mut c2s_queue: ResMut<IncomingC2SPacketsQueue>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                c2s_queue.0.push(Client2ServerMessage::new(
                    *client_id,
                    Client2ServerPacket::Ping,
                ));
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                c2s_queue.0.push(Client2ServerMessage::new(
                    *client_id,
                    Client2ServerPacket::Disconnect(C2SDisconnect::new(reason.to_string())),
                ));
            }
        }
    }
}

fn clear_c2s_queue_after_all_messages_read(mut c2s_queue: ResMut<IncomingC2SPacketsQueue>) {
    c2s_queue.0.clear()
}

/// ### Example bevy system
/// ```rust
/// use server_network::SendS2CPacketsQueue;
/// use protocol::server2client::Server2ClientPacket;
/// use protocol::server2client::S2CPing;
/// use bevy::prelude::*;
///
/// fn send_ping_every_tick(
///     mut s2c_queue: ResMut<SendS2CPacketsQueue>,
/// ) {
///     s2c_queue.send_all(Server2ClientPacket::Ping(S2CPing::new("ServerName".to_string())));
/// }
/// ```
#[derive(Default, Resource)]
pub struct SendS2CPacketsQueue {
    send_to_specific_client_queue: Vec<(Channel, Server2ClientMessage)>,
    send_all_queue: Vec<(Channel, Server2ClientPacket)>,
}

impl SendS2CPacketsQueue {
    pub fn send(&mut self, message: Server2ClientMessage, channel: Channel) {
        self.send_to_specific_client_queue.push((channel, message));
    }

    pub fn send_to(&mut self, client_id: u64, packet: Server2ClientPacket, channel: Channel) {
        self.send_to_specific_client_queue
            .push((channel, Server2ClientMessage { client_id, packet }));
    }

    pub fn send_all(&mut self, packet: Server2ClientPacket, channel: Channel) {
        self.send_all_queue.push((channel, packet));
    }
}

#[derive(PartialEq, Debug, Clone, new)]
pub struct Server2ClientMessage {
    pub client_id: u64,
    pub packet: Server2ClientPacket,
}

fn send_s2c_packets_from_queue_and_clear_it_after(
    mut renet_server: ResMut<RenetServer>,
    mut send_packets_queue: ResMut<SendS2CPacketsQueue>,
) {
    for (channel, s2c_message) in send_packets_queue.send_to_specific_client_queue.iter() {
        let message = protocol::serialize(&s2c_message.packet);
        renet_server.send_message(s2c_message.client_id, *channel, message);
    }
    for (channel, s2c_packet) in send_packets_queue.send_all_queue.iter() {
        let message = protocol::serialize(&s2c_packet);
        for id in renet_server.clients_id() {
            renet_server.send_message(id, *channel, message.clone());
        }
    }
    send_packets_queue.send_to_specific_client_queue.clear();
    send_packets_queue.send_all_queue.clear();
}
