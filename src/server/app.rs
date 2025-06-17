use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use bevy_renet::{RenetServerPlugin, netcode::NetcodeServerPlugin};

use crate::server::network::*;

pub fn start() {
    let mut app = App::new();

    app.add_plugins(
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.,
        ))),
    );

    app.add_plugins(LogPlugin::default());

    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin);
    app.add_systems(Startup, create_renet_server);
    app.add_systems(Update, (server_events, receive_ping));

    app.run();
}
