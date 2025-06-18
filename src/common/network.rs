use bevy::prelude::*;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum ClientMessage {
    Ping,
    Hello(String),
    Move([f32; 3]),
}

#[derive(Encode, Decode, Debug)]
pub enum ServerMessage {
    Pong,
    Welcome(u64),
    Broadcast(String),
}

pub const PROTOCOL_ID: u64 = 69;

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

pub fn string_to_fixed_bytes(s: &str) -> [u8; 256] {
    let bytes = s.as_bytes();
    let truncated = &bytes[..bytes.len().min(256)];
    assert!(truncated.len() <= 256, "String too long to fit in [u8; 256]");

    let mut fixed = [0u8; 256];
    fixed[..truncated.len()].copy_from_slice(truncated);
    fixed
}

pub fn fixed_bytes_to_string(bytes: &[u8; 256]) -> String {
    let nul_trimmed = bytes.split(|b| *b == 0).next().unwrap_or(&[]);
    String::from_utf8_lossy(nul_trimmed).to_string()
}