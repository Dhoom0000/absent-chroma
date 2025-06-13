use bevy::prelude::*;

// The Enemy Component
#[derive(Component, Debug)]
struct Enemy {
    pub health: u32,
    pub damage: u32,
}