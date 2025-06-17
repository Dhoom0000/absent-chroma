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
use local_ip_address::local_ip;
use fips203::{ml_kem_512::EncapsKey, traits::Encaps};
use fips203::traits::SerDes;

use crate::common::{
    self,
    network::{ClientMessage, PROTOCOL_ID},
};

pub fn connect_to_server(mut commands: Commands) {
    let handshake_channel = renet::ChannelConfig {
        channel_id: 7,
        send_type: renet::SendType::ReliableOrdered { resend_time: Duration::from_millis(300) },
        max_memory_usage_bytes: 5 * 1024 * 1024,
    };
    // Insert a RenetClient resource
    let client = RenetClient::new(ConnectionConfig {
        server_channels_config: vec![handshake_channel.clone()],
        client_channels_config:vec![handshake_channel],
        ..Default::default()
    });
    commands.insert_resource(client);

    // get current system time
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    // use time as client id for random ids
    let client_id = current_time.as_millis() as u64;

    // A local server
    let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);

    // create a randomized private key
    // let mut private_key = [0u8; 32];
    // getrandom::fill(&mut private_key).unwrap();

    // setup authentication
    // let connect_token = ConnectToken::generate(
    //     current_time,
    //     PROTOCOL_ID,
    //     600,
    //     client_id,
    //     15,
    //     vec![server_addr],
    //     None,
    //     &private_key,
    // )
    // .unwrap();
    let authentication = ClientAuthentication::Unsecure { protocol_id: PROTOCOL_ID, client_id, server_addr, user_data:None };

    // Open a UDP socket and configure a transport resource to use
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    commands.insert_resource(transport);

}

fn establish_secure_connection(mut client: ResMut<RenetClient>){
    let raw_key_bytes = client.receive_message(7).unwrap();

    let deserialized_key = EncapsKey::try_from_bytes(bincode::decode_from_slice::<[u8;800],_>(&raw_key_bytes, bincode::config::standard()).unwrap().0).unwrap();

    let (ssk,ciphertext) = deserialized_key.try_encaps().unwrap();

    let message = bincode::encode_to_vec::<[u8;768],_>(ciphertext.into_bytes(), bincode::config::standard()).unwrap();

    client.send_message(7, message);
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
