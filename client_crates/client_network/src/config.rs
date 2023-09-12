use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

use bevy_renet::renet::{ConnectionConfig, RenetClient};
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};

use common_network::{DEFAULT_CLIENT_BIND_ADDRESS, DEFAULT_SERVER_ADDRESS};

use protocol::PROTOCOL_VERSION;

pub fn config_client(
    server_addr: Option<SocketAddr>,
    socket_bind_addr: Option<SocketAddr>,
) -> (RenetClient, NetcodeClientTransport) {

    let client = RenetClient::new(ConnectionConfig::default());

    let server_addr: SocketAddr = server_addr.unwrap_or(DEFAULT_SERVER_ADDRESS);
    let socket = UdpSocket::bind(socket_bind_addr.unwrap_or(DEFAULT_CLIENT_BIND_ADDRESS)).unwrap();

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