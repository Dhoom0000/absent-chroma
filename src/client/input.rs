use bevy::prelude::*;

use crate::client::ui::MainMenu;

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut menu_query: Query<&mut Visibility, With<MainMenu>>,
) {
    // Handle user input
    for key in keyboard_input.get_just_pressed() {
        match key {
            // Toggle menu visibility if user presses 'Esc' key
            KeyCode::Escape => {
                let mut visibility = menu_query
                    .single_mut() // get the query
                    .expect("Couldn't query the Main Menu.");
                visibility.toggle_visible_hidden(); // toggle visibility
            }
            _ => {}
        }
    }
}
