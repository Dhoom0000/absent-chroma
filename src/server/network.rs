use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::ecs::{event::EventReader, system::Commands};
use bevy::prelude::*;
use bevy_renet::{
    netcode::{NetcodeServerTransport, ServerConfig},
    renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent},
};
use local_ip_address::local_ip;

use crate::common::network::PROTOCOL_ID;

pub fn create_renet_server(mut commands: Commands) {
    let server = RenetServer::new(ConnectionConfig::default());

    commands.insert_resource(server);

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);
    info!("Creating Server!: {:?}", server_addr);

    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![server_addr],
        authentication: bevy_renet::netcode::ServerAuthentication::Unsecure,
        current_time,
    };

    let inbound_server_addr = SocketAddr::new(local_ip().unwrap(), 42069);

    let socket = UdpSocket::bind(inbound_server_addr).unwrap();

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    commands.insert_resource(transport);
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

pub fn receive_message(mut server: ResMut<RenetServer>) {
    while let Some(message) = server.receive_message(0, DefaultChannel::ReliableOrdered) {
        // Handle Received message
        println!("{:?}", message);
    }
}
