use bevy::prelude::*;

use crate::client::{AppState, setup::GameLoaded};

pub struct InputPlugin;

impl InputPlugin {
    fn handle_keyboard_input(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut commands: Commands,
        app_state: Res<State<AppState>>,
        is_loaded: Option<Res<GameLoaded>>,
    ) {
        // Handle user input
        for key in keyboard_input.get_just_pressed() {
            match key {
                // Toggle menu visibility if user presses 'Esc' key
                KeyCode::Escape => match app_state.get() {
                    AppState::InGame => {
                        commands.set_state(AppState::MainMenu);
                    }

                    AppState::MainMenu => {
                        if is_loaded.is_none() {
                            commands.set_state(AppState::LoadingScreen);
                        } else {
                            commands.set_state(AppState::InGame);
                        }
                    }

                    _ => {}
                },
                _ => {}
            }
        }
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::handle_keyboard_input);
    }
}
