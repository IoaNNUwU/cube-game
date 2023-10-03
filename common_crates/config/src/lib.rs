use std::net::Ipv4Addr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub client: ClientConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub ip: Ipv4Addr,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    
}