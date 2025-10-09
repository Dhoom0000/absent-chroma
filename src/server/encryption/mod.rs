use std::collections::HashMap;

use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetServer};
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

    server.send_message(client_id, DefaultChannel::ReliableOrdered, message);

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
