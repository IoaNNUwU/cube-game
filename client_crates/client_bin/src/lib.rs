use bevy::prelude::*;

use bevy::DefaultPlugins;
use client_network::{ClientNetworkPlugin, IncomingS2CPacketsQueue, SendC2SPacketQueue};
use client_state::ClientStatePlugin;
use client_gameplay::ClientGameplayPlugin;
use protocol::client2server::{C2SConnect, Client2ServerPacket};
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

fn answer_handshake(
    mut s2c_queue: ResMut<IncomingS2CPacketsQueue>,
    mut c2s_queue: ResMut<SendC2SPacketQueue>,
) {
    for packet in s2c_queue.incoming_packets() {
        if let Server2ClientPacket::Ping(packet) = packet {
            c2s_queue.send(Client2ServerPacket::Connect(C2SConnect {
                player_name: format!("Steve_{}_player", packet.server_name),
            }));
        }
    }
}
