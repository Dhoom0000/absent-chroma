use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use bevy_renet::{RenetServerPlugin, netcode::NetcodeServerPlugin};

use crate::{
    common::{network::KEMServerState, user::ConnectedUsers},
    server::network::*,
};

pub fn start() {
    let mut app = App::new();

    // Limit to 60 fps
    app.add_plugins(
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.,
        ))),
    );

    // Enable logging for dev
    app.add_plugins(LogPlugin::default());

    // networking plugins
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin);

    // Server state plugins
    app.insert_resource(ConnectedUsers::default());
    app.insert_resource(KEMServerState::default());

    // Opens a port at given address, to allow incoming connections
    app.add_systems(Startup, create_renet_server);

    // Check for system events and client messages every frame
    app.add_systems(Update, (server_events, receive_client_message));

    app.run();
}
