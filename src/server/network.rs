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

use crate::common::{encryption::KEMServerState, network::*, user::ConnectedUsers};

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
    let server_addr = SocketAddr::new(local_ip().expect("Could not find local ip."), 42069);
    info!("Creating Server!: {:?}", server_addr);

    // the build.rs script creates a randomized private key as an env variable
    let private_key = get_private_key_env();

    // set authentication
    let authentication = ServerAuthentication::Secure { private_key };

    // Configure the server
    let server_config = ServerConfig {
        max_clients: 2,
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
    // generate encapsulation and decapsulation keys
    let (encaps_key, decaps_key) = KG::try_keygen().expect("FIPS203 KEM keygen failure.");

    // serialize the encapsulation key into ByteArray
    let ser_encaps_key = encaps_key.into_bytes();

    // convert to the ServerMessage type to send the key to client
    let server_message = ServerMessage::KEMEncapsKey(Box::new(ser_encaps_key));

    // store the decaps key for later use
    kem_state
        .decaps_key
        .insert(client_id, Box::new(decaps_key.into_bytes()));

    // send the message to the client
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
    // read the event queue
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let username = transport
                    .user_data(*client_id)
                    .unwrap_or(string_to_fixed_bytes("None"));
                // add the user id and username to the hashmap for later use
                users.0.insert(*client_id, username);
                info!(
                    "Connected {}! User: {}",
                    client_id,
                    fixed_bytes_to_string(&username)
                );

                // send kem handshake key
                establish_kem_encryption_handshake(&mut server, &mut kem_resource, *client_id);
            }

            ServerEvent::ClientDisconnected { client_id, reason } => {
                let fallback_username = string_to_fixed_bytes("None");
                // remove the user from the list
                let username = users.0.get(client_id).unwrap_or(&fallback_username);
                kem_resource.decaps_key.remove(client_id);
                kem_resource.shared_secrets.remove(client_id);
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
    // list of channels
    let channels: [u8; 3] = [
        DefaultChannel::ReliableOrdered.into(),
        DefaultChannel::ReliableUnordered.into(),
        DefaultChannel::Unreliable.into(),
    ];

    // check for all clients
    for &client_id in server.clients_id().iter() {
        // poll the hashmap for username if necessary
        let username = users.0.get(&client_id).unwrap_or(&[0u8; 256]);

        // for each channel
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

                        // Send Pong if we receive Ping from a client
                        ServerMessage::send(
                            ServerMessage::Pong,
                            &mut server,
                            client_id,
                            channel_id,
                        );
                        info!("Sent Pong!");
                    }

                    // If its a response for the KEM handshake, get the secret key and store for later use
                    ClientMessage::KEMCipherText(ser_cipher) => {
                        let cipher = CipherText::try_from_bytes(*ser_cipher)
                            .expect("error trying to get ciphertext to decaps key");

                        let ssk = DecapsKey::try_from_bytes(
                            **kem
                                .decaps_key
                                .get(&client_id)
                                .expect("Client decaps key does not exist"),
                        )
                        .expect("error trying to convert bytes to DecapsKey")
                        .try_decaps(&cipher)
                        .expect("error trying to get decaps ssk");

                        kem.shared_secrets
                            .insert(client_id, Box::new(ssk.into_bytes()));

                        kem.decaps_key
                            .remove(&client_id)
                            .expect("Cannot remove client decaps key.");

                        info!("{:?}", kem);
                    }

                    _ => {}
                }
            }
        }
    }
}
