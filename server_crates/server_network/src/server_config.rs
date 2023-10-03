use std::net::{SocketAddr, UdpSocket, SocketAddrV4};
use std::time::SystemTime;
use common_network::{ConnectionConfig, RenetServer};
use common_network::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use config::Config;
use protocol::PROTOCOL_VERSION;

pub fn config_server_in_config_toml_file() -> (RenetServer, NetcodeServerTransport) {

    let config_file_path = std::env::current_dir()
        .expect("Unable to find current dir")
        .join("Config.toml");

    let config_text = std::fs::read_to_string(&config_file_path)
        .expect(&format!("Unable to find Config.toml in current dir ({:?})", config_file_path.to_str()));

    let config: Config = toml::from_str(&config_text)
        .expect(&format!("Config text is wrong: ({})",  &config_text));

    let public_addr = SocketAddr::V4(SocketAddrV4::new(config.server.ip, config.server.port));

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
    ).unwrap_or_else(|_| panic!("Unable to change Udp socket to non-blocking mode"));

    let server = RenetServer::new(ConnectionConfig::default());

    (server, transport)
}