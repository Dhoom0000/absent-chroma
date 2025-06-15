use std::{net::{SocketAddr, UdpSocket}, time::{Duration, SystemTime}};
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_renet::{netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerConfig}, renet::{ConnectionConfig, RenetServer, ServerEvent}, RenetServerPlugin};
use local_ip_address::local_ip;

use super::super::common;

pub fn start() {

    let mut app = App::new(); 

    app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.))))
        .add_plugins((RenetServerPlugin,NetcodeServerPlugin))
        .add_systems(Startup, create_renet_server)
        .add_systems(Update, server_events);
        // .add_systems(Update, print_hello)
    
    app.run();
}

fn print_hello(time: Res<Time>) {
    println!("{:?} Hello World!",time.delta());
}

fn create_renet_server(mut commands:Commands){
    let server = RenetServer::new(ConnectionConfig::default());

    commands.insert_resource(server);

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);
    info!("Creating Server!: {:?}",server_addr);

    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: common::network::PROTOCOL_ID,
        public_addresses:vec![server_addr],
        authentication: bevy_renet::netcode::ServerAuthentication::Unsecure,
        current_time
    };

    let inbound_server_addr = SocketAddr::new(local_ip().unwrap(), 42069);

    let socket = UdpSocket::bind(inbound_server_addr).unwrap();

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    commands.insert_resource(transport);


}

fn server_events(mut events:EventReader<ServerEvent>){
    for event in events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Connected {}!", client_id)
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Disconnected {}! Reason: {}",client_id,reason)
            }
        }
    }
}