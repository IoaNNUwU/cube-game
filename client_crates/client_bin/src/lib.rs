use bevy::prelude::*;

use bevy::DefaultPlugins;
use client_network::{ClientNetworkPlugin, ReceivePacketsFromServerQueue, SendPacketsToServerQueue};
use client_state::ClientStatePlugin;
use client_gameplay::ClientGameplayPlugin;
use protocol::client2server::{C2sHandShake, Client2ServerPacket};
use protocol::server2client::Server2ClientPacket;
use textures::TexturesPlugin;
use window::CubeWindowPlugin;

pub fn run_client() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest())
        )
        .add_plugins((
            ClientStatePlugin,
            ClientNetworkPlugin,
            TexturesPlugin,
            CubeWindowPlugin,
            ClientGameplayPlugin,
        ))
        .add_systems(Update, (
            answer_handshake,
        ))
        .run();
}

fn setup() {
    
}

fn answer_handshake(
    mut s2c_queue: ResMut<ReceivePacketsFromServerQueue>,
    mut c2s_queue: ResMut<SendPacketsToServerQueue>,
) {
    for packet in s2c_queue.incoming_packets() {
        if let Server2ClientPacket::HandShake(packet) = packet {
            c2s_queue.send(Client2ServerPacket::HandShake(C2sHandShake {
                player_name: format!("Steve_{}_player", packet.server_name),
            }));
        }
    }
}






