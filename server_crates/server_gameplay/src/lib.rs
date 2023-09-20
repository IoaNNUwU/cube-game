use bevy::prelude::*;
use protocol::client2server::{C2SConnect, Client2ServerPacket};
use protocol::server2client::{S2CChatMessage, S2CPing, Server2ClientPacket};
use server_network::{Client2ServerMessage, IncomingC2SPacketsQueue, SendS2CPacketsQueue, Server2ClientMessage};

pub struct ServerGameplayPlugin;

impl Plugin for ServerGameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ServerName("HelloWorldCraft".to_string()))
            .add_systems(Update, (
                send_ping_on_ping_from_client,
                send_allow_connect_on_connect_from_client,
            ));
    }
}

#[derive(Resource)]
pub struct ServerName(pub String);

fn send_ping_on_ping_from_client(
    mut c2s_queue: ResMut<IncomingC2SPacketsQueue>,
    mut s2c_queue: ResMut<SendS2CPacketsQueue>,
    server_name: Res<ServerName>,
) {
    for Client2ServerMessage { client_id, packet } in c2s_queue.incoming_packets() {
        if let Client2ServerPacket::Ping = packet {
            let message = Server2ClientMessage {
                client_id,
                packet: Server2ClientPacket::Ping(S2CPing {
                    server_name: server_name.0.to_string(),
                }),
            };
            s2c_queue.send(message);
        }
    }
}

fn send_allow_connect_on_connect_from_client(
    mut c2s_queue: ResMut<IncomingC2SPacketsQueue>,
    mut s2c_queue: ResMut<SendS2CPacketsQueue>,
) {
    for Client2ServerMessage { client_id, packet } in c2s_queue.incoming_packets() {
        if let Client2ServerPacket::Connect(C2SConnect { player_name }) = packet {
            let message = Server2ClientMessage {
                client_id,
                packet: Server2ClientPacket::Joined,
            };
            s2c_queue.send(message);
            s2c_queue.send_all(Server2ClientPacket::Message(S2CChatMessage {
                message: format!("> {} joined", player_name),
            }))
        }
    }
}