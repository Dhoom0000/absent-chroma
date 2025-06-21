use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_renet::{
    netcode::{ClientAuthentication, ConnectToken, NetcodeClientTransport},
    renet::{ConnectionConfig, DefaultChannel, RenetClient},
    *,
};
use bincode::config;
use fips203::traits::SerDes;
use fips203::{ml_kem_512::EncapsKey, traits::Encaps};
use local_ip_address::local_ip;

use crate::common::{
    self,
    network::{
        ClientMessage, KEMClientKey, PROTOCOL_ID, ServerMessage, get_private_key_env,
        string_to_fixed_bytes,
    },
    user::UserLogin,
};

pub fn connect_to_server(mut commands: Commands, user: Res<UserLogin>) {
    // Insert a RenetClient
    let client = RenetClient::new(ConnectionConfig::default());
    commands.insert_resource(client);

    // get current system time
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    // use time as client id for random ids
    let client_id = current_time.as_millis() as u64;

    // A local server
    let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);

    // build.rs loads an environment private key; retrieve and parse it
    let private_key = get_private_key_env();

    let anon_username = &string_to_fixed_bytes("Anon");

    let user_data = match &*user {
        UserLogin::NotLoggedIn => Some(anon_username),
        UserLogin::IsLoggedIn {
            username,
            email: _,
            password: _,
        } => Some(username),
    };

    // create a connection token to use for authentication
    let connect_token = ConnectToken::generate(
        current_time,
        PROTOCOL_ID,
        30,
        client_id,
        10 * 60,
        vec![server_addr],
        user_data,
        &private_key,
    )
    .unwrap();

    let authentication = ClientAuthentication::Secure { connect_token };

    // Open a UDP socket and configure a transport resource to use
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    commands.insert_resource(transport);
}

pub fn client_ping(mut client: ResMut<RenetClient>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        let ping_message = bincode::encode_to_vec::<ClientMessage, _>(
            ClientMessage::Ping,
            bincode::config::standard(),
        )
        .unwrap();

        client.send_message(DefaultChannel::ReliableOrdered, ping_message);
        info!("Sent Ping!");
    }
}

pub fn receive_server_message(mut client: ResMut<RenetClient>, mut kem: ResMut<KEMClientKey>) {
    let channels: [u8; 3] = [
        DefaultChannel::ReliableOrdered.into(),
        DefaultChannel::ReliableUnordered.into(),
        DefaultChannel::Unreliable.into(),
    ];

    for &channel_id in channels.iter() {
        while let Some(message) = client.receive_message(channel_id) {
            let server_message = ServerMessage::decode(&message);
            match server_message {
                ServerMessage::Pong => {
                    info!("Received Pong from Server!");
                }

                ServerMessage::KEMEncapsKey(encaps_key) => {
                    respond_kem_handshake(encaps_key, &mut client, &mut *kem);
                }

                _ => {}
            }
        }
    }
}

fn respond_kem_handshake(encaps_key: [u8; 800], client: &mut RenetClient, kem: &mut KEMClientKey) {
    let (ssk, ciphertext) = EncapsKey::try_from_bytes(encaps_key)
        .expect("Error trying to get encaps key from ByteArray.")
        .try_encaps()
        .expect("Error trying to get ssk from encaps key.");

    *kem = KEMClientKey::SharedSecret(ssk);

    let ser_cipher = ciphertext.into_bytes();

    ClientMessage::send(
        ClientMessage::KEMCipherText(ser_cipher),
        client,
        DefaultChannel::ReliableOrdered.into(),
    );
}
