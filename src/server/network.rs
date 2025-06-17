use std::{
    collections::HashMap, net::{SocketAddr, UdpSocket}, time::{Duration, SystemTime}
};

use bevy::ecs::{event::EventReader, system::{Commands}};
use bevy::prelude::*;
use bevy_renet::{
    netcode::{NetcodeServerTransport, ServerConfig},
    renet::{self, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent},
};
use local_ip_address::local_ip;

use crate::common::network::{ClientMessage, PROTOCOL_ID};

use fips203::{ml_kem_512::*, traits::{Decaps, KeyGen, SerDes}, SharedSecretKey};

#[derive(Resource)]
pub struct KEMServerState {
    pub decaps_key: DecapsKey,
    pub shared_secrets: HashMap<u64, SharedSecretKey>, // client_id -> ssk
}

pub fn create_renet_server(mut commands: Commands) {

    let handshake_channel = renet::ChannelConfig {
        channel_id: 7,
        send_type: renet::SendType::ReliableOrdered { resend_time: Duration::from_millis(300) },
        max_memory_usage_bytes: 5 * 1024 * 1024,
    };

    // insert a RenetServer resource
    let server = RenetServer::new(ConnectionConfig {
        server_channels_config: vec![handshake_channel.clone()],
        client_channels_config: vec![handshake_channel],
        ..Default::default()
    });
    commands.insert_resource(server);

    // Get current system time to use for configuration
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    // Open a local server at given port
    let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);
    info!("Creating Server!: {:?}", server_addr);

    

    // set authentication
    let authentication = bevy_renet::netcode::ServerAuthentication::Unsecure;

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

    let sec_con_sys_id = commands.register_system(secure_connection);
    commands.run_system(sec_con_sys_id);
}

fn secure_connection(mut server:ResMut<RenetServer>,mut commands:Commands){

    let mut d_buf = [0u8;32];
    let mut z_buf = [0u8;32];
    getrandom::fill(&mut d_buf);
    getrandom::fill(&mut z_buf);
    let (encaps_key, decaps_key) = KG::keygen_from_seed(d_buf,z_buf);

    let message = bincode::encode_to_vec::<[u8;800],_>(encaps_key.into_bytes(), bincode::config::standard()).unwrap();

    server.broadcast_message(7, message);

    commands.insert_resource(KEMServerState {
        decaps_key,
        shared_secrets:HashMap::new()
    });

}

fn receive_secure_cipher(mut server: ResMut<RenetServer>, mut kem_resource:ResMut<KEMServerState>) {
    for client_id in server.clients_id().iter() {
        let raw_cipher = bincode::decode_from_slice::<[u8;768],_>(&(server.receive_message(*client_id, 7).unwrap()), bincode::config::standard()).unwrap();

        let ciphertext = CipherText::try_from_bytes(raw_cipher.0).unwrap();

        let ssk = kem_resource.decaps_key.try_decaps(&ciphertext).unwrap();

        kem_resource.shared_secrets.insert(*client_id, ssk);


    }
}

pub fn server_events(mut events: EventReader<ServerEvent>) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Connected {}!", client_id)
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Disconnected {}! Reason: {}", client_id, reason)
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
