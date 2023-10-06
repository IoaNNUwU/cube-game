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
use strum::EnumIter;

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

#[derive(EnumIter, Clone, Copy)]
pub enum Channel {
    Unreliable = 0,
    ReliableUnordered = 1,
    ReliableOrdered = 2,
}

impl From<Channel> for u8 {
    fn from(channel: Channel) -> u8 {
        channel as u8
    }
}

impl Channel {
    pub const CONFIG: [ChannelConfig; 3] = [
        UNRELIABLE_CHANNEL_CONFIG,
        RELIABLE_ORDERED_CHANNEL_CONFIG,
        RELIABLE_UNORDERED_CHANNEL_CONFIG,
    ];
}

const UNRELIABLE_CHANNEL_CONFIG: ChannelConfig = ChannelConfig {
    channel_id: Channel::Unreliable as u8,
    max_memory_usage_bytes: 5 * 1024 * 1024,
    send_type: SendType::Unreliable,
};

const RELIABLE_UNORDERED_CHANNEL_CONFIG: ChannelConfig = ChannelConfig {
    channel_id: Channel::ReliableUnordered as u8,
    max_memory_usage_bytes: 5 * 1024 * 1024,
    send_type: SendType::ReliableUnordered {
        resend_time: Duration::from_millis(300),
    },
};

const RELIABLE_ORDERED_CHANNEL_CONFIG: ChannelConfig = ChannelConfig {
    channel_id: Channel::ReliableOrdered as u8,
    max_memory_usage_bytes: 5 * 1024 * 1024,
    send_type: SendType::ReliableOrdered {
        resend_time: Duration::from_millis(300),
    },
};
