use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;

#[cfg(feature = "server")]
mod server {
    pub use bevy_renet::renet::RenetServer;
    pub use bevy_renet::transport::NetcodeServerPlugin;
    pub use bevy_renet::RenetServerPlugin;
}

#[cfg(feature = "server")]
pub use server::*;

#[cfg(feature = "client")]
mod client {
    pub use bevy_renet::renet::RenetClient;
    pub use bevy_renet::transport::NetcodeClientPlugin;
    pub use bevy_renet::RenetClientPlugin;
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
    SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]),
        port,
    ))
}

pub struct Channels;

impl Channels {
    pub const UNRELIABLE: u8 = 0;
    pub const RELIABLE_ORDERED: u8 = 1;
    pub const RELIABLE_UNORDERED: u8 = 2;

    pub const CONFIG: [ChannelConfig; 3] = [
        UNRELIABLE_CHANNEL_CONFIG,
        RELIABLE_ORDERED_CHANNEL_CONFIG,
        RELIABLE_UNORDERED_CHANNEL_CONFIG,
    ];
}

impl IntoIterator for Channels {
    type Item = u8;

    type IntoIter = std::array::IntoIter<u8, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [Self::UNRELIABLE, Self::RELIABLE_ORDERED, Self::RELIABLE_UNORDERED].into_iter()
    }
}

const UNRELIABLE_CHANNEL_CONFIG: ChannelConfig = ChannelConfig {
    channel_id: Channels::UNRELIABLE,
    max_memory_usage_bytes: 5 * 1024 * 1024,
    send_type: SendType::Unreliable,
};

const RELIABLE_UNORDERED_CHANNEL_CONFIG: ChannelConfig = ChannelConfig {
    channel_id: Channels::RELIABLE_UNORDERED,
    max_memory_usage_bytes: 5 * 1024 * 1024,
    send_type: SendType::ReliableUnordered {
        resend_time: Duration::from_millis(300),
    },
};

const RELIABLE_ORDERED_CHANNEL_CONFIG: ChannelConfig = ChannelConfig {
    channel_id: Channels::RELIABLE_ORDERED,
    max_memory_usage_bytes: 5 * 1024 * 1024,
    send_type: SendType::ReliableOrdered {
        resend_time: Duration::from_millis(300),
    },
};
