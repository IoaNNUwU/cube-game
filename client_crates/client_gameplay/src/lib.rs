use bevy::app::{App, Update};
use bevy::prelude::{Component, in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Res, ResMut, Resource};
use camera::CubeCameraPlugin;
use client_network::{IncomingS2CPacketsQueue, SendC2SPacketQueue};
use client_state::ClientState;
use protocol::client2server::{C2SConnect, Client2ServerPacket};
use protocol::server2client::{S2CPing, Server2ClientPacket};

pub struct ClientGameplayPlugin;

impl Plugin for ClientGameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerName("Steve1".to_string()))
            .add_plugins((
                CubeCameraPlugin,
            ))
            .add_systems(
                OnEnter(ClientState::InGame), (
                    dummy,
                ))
            .add_systems(
                OnExit(ClientState::InGame), (
                    dummy,
                ))
            .add_systems(Update, (
                send_connection_request_on_ping_from_server,
            ).run_if(in_state(ClientState::InGame)));
    }
}

#[derive(Resource)]
pub struct PlayerName(pub String);

fn dummy() {}

#[derive(Component)]
pub struct ClientGameplayElementMarker;

fn send_connection_request_on_ping_from_server(
    mut s2c_queue: ResMut<IncomingS2CPacketsQueue>,
    mut c2s_queue: ResMut<SendC2SPacketQueue>,
    player_name: Res<PlayerName>
) {
    //  !!!  This logic isn't necessary right after connection  !!!
    for packet in s2c_queue.incoming_packets() {
        if let Server2ClientPacket::Ping(S2CPing { server_name }) = packet {
            println!("Ping {}", server_name);
            c2s_queue.send(Client2ServerPacket::Connect(C2SConnect {
                player_name: player_name.0.clone(),
            }));
        }
    }
}