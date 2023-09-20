use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;
use common_network::{ConnectionConfig, RenetServer};
use common_network::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use common_network::DEFAULT_SERVER_ADDRESS;
use protocol::PROTOCOL_VERSION;

pub fn config_server(public_addr: Option<SocketAddr>) -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(ConnectionConfig::default());

    let public_addr = public_addr.unwrap_or(DEFAULT_SERVER_ADDRESS);

    let socket = UdpSocket::bind(public_addr)
        .unwrap_or_else(|_| panic!("Unable to bind UdpSocket to {}", public_addr));

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
    ).unwrap_or_else(|_| panic!("Unable to create transport"));

    (server, transport)
}