use bevy::ecs::resource::Resource;

use crate::common::network::UserData;

#[derive(Default, Clone, Copy, Resource)]
pub enum UserLogin {
    LoggedIn(UserData),
    #[default]
    NotLoggedIn,
}
