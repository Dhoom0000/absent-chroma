use bevy::{ecs::resource::Resource, log::info};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use cryptoxide::chacha20poly1305::ChaCha20Poly1305;
use fips203::{
    SharedSecretKey,
    ml_kem_512::EncapsKey,
    traits::{Encaps, SerDes},
};
use zeroize::Zeroizing;

use crate::common::network::ClientMessage;

pub fn get_ciphertext(e_key_bytes: [u8; 800]) -> (SharedSecretKey, [u8; 768]) {
    let e_key = EncapsKey::try_from_bytes(e_key_bytes).expect("Encaps key parse failed.");

    let (ssk, ct) = e_key.try_encaps().expect("Failed ssk generation.");

    let ct_bytes = ct.into_bytes();

    (ssk, ct_bytes)
}

#[derive(Resource)]
pub struct SskStore(pub Zeroizing<[u8; 32]>);

#[derive(Resource)]
pub struct Nonce(pub [u8; 12]);

impl ClientMessage {
    pub fn send_encrypted(
        client: &mut RenetClient,
        ssk: &[u8; 32],
        message: &Self,
        nonce_res: &mut Nonce,
    ) {
        let key = ssk;

        let mut nonce = nonce_res.0;

        let aad = [0u8; 0];

        let mut cipher = ChaCha20Poly1305::new(key, &nonce, &aad);

        let mut input = vec![];

        let mut channel_id = DefaultChannel::ReliableOrdered;

        match *message {
            ClientMessage::Ping => {
                input = bincode::encode_to_vec(ClientMessage::Ping, bincode::config::standard())
                    .expect("Error sending encmsg.");
                channel_id = DefaultChannel::ReliableOrdered;
                info!("Sent Ping (Encrypted).");
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

        client.send_message(channel_id, output);
    }
}
