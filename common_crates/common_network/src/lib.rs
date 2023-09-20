use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

#[cfg(feature = "server")]
mod server {
    pub use bevy_renet::RenetServerPlugin;
    pub use bevy_renet::transport::NetcodeServerPlugin;
    pub use bevy_renet::renet::RenetServer;
}

#[cfg(feature = "server")]
pub use server::*;

#[cfg(feature = "client")]
mod client {
    pub use bevy_renet::RenetClientPlugin;
    pub use bevy_renet::transport::NetcodeClientPlugin;
    pub use bevy_renet::renet::RenetClient;
}

#[cfg(feature = "client")]
pub use client::*;

pub use bevy_renet::renet::*;
pub use bevy_renet::transport::*;

pub const DEFAULT_SERVER_ADDRESS: SocketAddr = local_host(25344);
pub const DEFAULT_CLIENT_BIND_ADDRESS: SocketAddr = local_host(25343);

pub const fn local_host(port: u16) -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))
}

pub fn ip(ip: impl Into<[u8; 4]>, port: u16) -> SocketAddr {
    let ip = ip.into();
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]), port))
}