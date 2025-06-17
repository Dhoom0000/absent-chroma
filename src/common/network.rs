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
