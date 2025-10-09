use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use bevy_renet::netcode::NetcodeClientPlugin;

use super::controls;
use super::network;
use super::ui;
use super::world;

pub struct SuperPlugin;

impl Plugin for SuperPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ui::UiPlugin);
        app.add_plugins(world::WorldPlugin);
        app.add_plugins(controls::ControlsPlugin);

        app.add_plugins(RenetClientPlugin);
        app.add_plugins(NetcodeClientPlugin);

        app.add_plugins(network::NetworkPlugin);
    }
}
