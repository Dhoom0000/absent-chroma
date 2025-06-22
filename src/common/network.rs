use std::collections::HashMap;

use bevy::prelude::*;
use bevy_renet::renet::{Bytes, RenetClient, RenetServer};
use bincode::*;
use fips203::{SharedSecretKey, ml_kem_512::DecapsKey};

#[derive(Encode, Decode, Debug)]
pub enum ClientMessage {
    Ping,
    KEMCipherText(Box<[u8; 768]>),
    Hello(String),
    Move([f32; 3]),
    Error(String),
}

impl ClientMessage {
    pub fn decode(message: &Bytes) -> ClientMessage {
        let deserialized_message =
            bincode::decode_from_slice::<ClientMessage, _>(message, config::standard())
                .expect("Error trying to deserialize Pong message.")
                .0;

        match deserialized_message {
            ClientMessage::Ping => ClientMessage::Ping,

            ClientMessage::KEMCipherText(ciphertext) => ClientMessage::KEMCipherText(ciphertext),

            ClientMessage::Hello(hello_str) => ClientMessage::Hello(hello_str),

            ClientMessage::Move(vec) => ClientMessage::Move(vec),

            _ => ClientMessage::Error("Client Message mismatch.".to_string()),
        }
    }

    pub fn send(message: ClientMessage, client: &mut RenetClient, channel_id: u8) {
        let serialized_message =
            bincode::encode_to_vec::<ClientMessage, _>(message, config::standard())
                .expect("Error trying to encode Client Message to vec.");

        client.send_message(channel_id, serialized_message);
    }
}

#[derive(Encode, Decode, Debug)]
pub enum ServerMessage {
    Pong,
    KEMEncapsKey(Box<[u8; 800]>),
    Welcome(u64),
    Broadcast(String),
    Error(String),
}

impl ServerMessage {
    pub fn decode(message: &Bytes) -> ServerMessage {
        let deserialized_message =
            bincode::decode_from_slice::<ServerMessage, _>(message, config::standard())
                .expect("Error trying to deserialize Pong message.")
                .0;

        match deserialized_message {
            ServerMessage::Pong => ServerMessage::Pong,

            ServerMessage::KEMEncapsKey(encaps_key) => ServerMessage::KEMEncapsKey(encaps_key),

            ServerMessage::Welcome(user_id) => ServerMessage::Welcome(user_id),

            ServerMessage::Broadcast(message) => ServerMessage::Broadcast(message),

            _ => ServerMessage::Error("Server Message mismatch.".to_string()),
        }
    }

    pub fn send(message: ServerMessage, server: &mut RenetServer, client_id: u64, channel_id: u8) {
        let serialized_message =
            bincode::encode_to_vec::<ServerMessage, _>(message, config::standard())
                .expect("Error trying to encode Server Message to vec.");

        server.send_message(client_id, channel_id, serialized_message);
    }
}

#[derive(Resource, Default)]
pub enum KEMClientKey {
    #[default]
    Pending,
    SharedSecret(SharedSecretKey),
}

#[derive(Resource, Default)]
pub struct KEMServerState {
    pub decaps_key: HashMap<u64, DecapsKey>,
    pub shared_secrets: HashMap<u64, SharedSecretKey>, // client_id -> ssk
}

impl std::fmt::Debug for KEMServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KEMServerState {{ decaps_keys: {}, shared_secrets: [{} clients] }}",
            self.decaps_key.len(),
            self.shared_secrets.len()
        )
    }
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
    assert!(
        truncated.len() <= 256,
        "String too long to fit in [u8; 256]"
    );

    let mut fixed = [0u8; 256];
    fixed[..truncated.len()].copy_from_slice(truncated);
    fixed
}

pub fn fixed_bytes_to_string(bytes: &[u8; 256]) -> String {
    let nul_trimmed = bytes.split(|b| *b == 0).next().unwrap_or(&[]);
    String::from_utf8_lossy(nul_trimmed).to_string()
}

#[test]
fn test_private_key() {
    assert_ne!(
        get_private_key_env(),
        [0u8; 32],
        "Environment key or file does not exist, using a ZERO array key. Unsafe."
    );
}

#[test]
fn test_str_to_u8_arr() {
    // Case 1: Basic string
    let mut expected1 = [0u8; 256];
    let bytes1 = b"Hello World!";
    expected1[..bytes1.len()].copy_from_slice(bytes1);
    assert_eq!(
        string_to_fixed_bytes("Hello World!"),
        expected1,
        "Failed on input: 'Hello World!'"
    );

    // Case 2: Empty string
    assert_eq!(
        string_to_fixed_bytes(""),
        [0u8; 256],
        "Failed on empty string"
    );

    // Case 3: String of exactly 256 characters
    let input_256 = "a".repeat(256);
    let mut expected2 = [0u8; 256];
    expected2.copy_from_slice(input_256.as_bytes());
    assert_eq!(
        string_to_fixed_bytes(&input_256),
        expected2,
        "Failed on 256-character string"
    );

    // Case 4: String longer than 256 characters (should be truncated)
    let input_long = "b".repeat(300);
    let mut expected3 = [0u8; 256];
    expected3.copy_from_slice(&input_long.as_bytes()[..256]);
    assert_eq!(
        string_to_fixed_bytes(&input_long),
        expected3,
        "Failed on string longer than 256 characters"
    );

    // Case 5: String with special characters
    let special = "Rüst!";
    let mut expected4 = [0u8; 256];
    let special_bytes = special.as_bytes(); // UTF-8: [82, 195, 188, 115, 116, 33]
    expected4[..special_bytes.len()].copy_from_slice(special_bytes);
    assert_eq!(
        string_to_fixed_bytes(special),
        expected4,
        "Failed on special character string"
    );
}

#[test]
fn test_u8_to_str() {
    // Case 1: Basic string
    let mut expected1 = [0u8; 256];
    let bytes1 = b"Hello World!";
    expected1[..bytes1.len()].copy_from_slice(bytes1);
    assert_eq!(
        fixed_bytes_to_string(&expected1),
        "Hello World!",
        "Failed on input: 'Hello World!'"
    );

    // Case 2: Empty string
    assert_eq!(
        fixed_bytes_to_string(&[0u8; 256]),
        "",
        "Failed on empty string"
    );

    // Case 3: String of exactly 256 characters
    let input_256 = "a".repeat(256);
    let mut expected2 = [0u8; 256];
    expected2.copy_from_slice(input_256.as_bytes());
    assert_eq!(
        fixed_bytes_to_string(&expected2),
        input_256,
        "Failed on 256-character string"
    );

    // Case 4: String longer than 256 characters (should be truncated)
    let mut input_long = "b".repeat(300);
    let mut expected3 = [0u8; 256];
    expected3.copy_from_slice(&input_long.as_bytes()[..256]);
    input_long.truncate(256);
    assert_eq!(
        fixed_bytes_to_string(&expected3),
        input_long,
        "Failed on string longer than 256 characters"
    );

    // Case 5: String with special characters
    let special = "Rüst!";
    let mut expected4 = [0u8; 256];
    let special_bytes = special.as_bytes(); // UTF-8: [82, 195, 188, 115, 116, 33]
    expected4[..special_bytes.len()].copy_from_slice(special_bytes);
    assert_eq!(
        fixed_bytes_to_string(&expected4),
        special,
        "Failed on special character string"
    );
}
