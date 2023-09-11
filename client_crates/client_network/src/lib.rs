use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_renet::{RenetClientPlugin, RenetServerPlugin};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::transport::NetcodeClientPlugin;
use connection::{DEFAULT_CLIENT_BIND_ADDRESS, DEFAULT_SERVER_ADDRESS};
use protocol::client2server::{Client2ServerPacket, ClientGivesHandshake};
use protocol::PROTOCOL_VERSION;
use protocol::server2client::{Server2ClientPacket, ServerGivesHandshake};

pub struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        let (client, transport) = new_renet_client();

        app
            .insert_resource(client)
            .insert_resource(transport)
            .add_plugins((
                RenetClientPlugin,
                NetcodeClientPlugin,
            ))
            .add_systems(
                Update, (
                    client_send.run_if(bevy_renet::transport::client_connected()),
                ));
    }
}

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let client = RenetClient::new(ConnectionConfig::default());
    let server_addr: SocketAddr = DEFAULT_SERVER_ADDRESS.parse().unwrap();
    let socket = UdpSocket::bind(DEFAULT_CLIENT_BIND_ADDRESS).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_VERSION,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    (client, transport)
}

fn client_send(mut client: ResMut<RenetClient>) {
    let packet = protocol::serialize(
        &Client2ServerPacket::HandShake(
            ClientGivesHandshake {
                player_name: "player1".to_string(),
            })
    );

    println!("Send {:?}", packet);
    client.send_message(DefaultChannel::ReliableOrdered, packet);
}