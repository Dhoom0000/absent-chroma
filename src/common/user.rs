use std::collections::HashMap;

use bevy::ecs::resource::Resource;

#[derive(Resource, Default)]
pub enum UserLogin {
    #[default]
    NotLoggedIn,
    IsLoggedIn {
        username: Box<[u8; 256]>,
        email: Box<[u8; 256]>,
        password: Box<[u8; 256]>,
    },
}

#[derive(Resource, Default)]
pub struct ConnectedUsers(pub HashMap<u64, [u8; 256]>);
