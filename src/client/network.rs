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
use fips203::traits::SerDes;
use fips203::{ml_kem_512::EncapsKey, traits::Encaps};
use local_ip_address::local_ip;

use crate::common::{
    self,
    network::{ClientMessage, PROTOCOL_ID, get_private_key_env, string_to_fixed_bytes},
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

fn establish_secure_connection(mut client: ResMut<RenetClient>) {
    let raw_key_bytes = client
        .receive_message(DefaultChannel::ReliableOrdered)
        .unwrap();

    let deserialized_key = EncapsKey::try_from_bytes(
        bincode::decode_from_slice::<[u8; 800], _>(&raw_key_bytes, bincode::config::standard())
            .unwrap()
            .0,
    )
    .unwrap();

    let (ssk, ciphertext) = deserialized_key.try_encaps().unwrap();

    let message = bincode::encode_to_vec::<[u8; 768], _>(
        ciphertext.into_bytes(),
        bincode::config::standard(),
    )
    .unwrap();

    client.send_message(DefaultChannel::ReliableOrdered, message);
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
