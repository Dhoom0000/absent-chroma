use std::collections::HashMap;

use bevy::ecs::resource::Resource;

#[derive(Resource, Default)]
pub enum KEMClientKey {
    #[default]
    Pending,
    SharedSecret(Box<[u8; 32]>),
}

#[derive(Resource, Default)]
pub struct KEMServerState {
    pub decaps_key: HashMap<u64, Box<[u8; 1632]>>,
    pub shared_secrets: HashMap<u64, Box<[u8; 32]>>, // client_id -> ssk
}

impl std::fmt::Debug for KEMServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KEMServerState {{ decaps_keys: {}, shared_secrets: [{} clients] }}",
            self.decaps_key.len(),
            self.shared_secrets.len()
        )
    }
}
