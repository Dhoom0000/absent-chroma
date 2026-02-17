use bevy::prelude::*;
use bevy_renet::client_connected;
use bevy_renet::renet::RenetClient;

use crate::client::network::encryption::{Nonce, SskStore};
use crate::client::world::enemy::Enemy;
use crate::client::world::player::Player;
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
        mut player_transform: Query<
            &mut Transform,
            (With<Player>, Without<MainCamera>, Without<Enemy>),
        >,
        mut camera_transform: Query<
            &mut Transform,
            (With<MainCamera>, Without<Player>, Without<Enemy>),
        >,
        mut enemy_transform: Query<
            &mut Transform,
            (With<Enemy>, Without<Player>, Without<MainCamera>),
        >,
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

                    if transform.translation.z >= 45. {
                        break;
                    }

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

                    if transform.translation.x >= 45. {
                        break;
                    }

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

                    if transform.translation.z <= -45. {
                        break;
                    }

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

                    if transform.translation.x <= -45. {
                        break;
                    }

                    transform.translation.x -= 5. * time.delta_secs();

                    let mut transform = camera_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.x -= 5. * time.delta_secs();
                }

                KeyCode::KeyL => {
                    let player_transform = player_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    for enemy_transform in &mut enemy_transform {
                        if (enemy_transform.translation.x - player_transform.translation.x).abs()
                            <= 7.
                            && (enemy_transform.translation.z - player_transform.translation.z)
                                .abs()
                                <= 7.
                        {
                            info!("Player attack!");
                        }
                    }
                }

                _ => {}
            }
        }

        let mut transform = player_transform
            .single_mut()
            .expect("Multiple Players exist.");
        if transform.translation.y <= -10. {
            transform.translation.y = 15.;
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
            FixedUpdate,
            (Self::keyboard_input).run_if(in_state(AppState::InGame)),
        );
        app.add_systems(FixedUpdate, (Self::send_ping).run_if(client_connected));
    }
}
