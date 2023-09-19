use std::time::UNIX_EPOCH;
use bevy::prelude::*;
use protocol::client2server::Client2ServerPacket;
use protocol::server2client::{S2CPing, Server2ClientPacket};
use server_network::{Client2ServerMessage, IncomingC2SPacketsQueue, SendS2CPacketsQueue, Server2ClientMessage};

pub struct ServerGameplayPlugin;

impl Plugin for ServerGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            tick,
            print_if_client_gives_handshake,
        ));
    }
}

fn tick(
    mut s2c_queue: ResMut<SendS2CPacketsQueue>,
) {
    let time = std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    if time % 10 == 0 {
        s2c_queue.send_all(Server2ClientPacket::Ping(S2CPing {
            server_name: "LupaWorld".to_string(),
        }));
    }
}

fn print_if_client_gives_handshake(
    mut c2s_queue: ResMut<IncomingC2SPacketsQueue>,
) {
    for Client2ServerMessage { id, packet } in c2s_queue.incoming_packets() {
        if let Client2ServerPacket::Connected(handshake) = packet {
            let nick = handshake.player_name;
            println!("Client {nick}#{id} sends handshake");
        }
    }
}