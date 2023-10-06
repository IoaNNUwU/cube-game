use std::net::Ipv4Addr;
use serde::{Serialize, Deserialize};

#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub client: ClientConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub expose_ip: Ipv4Addr,
    pub expose_port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { expose_ip: Ipv4Addr::new(127, 0, 0, 1), expose_port: 4456 }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub local_port: u16,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self { local_port: 4455 }
    }
}