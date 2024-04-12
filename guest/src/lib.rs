#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use ed25519_compact::PublicKey;

#[jolt::provable(stack_size = 1830066210)]
fn verify_eddsa(public_key:[u8;32], message:[u8;32], sig_1:[u8;32],sig_2:[u8;32]) -> bool {

    let mut signature = [0u8;64];

    for i in 0..32 {
        signature[i] = sig_1[i];
        signature[i+32] = sig_2[i];
    }

    let public_key = PublicKey::from_slice(&public_key).unwrap();
    let signature = ed25519_compact::Signature::from_slice(&signature).unwrap();
    public_key.verify(&message, &signature).is_ok()
}
