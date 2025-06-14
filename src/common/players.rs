use bevy::prelude::*;


// The Player Component
#[derive(Component, Debug, Default)]
struct Player {
    pub uuid: u64,
    pub name: String,
    pub level: u32,
    pub exp: u64,
    pub health: u32,
    pub speed: f32,
    pub max_height: u32,
}

// A Team Player Component
#[derive(Component, Debug, Default)]
struct TeamPlayer {
    pub team_id: String,
    pub team_name: String,
}


// The Human Deaf Player
#[derive(Component,Debug, Default)]
struct Gray{
    pub armor: u32,
}

// The Robot Blind Player
#[derive(Component,Debug, Default)]
struct Note{
    pub mana: u32,
}
