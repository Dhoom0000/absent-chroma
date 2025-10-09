use bevy::prelude::*;

use crate::client::world::player;
use crate::client::{AppState, PreviousAppState};

pub struct ControlsPlugin;

impl ControlsPlugin {
    fn keyboard_input(
        keyboard: Res<ButtonInput<KeyCode>>,
        app_state: Res<State<AppState>>,
        mut commands: Commands,
        mut player_transform: Query<&mut Transform, With<player::Player>>,
        time: Res<Time>,
    ) {
        for key_pressed in keyboard.get_pressed() {
            match key_pressed {
                KeyCode::Escape => {
                    if *app_state.get() == AppState::InGame {
                        commands.insert_resource(PreviousAppState(Some(AppState::InGame)));
                        commands.set_state(AppState::MainMenu);
                    }
                }

                KeyCode::KeyW => {
                    let mut transform = player_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.z += 5. * time.delta_secs();
                }

                KeyCode::KeyA => {
                    let mut transform = player_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.x += 5. * time.delta_secs();
                }

                KeyCode::KeyS => {
                    let mut transform = player_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.z -= 5. * time.delta_secs();
                }

                KeyCode::KeyD => {
                    let mut transform = player_transform
                        .single_mut()
                        .expect("Multiple Players exist.");

                    transform.translation.x -= 5. * time.delta_secs();
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
    }
}
