use serde::{Deserialize, Serialize};
use serde_json::Error;

pub mod client2server;
pub mod server2client;

pub const PROTOCOL_VERSION: u64 = 1;

pub fn serialize<T: Serialize>(value: &T) -> Box<[u8]> {
    serde_json::to_vec(value).expect("Serialization failed").into_boxed_slice()
}

pub fn serialize_owned<T: Serialize>(value: T) -> Box<[u8]> {
    serialize(&value)
}

pub fn deserialize<'a, T: Deserialize<'a>>(slice: &'a [u8]) -> Result<T, Error> {
    serde_json::from_slice(slice)
}