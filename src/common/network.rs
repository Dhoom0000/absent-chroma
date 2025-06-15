use serde::{Serialize,Deserialize};
use bevy::prelude::*;

#[derive(Serialize,Deserialize,Debug)]
pub enum ClientMessage{
    Ping,
    Hello(String),
    Move(Vec3)

}

#[derive(Serialize,Deserialize,Debug)]
pub enum ServerMessage{
    Pong,
    Welcome(u64),
    Broadcast(String)
}

pub const PROTOCOL_ID: u64 = 1000;



