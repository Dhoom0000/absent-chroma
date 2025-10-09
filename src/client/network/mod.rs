use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

use bevy::prelude::*;
use bevy_renet::{
    client_connected,
    netcode::{ClientAuthentication, ConnectToken, NetcodeClientTransport},
    renet::{ChannelConfig, ConnectionConfig, RenetClient},
};
use local_ip_address::local_ip;
use rand::TryRngCore;
use zeroize::Zeroize;

use crate::{
    client::{
        AppState,
        network::{login::UserLogin, messages::receive_kem_messages},
    },
    common::network::{UserData, get_private_key_env},
};
mod encryption;
pub mod login;
mod messages;

pub struct NetworkPlugin;

impl NetworkPlugin {
    fn connect_to_server(mut commands: Commands, user: Res<UserLogin>) {
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

        let client = RenetClient::new(connection_config);

        commands.insert_resource(client);

        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();

        let client_id = rand::rngs::OsRng.try_next_u64().unwrap_or_default();

        let server_addr = SocketAddr::new(local_ip().expect("Cannot find local ip."), 42069);

        let mut private_key = get_private_key_env();

        let user_data = match *user {
            UserLogin::LoggedIn(data) => data.0,

            UserLogin::NotLoggedIn => UserData::from_str("Anon").0,
        };

        let connect_token = ConnectToken::generate(
            current_time,
            69,
            30,
            client_id,
            2 * 60,
            vec![server_addr],
            Some(&user_data),
            &private_key,
        )
        .expect("Error building connection token.");

        private_key.zeroize();

        let authentication = ClientAuthentication::Secure { connect_token };

        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

        let transport = NetcodeClientTransport::new(current_time, authentication, socket)
            .expect("Could not create netcode transport.");

        commands.insert_resource(transport);

        commands.set_state(AppState::MainMenu);
    }
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ConnectToServer), Self::connect_to_server);
        app.add_systems(Update, receive_kem_messages.run_if(client_connected));
    }
}
