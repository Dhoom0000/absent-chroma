use bevy::prelude::*;

use crate::client::{AppState, PreviousAppState, ui::UiLabelType};

pub fn listen_ui_input(
    query: Query<(&UiLabelType, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
    mut event_writer: MessageWriter<AppExit>,
    mut app_state_log: ResMut<PreviousAppState>,
) {
    for (label, interaction) in query.iter() {
        if *interaction == Interaction::Pressed {
            match *label {
                UiLabelType::Play => {
                    app_state_log.0 = Some(AppState::MainMenu);
                    commands.set_state(AppState::Load);
                }

                UiLabelType::Connect => {
                    app_state_log.0 = Some(AppState::MainMenu);
                    commands.set_state(AppState::ConnectToServer);
                }

                UiLabelType::Exit => {
                    event_writer.write(AppExit::Success);
                }
            }
        }
    }
}
