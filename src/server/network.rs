use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::ecs::{event::EventReader, system::Commands};
use bevy::prelude::*;
use bevy_renet::{
    netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent},
};
use local_ip_address::local_ip;

use crate::common::{network::*, user::ConnectedUsers};

use fips203::{
    ml_kem_512::*,
    traits::{Decaps, KeyGen, SerDes},
};

pub fn create_renet_server(mut commands: Commands) {
    // insert a RenetServer resource
    let server = RenetServer::new(ConnectionConfig::default());

    commands.insert_resource(server);

    // Get current system time to use for configuration
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();

    // Open a local server at given port
    let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);
    info!("Creating Server!: {:?}", server_addr);

    // the build.rs script creates a randomized private key as an env variable
    let private_key = get_private_key_env();

    // set authentication
    let authentication = ServerAuthentication::Secure { private_key };

    // Configure the server
    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![server_addr],
        authentication,
        current_time,
    };

    // Add a UDP transport socket and resource
    let socket = UdpSocket::bind(server_addr)
        .expect("UdpSocket bind failure. Consider restarting the server.");
    let transport = NetcodeServerTransport::new(server_config, socket)
        .expect("NetcodeServerTransport could not be established. Consider restarting the server.");
    commands.insert_resource(transport);
}

fn establish_kem_encryption_handshake(
    server: &mut RenetServer,
    kem_state: &mut KEMServerState,
    client_id: u64,
) {
    let (encaps_key, decaps_key) = KG::try_keygen().expect("FIPS203 KEM keygen failure.");

    let ser_encaps_key = encaps_key.into_bytes();

    let server_message = ServerMessage::KEMEncapsKey(ser_encaps_key);

    kem_state.decaps_key = decaps_key;

    ServerMessage::send(
        server_message,
        server,
        client_id,
        DefaultChannel::ReliableOrdered.into(),
    );
}

pub fn server_events(
    mut events: EventReader<ServerEvent>,
    transport: Res<NetcodeServerTransport>,
    mut users: ResMut<ConnectedUsers>,
    mut server: ResMut<RenetServer>,
    mut kem_resource: ResMut<KEMServerState>,
) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let username = transport
                    .user_data(*client_id)
                    .unwrap_or(string_to_fixed_bytes("None"));
                users.0.insert(*client_id, username);
                info!(
                    "Connected {}! User: {}",
                    client_id,
                    fixed_bytes_to_string(&username)
                );

                establish_kem_encryption_handshake(&mut *server, &mut *kem_resource, *client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                let fallback_username = string_to_fixed_bytes("None");
                let username = users.0.get(client_id).unwrap_or(&fallback_username);
                Some(kem_resource.shared_secrets.remove(client_id));
                info!(
                    "Disconnected {}! User: {} Reason: {}",
                    client_id,
                    fixed_bytes_to_string(username),
                    reason
                );
            }
        }
    }
}

pub fn receive_client_message(
    mut server: ResMut<RenetServer>,
    users: ResMut<ConnectedUsers>,
    mut kem: ResMut<KEMServerState>,
) {
    let channels: [u8; 3] = [
        DefaultChannel::ReliableOrdered.into(),
        DefaultChannel::ReliableUnordered.into(),
        DefaultChannel::Unreliable.into(),
    ];

    for &client_id in server.clients_id().iter() {
        let username = users.0.get(&client_id).unwrap_or(&[0u8; 256]);
        for &channel_id in channels.iter() {
            while let Some(message) = server.receive_message(client_id, channel_id) {
                let client_message = ClientMessage::decode(&message);
                match client_message {
                    ClientMessage::Ping => {
                        info!(
                            "Received Ping from ID: {}, Username: {}",
                            client_id,
                            fixed_bytes_to_string(username)
                        );

                        ServerMessage::send(
                            ServerMessage::Pong,
                            &mut server,
                            client_id,
                            channel_id,
                        );
                        info!("Sent Pong!");
                    }

                    ClientMessage::KEMCipherText(ser_cipher) => {
                        let cipher = CipherText::try_from_bytes(ser_cipher)
                            .expect("error trying to get ciphertext to decaps key");

                        let ssk = kem
                            .decaps_key
                            .try_decaps(&cipher)
                            .expect("error trying to get decaps ssk");

                        kem.shared_secrets.insert(client_id, ssk);

                        info!("{:?}", kem);
                    }

                    _ => {}
                }
            }
        }
    }
}
