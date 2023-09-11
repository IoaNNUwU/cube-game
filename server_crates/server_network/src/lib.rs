use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::RenetServerPlugin;
use bevy_renet::transport::NetcodeServerPlugin;
use connection::DEFAULT_SERVER_ADDRESS;
use protocol::client2server::Client2ServerPacket;
use protocol::PROTOCOL_VERSION;
use protocol::server2client::Server2ClientPacket;

pub struct ServerNetworkPlugin;

impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        let (server, transport) = new_renet_server();

        app
            .insert_resource(server)
            .insert_resource(transport)
            .add_plugins((
                RenetServerPlugin,
                NetcodeServerPlugin,
            ))
            .add_systems(Update, (
                server_update.run_if(resource_exists::<RenetServer>()),
            ));
    }
}

fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(ConnectionConfig::default());

    let public_addr = DEFAULT_SERVER_ADDRESS.parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: PROTOCOL_VERSION,
        public_addr,
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(
        current_time,
        server_config,
        socket,
    ).unwrap();

    (server, transport)
}

fn server_update(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            let packet: Client2ServerPacket = protocol::deserialize(&message).unwrap();
            println!("Recieved {:?} from #{}", packet, client_id);
        }
    }
}
