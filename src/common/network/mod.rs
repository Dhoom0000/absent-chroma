use std::collections::HashMap;

use bevy::ecs::resource::Resource;
use bevy_renet::renet::DefaultChannel;
use bincode::{Decode, Encode};

pub fn get_private_key_env() -> [u8; 32] {
    let private_key_string = env!("PRIVATE_KEY");
    // parse the key from raw string
    let vec: Vec<u8> = private_key_string
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|b| b.trim().parse::<u8>().unwrap())
        .collect();
    let private_key: [u8; 32] = vec.try_into().unwrap();

    private_key
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct UserData(pub [u8; 256]);

impl UserData {
    pub fn to_username(&self) -> &str {
        str::from_utf8(&self.0).unwrap_or("Null")
    }

    pub fn from_str(str: &str) -> UserData {
        let string = str.as_bytes();

        let len = string.len().min(256);

        let mut arr = [0u8; 256];

        arr[..len].copy_from_slice(&string[..len]);

        UserData(arr)
    }
}

#[derive(Resource, Default)]
pub struct ConnectedUsers(pub HashMap<u64, UserData>);

#[derive(Encode, Debug, Clone, Copy, Decode, Default)]
pub enum ServerMessage {
    #[default]
    Pong,
    KEMEncapsKey([u8; 800]),
}

#[derive(Encode, Debug, Clone, Copy, Decode, Default)]
pub enum ClientMessage {
    #[default]
    Ping,
    KEMCipherText([u8; 768]),
}

pub const NETWORK_CHANNELS: [u8; 4] = [
    DefaultChannel::ReliableOrdered as u8,
    DefaultChannel::ReliableUnordered as u8,
    DefaultChannel::Unreliable as u8,
    3, // For unencrypted KEM handshake
];
