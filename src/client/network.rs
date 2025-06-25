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
    encryption::KEMClientKey,
    network::{
        ClientMessage, PROTOCOL_ID, ServerMessage, get_private_key_env, string_to_fixed_bytes,
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
        } => Some(&**username),
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

    // create an authentication requirement according to the connection token
    let authentication = ClientAuthentication::Secure { connect_token };

    // Open a UDP socket and configure a transport resource to use
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    commands.insert_resource(transport);
}

pub fn client_ping(mut client: ResMut<RenetClient>, keyboard: Res<ButtonInput<KeyCode>>) {
    // ping the server, if connected, when 'Space' key is pressed
    if keyboard.just_pressed(KeyCode::Space) {
        let ping_message = bincode::encode_to_vec::<ClientMessage, _>(
            ClientMessage::Ping,         // The message we are sending, just a simple ping
            bincode::config::standard(), // standard config
        )
        .unwrap();

        // use the Reliable cjannel cuz we want to make sure it reaches perfectly
        client.send_message(DefaultChannel::ReliableOrdered, ping_message);
        info!("Sent Ping!"); // log it on our terminal
    }
}

pub fn receive_server_message(mut client: ResMut<RenetClient>, mut kem: ResMut<KEMClientKey>) {
    // define the channels we are using currently to make it easier to use later
    let channels: [u8; 3] = [
        DefaultChannel::ReliableOrdered.into(),
        DefaultChannel::ReliableUnordered.into(),
        DefaultChannel::Unreliable.into(),
    ];

    // iterate through the clients, and then through the channels, and write logic for each
    for &channel_id in channels.iter() {
        while let Some(message) = client.receive_message(channel_id) {
            let server_message = ServerMessage::decode(&message); // use the function; defined in common crate
            match server_message {
                ServerMessage::Pong => {
                    info!("Received Pong from Server!"); // if its a Pong, just show it on the terminal
                }

                ServerMessage::KEMEncapsKey(encaps_key) => {
                    respond_kem_handshake(*encaps_key, &mut client, &mut kem); // if its encaps key, then use the encryption logic to send ciphertext key
                }

                _ => {}
            }
        }
    }
}

fn respond_kem_handshake(encaps_key: [u8; 800], client: &mut RenetClient, kem: &mut KEMClientKey) {
    // derive shared secret key and ciphertext
    let (ssk, ciphertext) = EncapsKey::try_from_bytes(encaps_key)
        .expect("Error trying to get encaps key from ByteArray.")
        .try_encaps()
        .expect("Error trying to get ssk from encaps key.");

    // save the ssk for later use
    *kem = KEMClientKey::SharedSecret(Box::new(ssk.into_bytes()));

    // serialize the cipher
    let ser_cipher = ciphertext.into_bytes();

    // send it back to server, to complete the handshake
    ClientMessage::send(
        ClientMessage::KEMCipherText(Box::new(ser_cipher)),
        client,
        DefaultChannel::ReliableOrdered.into(),
    );
}
