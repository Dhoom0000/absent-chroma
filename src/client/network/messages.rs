use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use cryptoxide::chacha20poly1305::ChaCha20Poly1305;
use fips203::traits::SerDes;

use crate::{
    client::network::encryption::{Nonce, SskStore, get_ciphertext},
    common::network::{ClientMessage, NETWORK_CHANNELS, ServerMessage},
};

pub fn receive_kem_messages(
    mut client: ResMut<RenetClient>,
    mut ssks: ResMut<SskStore>,
    mut nonce_res: ResMut<Nonce>,
) {
    let channel_id = 3;
    while let Some(message) = client.receive_message(channel_id) {
        let (server_message, _) =
            bincode::decode_from_slice::<ServerMessage, _>(&message, bincode::config::standard())
                .unwrap_or_default();

        match server_message {
            ServerMessage::KEMEncapsKey(e_key) => {
                let (ssk, ct) = get_ciphertext(e_key);

                let message = ClientMessage::KEMCipherText(ct);

                let ciphertext = bincode::encode_to_vec(message, bincode::config::standard())
                    .expect("Error converting ciphertext to vec.");

                client.send_message(3, ciphertext);

                ssks.0 = ssk.into_bytes().into();
                nonce_res.0 = [0u8; 12];
            }

            _ => {}
        }
    }
}

pub fn receive_encrypted(
    mut client: ResMut<RenetClient>,
    mut commands: Commands,
    ssks: Res<SskStore>,
    mut nonce_res: ResMut<Nonce>,
) {
    for channel_id in NETWORK_CHANNELS {
        while let Some(message) = client.receive_message(channel_id) {
            let key = &*ssks.0;

            let mut output = vec![0u8; message.len() - 16];

            let nonce = &nonce_res.0;

            let aad = &[0u8; 0];

            let mut cipher = ChaCha20Poly1305::new(key, nonce, aad);

            cipher.decrypt(
                &message[..message.len() - 16],
                &mut output,
                &message[message.len() - 16..],
            );

            for i in (0..12).rev() {
                if nonce_res.0[i] == 255 {
                    nonce_res.0[i] = 0;
                } else {
                    nonce_res.0[i] += 1;
                    break;
                }
            }

            let message = output;

            let (server_message, _) = bincode::decode_from_slice::<ServerMessage, _>(
                &message,
                bincode::config::standard(),
            )
            .unwrap_or_default();

            match server_message {
                ServerMessage::Pong => {
                    info!("Received Pong! (Encrypted)");
                }

                ServerMessage::KEMEncapsKey(e_key) => {
                    let (ssk, ct) = get_ciphertext(e_key);

                    let message = ClientMessage::KEMCipherText(ct);

                    let ciphertext = bincode::encode_to_vec(message, bincode::config::standard())
                        .expect("Error converting ciphertext to vec.");

                    client.send_message(DefaultChannel::ReliableOrdered, ciphertext);

                    commands.insert_resource(SskStore(ssk.into_bytes().into()));
                }
            }
        }
    }
}
