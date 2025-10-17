use std::collections::HashMap;

use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetServer};
use cryptoxide::chacha20poly1305::ChaCha20Poly1305;
use fips203::{
    SharedSecretKey, ml_kem_512,
    traits::{Decaps, KeyGen, SerDes},
};
use zeroize::{Zeroize, Zeroizing};

use crate::common::network::ServerMessage;

fn generate_key() -> ([u8; 800], [u8; 1632]) {
    let (encaps_key, decaps_key) = ml_kem_512::KG::try_keygen().expect("Failed encryption keygen.");

    let encaps_key_bytes = encaps_key.into_bytes();
    let decaps_key_bytes = decaps_key.into_bytes();

    (encaps_key_bytes, decaps_key_bytes)
}

pub fn try_encryption(server: &mut RenetServer, client_id: u64, d_key_res: &mut DKeyStore) {
    let (mut e_key, d_key) = generate_key();

    let server_message = ServerMessage::KEMEncapsKey(e_key);

    let message = bincode::encode_to_vec(server_message, bincode::config::standard()).unwrap();

    server.send_message(client_id, 3, message);

    d_key_res.0.insert(client_id, d_key.into());

    e_key.zeroize();
}

pub fn try_decaps(ct: [u8; 768], dk: &[u8; 1632]) -> SharedSecretKey {
    let ciphertext =
        ml_kem_512::CipherText::try_from_bytes(ct).expect("Error converting ciphertext.");
    let decaps_key =
        ml_kem_512::DecapsKey::try_from_bytes(*dk).expect("Error converting decaps key");
    let ssk = decaps_key
        .try_decaps(&ciphertext)
        .expect("Could not get ssk.");

    ssk
}

#[derive(Resource, Clone)]
pub struct DKeyStore(pub HashMap<u64, Zeroizing<[u8; 1632]>>);

#[derive(Resource, Clone)]
pub struct SSKStore(pub HashMap<u64, Zeroizing<[u8; 32]>>);

#[derive(Resource)]
pub struct Nonce(pub HashMap<u64, [u8; 12]>);

impl ServerMessage {
    pub fn send_encrypted(
        server: &mut RenetServer,
        ssk: &[u8; 32],
        message: &Self,
        client_id: u64,
        nonce_res: &mut Nonce,
    ) {
        let key = ssk;

        let default_nonce = [0u8; 12];

        let nonce = nonce_res.0.entry(client_id).or_insert(default_nonce);

        let aad = [0u8; 0];

        let mut cipher = ChaCha20Poly1305::new(key, &nonce, &aad);

        let mut input = vec![];

        let mut channel_id = DefaultChannel::ReliableOrdered;

        match *message {
            ServerMessage::Pong => {
                input = bincode::encode_to_vec(ServerMessage::Pong, bincode::config::standard())
                    .expect("Error sending encmsg.");
                channel_id = DefaultChannel::ReliableOrdered;
                info!("Sent Pong (Encrypted).");
            }

            _ => {}
        }

        let mut output = vec![0u8; input.len() + 16];
        let mut out_tag = [0u8; 16];

        cipher.encrypt(&input, &mut output[..input.len()], &mut out_tag);

        output[input.len()..].copy_from_slice(&out_tag);

        for i in (0..12).rev() {
            if nonce[i] == 255 {
                nonce[i] = 0;
            } else {
                nonce[i] += 1;
                break;
            }
        }

        server.send_message(client_id, channel_id, output);
    }
}
