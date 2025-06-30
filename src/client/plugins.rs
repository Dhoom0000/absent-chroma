use bevy::prelude::*;
use bevy_renet::{RenetClientPlugin, netcode::NetcodeClientPlugin};

use crate::{
    client::{
        input::InputPlugin, network::NetworkPlugin, player::PlayerPlugin, setup::SetupPlugin,
        ui::UIPlugin,
    },
    common::{encryption::KEMClientKey, user::UserLogin},
};

pub(super) struct SuperPlugin;

impl Plugin for SuperPlugin {
    fn build(&self, app: &mut App) {
        // Setup UI and show main menu
        app.add_plugins(UIPlugin);

        // setup lights, camera and action
        app.add_plugins(SetupPlugin);

        // Networking plugin
        app.add_plugins((RenetClientPlugin, NetcodeClientPlugin));

        // Resources to store encryption keys and User Login details
        app.insert_resource(UserLogin::default());
        app.insert_resource(KEMClientKey::default());

        app.add_plugins(NetworkPlugin);
        app.add_plugins(InputPlugin);
        app.add_plugins(PlayerPlugin);
    }
}
