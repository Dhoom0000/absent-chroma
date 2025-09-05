use bevy::prelude::*;

use crate::client::{AppState, PreviousAppState};

pub struct InputPlugin;

#[derive(Resource, Debug)]
pub struct Controls {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub strafe_left: KeyCode,
    pub strafe_right: KeyCode,
    pub rise: Option<KeyCode>,
    pub fall: Option<KeyCode>,
}

impl Controls {
    pub fn new_gray() -> Self {
        Controls {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            strafe_left: KeyCode::KeyA,
            strafe_right: KeyCode::KeyD,
            rise: None,
            fall: None,
        }
    }

    pub fn new_note() -> Self {
        Controls {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            strafe_left: KeyCode::KeyA,
            strafe_right: KeyCode::KeyD,
            rise: Some(KeyCode::KeyZ),
            fall: Some(KeyCode::KeyX),
        }
    }
}

impl InputPlugin {
    fn handle_keyboard_input(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut commands: Commands,
        app_state: Res<State<AppState>>,
        mut prev_state: ResMut<PreviousAppState>,
    ) {
        // Handle user input
        for key in keyboard_input.get_just_pressed() {
            match key {
                // Toggle menu visibility if user presses 'Esc' key
                KeyCode::Escape => match &prev_state.0 {
                    None => {}

                    Some(state) => {
                        commands.set_state(*state);
                        prev_state.0 = Some(*app_state.get());
                    }
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
