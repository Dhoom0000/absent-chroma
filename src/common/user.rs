use std::collections::HashMap;

use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub enum UserLogin{
    NotLoggedIn,
    IsLoggedIn {
        username:[u8;256],
        email:[u8;256],
        password:[u8;256],
    }
}

impl Default for UserLogin {
    fn default() -> Self {
        UserLogin::NotLoggedIn
    }
}


#[derive(Resource,Default)]
pub struct ConnectedUsers(pub HashMap<u64,[u8;256]>);