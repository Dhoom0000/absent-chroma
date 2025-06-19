use std::{
    collections::HashMap,
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
        .unwrap();

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
    let socket = UdpSocket::bind(server_addr).unwrap();
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    commands.insert_resource(transport);
}

fn secure_connection(mut server: ResMut<RenetServer>, mut commands: Commands) {
    // initiate two empty buffers for random numbers
    let mut d_buf = [0u8; 32];
    let mut z_buf = [0u8; 32];

    // fill the buffers with random system entropy values
    let _ = getrandom::fill(&mut d_buf).unwrap();
    let _ = getrandom::fill(&mut z_buf).unwrap();

    // use buffers to create encapsulation and decapsulation keys
    let (encaps_key, decaps_key) = KG::keygen_from_seed(d_buf, z_buf);

    // serialize the key
    let message = bincode::encode_to_vec::<ServerMessage, _>(
        ServerMessage::KEMHandshake {
            message: Vec::from(encaps_key.into_bytes()),
        },
        bincode::config::standard(),
    )
    .unwrap();

    // send the encapsulation key to the client
    server.broadcast_message(DefaultChannel::ReliableOrdered, message);

    // save the decaps key as a resource and leave th shared key empty for now
    commands.insert_resource(KEMServerState {
        decaps_key,
        shared_secrets: HashMap::new(),
    });
}

fn receive_secure_cipher(
    mut server: ResMut<RenetServer>,
    mut kem_resource: ResMut<KEMServerState>,
) {
    for &client_id in server.clients_id().iter() {
        let raw_cipher = bincode::decode_from_slice::<[u8; 768], _>(
            &(server.receive_message(client_id, 7).unwrap()),
            bincode::config::standard(),
        )
        .unwrap();

        let ciphertext = CipherText::try_from_bytes(raw_cipher.0).unwrap();

        let ssk = kem_resource.decaps_key.try_decaps(&ciphertext).unwrap();

        kem_resource.shared_secrets.insert(client_id, ssk);
    }
}

pub fn server_events(
    mut events: EventReader<ServerEvent>,
    transport: Res<NetcodeServerTransport>,
    mut users: ResMut<ConnectedUsers>,
) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let username = transport.user_data(*client_id).unwrap();
                users.0.insert(*client_id, username);
                info!(
                    "Connected {}! User: {}",
                    client_id,
                    fixed_bytes_to_string(&username)
                )
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                let username = users.0.get(client_id).unwrap();
                info!(
                    "Disconnected {}! User: {} Reason: {}",
                    client_id,
                    fixed_bytes_to_string(username),
                    reason
                )
            }
        }
    }
}

pub fn receive_ping(mut server: ResMut<RenetServer>) {
    for client_id in server.clients_id().iter() {
        while let Some(message) =
            server.receive_message(*client_id, DefaultChannel::ReliableOrdered)
        {
            let client_message = bincode::decode_from_slice::<ClientMessage, _>(
                &message,
                bincode::config::standard(),
            )
            .unwrap();
            match client_message.0 {
                ClientMessage::Ping => {
                    println!("Got ping from client: {:?}", client_id);
                }
                _ => {
                    continue;
                }
            }
        }
    }
}

pub fn receive_client_message(mut server: ResMut<RenetServer>, users: ResMut<ConnectedUsers>) {
    let channels: [u8; 3] = [
        DefaultChannel::ReliableOrdered.into(),
        DefaultChannel::ReliableUnordered.into(),
        DefaultChannel::Unreliable.into(),
    ];

    for &client_id in server.clients_id().iter() {
        let username = users.0.get(&client_id).unwrap_or(&[0u8; 256]);
        for &channel_id in channels.iter() {
            while let Some(message) = server.receive_message(client_id, channel_id) {
                let client_message = bincode::decode_from_slice::<ClientMessage, _>(
                    &message,
                    bincode::config::standard(),
                )
                .unwrap()
                .0;
                match client_message {
                    ClientMessage::Ping => {
                        info!(
                            "Received Ping from ID: {}, Username: {}",
                            client_id,
                            fixed_bytes_to_string(username)
                        );

                        send_message(ServerMessage::Pong, &mut server, client_id, channel_id);
                        info!("Sent Pong!");
                    }

                    _ => {}
                }
            }
        }
    }
}

fn send_message(message: ServerMessage, server: &mut RenetServer, client_id: u64, channel_id: u8) {
    match message {
        ServerMessage::Pong => {
            let pong_message = bincode::encode_to_vec::<ServerMessage, _>(
                ServerMessage::Pong,
                bincode::config::standard(),
            )
            .unwrap();

            server.send_message(client_id, channel_id, pong_message);
        }

        _ => {}
    }
}
