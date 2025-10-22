use bevy::prelude::*;
use bevy_renet::client_connected;
use bevy_renet::renet::RenetClient;

use crate::client::network::encryption::{Nonce, SskStore};
use crate::client::world::{MainCamera, player};
use crate::client::{AppState, PreviousAppState};
use crate::common::network::ClientMessage;

pub struct ControlsPlugin;

impl ControlsPlugin {
    fn keyboard_input(
        keyboard: Res<ButtonInput<KeyCode>>,
        app_state: Res<State<AppState>>,
        mut next_state: ResMut<NextState<AppState>>,
        mut commands: Commands,
        mut player_transform: Query<&mut Transform, (With<player::Player>, Without<MainCamera>)>,
        mut camera_transform: Query<&mut Transform, (With<MainCamera>, Without<player::Player>)>,
        time: Res<Time>,
    ) {
        for key_pressed in keyboard.get_pressed() {
            match key_pressed {
                KeyCode::Escape => match *app_state.get() {
                    AppState::InGame => {
                        commands.insert_resource(PreviousAppState(Some(AppState::InGame)));
                        next_state.set(AppState::MainMenu);
                    }
                    _ => {}
                },

                KeyCode::KeyW => {
                    let mut transform = player_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.z += 5. * time.delta_secs();

                    let mut transform = camera_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.z += 5. * time.delta_secs();
                }

                KeyCode::KeyA => {
                    let mut transform = player_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.x += 5. * time.delta_secs();

                    let mut transform = camera_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.x += 5. * time.delta_secs();
                }

                KeyCode::KeyS => {
                    let mut transform = player_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.z -= 5. * time.delta_secs();

                    let mut transform = camera_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.z -= 5. * time.delta_secs();
                }

                KeyCode::KeyD => {
                    let mut transform = player_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.x -= 5. * time.delta_secs();

                    let mut transform = camera_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.x -= 5. * time.delta_secs();
                }
                _ => {}
            }
        }
    }

    fn send_ping(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut client: ResMut<RenetClient>,
        ssks: Res<SskStore>,
        mut nonce_res: ResMut<Nonce>,
    ) {
        for key_pressed in keyboard.get_just_pressed() {
            match key_pressed {
                KeyCode::Space => {
                    let ssk = &*ssks.0;
                    ClientMessage::send_encrypted(
                        &mut client,
                        ssk,
                        &ClientMessage::Ping,
                        &mut nonce_res,
                    );
                }
                _ => {}
            }
        }
    }
}

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::keyboard_input).run_if(in_state(AppState::InGame)),
        );
        app.add_systems(Update, (Self::send_ping).run_if(client_connected));
    }
}
