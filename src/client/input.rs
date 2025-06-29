use bevy::prelude::*;

use crate::client::{AppState, ui::MainMenu};

pub struct Plugin;

impl Plugin {
    fn handle_keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
        // Handle user input
        for key in keyboard_input.get_just_pressed() {
            match key {
                // Toggle menu visibility if user presses 'Esc' key
                KeyCode::Escape => {
                    commands.set_state(AppState::MainMenu);
                }
                _ => {}
            }
        }
    }
}

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::handle_keyboard_input);
    }
}
