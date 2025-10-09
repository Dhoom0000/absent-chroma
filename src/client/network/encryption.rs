use bevy::ecs::resource::Resource;
use fips203::{
    SharedSecretKey,
    ml_kem_512::EncapsKey,
    traits::{Encaps, SerDes},
};
use zeroize::Zeroizing;

pub fn get_ciphertext(e_key_bytes: [u8; 800]) -> (SharedSecretKey, [u8; 768]) {
    let e_key = EncapsKey::try_from_bytes(e_key_bytes).expect("Encaps key parse failed.");

    let (ssk, ct) = e_key.try_encaps().expect("Failed ssk generation.");

    let ct_bytes = ct.into_bytes();

    (ssk, ct_bytes)
}

#[derive(Resource)]
pub struct SskStore(pub Zeroizing<[u8; 32]>);

#[derive(Resource)]
pub struct Nonce(pub [u8; 12]);
