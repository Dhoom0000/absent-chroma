use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetServer};
use bincode::config;
use cryptoxide::chacha20poly1305::ChaCha20Poly1305;
use fips203::traits::SerDes;

use crate::{
    common::network::{ClientMessage, ConnectedUsers, NETWORK_CHANNELS, ServerMessage, UserData},
    server::encryption::{DKeyStore, Nonce, SSKStore, try_decaps},
};

pub fn receive_client_messages(
    mut server: ResMut<RenetServer>,
    users: Res<ConnectedUsers>,
    mut dks: ResMut<DKeyStore>,
    mut ssks: ResMut<SSKStore>,
    mut nonce_res: ResMut<Nonce>,
) {
    for channel_id in NETWORK_CHANNELS {
        for client_id in server.clients_id() {
            let default_data = UserData::from_str("Unknown");
            let user_data = users.0.get(&client_id).unwrap_or(&default_data);
            let username = user_data.to_username();

            while let Some(mut message) = server.receive_message(client_id, channel_id) {
                if channel_id != 3 {
                    let key = &**ssks.0.get(&client_id).expect("Could not find Client Key.");

                    let mut output = vec![0u8; message.len() + 16];

                    let nonce = nonce_res.0.entry(client_id).or_insert([0u8; 12]);

                    let aad = [0u8; 0];

                    let mut cipher = ChaCha20Poly1305::new(key, nonce, &aad);

                    cipher.decrypt(
                        &message[..message.len() - 16],
                        &mut output,
                        &message[message.len() - 16..],
                    );

                    message = output.into();
                }

                let (client_message, _) = bincode::decode_from_slice::<ClientMessage, _>(
                    &message,
                    bincode::config::standard(),
                )
                .expect("Error decoding client message.");

                match client_message {
                    ClientMessage::Ping => {
                        let ssk = &**ssks.0.get(&client_id).expect("No SSK for the client");

                        send_encrypted(
                            &mut server,
                            ssk,
                            &ServerMessage::Pong,
                            client_id,
                            &mut nonce_res,
                        );
                        // server.send_message(client_id, DefaultChannel::ReliableOrdered, pong);

                        info!("Received Ping from client: {} id: {}", username, client_id);
                    }

                    ClientMessage::KEMCipherText(ct) => {
                        let dk = dks.0.get(&client_id).expect("Cannot find decaps key.");
                        let ssk = try_decaps(ct, dk);

                        dks.0.remove(&client_id);

                        send_encrypted(
                            &mut server,
                            &ssk.clone().into_bytes(),
                            &ServerMessage::Pong,
                            client_id,
                            &mut nonce_res,
                        );

                        ssks.0.insert(client_id, ssk.into_bytes().into());

                        info!("KEM encryption success.");
                    }
                }
            }
        }
    }
}

fn send_encrypted(
    server: &mut RenetServer,
    ssk: &[u8; 32],
    message: &ServerMessage,
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
            input = bincode::encode_to_vec(ServerMessage::Pong, config::standard())
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

    server.send_message(client_id, channel_id, output);
}
