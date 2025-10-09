use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

use bevy::prelude::*;
use bevy_renet::{
    netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ChannelConfig, ConnectionConfig, RenetServer, ServerEvent},
};
use local_ip_address::local_ip;
use zeroize::Zeroize;

use crate::{
    common::network::{ConnectedUsers, UserData, get_private_key_env},
    server::{
        encryption::{self, DKeyStore, Nonce, SSKStore},
        network::messages::receive_client_messages,
    },
};
mod messages;

pub struct NetworkPlugin;

impl NetworkPlugin {
    fn create_renet_server(mut commands: Commands) {
        let mut connection_config = ConnectionConfig::default();

        connection_config
            .client_channels_config
            .push(ChannelConfig {
                channel_id: 3,
                send_type: bevy_renet::renet::SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(300),
                },
                max_memory_usage_bytes: 5 * 1024 * 1024,
            });

        connection_config
            .server_channels_config
            .push(ChannelConfig {
                channel_id: 3,
                send_type: bevy_renet::renet::SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(300),
                },
                max_memory_usage_bytes: 5 * 1024 * 1024,
            });


        let server = RenetServer::new(connection_config);

        commands.insert_resource(server);

        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();

        let server_addr =
            SocketAddr::new(local_ip().expect("Could not find local ip address."), 42069);

        info!("Creating Server!: {:?}", server_addr);

        let mut private_key = get_private_key_env();

        let authentication = ServerAuthentication::Secure { private_key };

        let server_config = ServerConfig {
            max_clients: 2,
            protocol_id: 69,
            public_addresses: vec![server_addr],
            authentication,
            current_time,
        };

        private_key.zeroize();

        let socket = UdpSocket::bind(server_addr)
            .expect("UdpSocket bind failure. Consider restarting the server.");

        let transport = NetcodeServerTransport::new(server_config, socket).expect(
            "NetcodeServerTransport could not be established. Consider restarting the server.",
        );

        commands.insert_resource(transport);
    }

    fn server_events(
        mut event_reader: MessageReader<ServerEvent>,
        transport: Res<NetcodeServerTransport>,
        mut users: ResMut<ConnectedUsers>,
        mut server: ResMut<RenetServer>,
        mut d_key_res: ResMut<DKeyStore>,
        mut ssk_res: ResMut<SSKStore>,
    ) {
        for event in event_reader.read() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    let username = transport
                        .user_data(*client_id)
                        .unwrap_or_else(|| UserData::from_str("Anon").0);

                    users.0.insert(*client_id, UserData(username));

                    let username_str = str::from_utf8(&username).unwrap_or("Anon");

                    info!(
                        "Client Connected => username: {} id: {}",
                        username_str, client_id
                    );

                    encryption::try_encryption(&mut server, *client_id, &mut d_key_res);
                }

                ServerEvent::ClientDisconnected { client_id, reason } => {
                    let username = users
                        .0
                        .get(client_id)
                        .unwrap_or(&UserData([0u8; 256]))
                        .to_username();

                    info!(
                        "Client Disconnected => username: {} id: {} reason: {:?}",
                        username, client_id, reason
                    );

                    ssk_res.0.remove(client_id);
                }
            }
        }
    }
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConnectedUsers(HashMap::new()));
        app.insert_resource(DKeyStore(HashMap::new()));
        app.insert_resource(SSKStore(HashMap::new()));
        app.insert_resource(Nonce(HashMap::new()));
        app.add_systems(Startup, Self::create_renet_server);
        app.add_systems(Update, Self::server_events);
        app.add_systems(Update, receive_client_messages);
    }
}
