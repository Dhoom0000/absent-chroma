use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_renet::{
    netcode::{ClientAuthentication, NetcodeClientTransport},
    renet::{ConnectionConfig, DefaultChannel, RenetClient},
    *,
};
use local_ip_address::local_ip;

use super::super::common;

pub fn connect_to_server(mut commands: Commands) {
    let client = RenetClient::new(ConnectionConfig::default());

    commands.insert_resource(client);

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let client_id = current_time.as_millis() as u64;

    let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);

    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id,
        user_data: None,
        protocol_id: common::network::PROTOCOL_ID,
    };

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    commands.insert_resource(transport);
}

pub fn send_message(mut client: ResMut<RenetClient>) {
    // client.send_message(DefaultChannel::ReliableOrdered, "server message");
}

pub fn receive_message(mut client: ResMut<RenetClient>) {
    // while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
    //     // Handle Received message
    // }
}
