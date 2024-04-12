use ed25519_compact::KeyPair;

use ark_bn254::Fr;

pub fn main() {
    let key_pair = KeyPair::from_seed([42u8; 32].into());
    let message = b"I love ZKPs";
    let signature = key_pair.sk.sign(message, None);
    let mut public_key = [0u8;32];
    public_key.copy_from_slice(&key_pair.pk.to_vec().as_slice());

    let mut padded_message: [u8;32]= [0;32];
    padded_message[..message.len()].copy_from_slice(message);


    let mut sig_1 = [0u8;32];
    let mut sig_2 = [0u8;32];
    sig_1.copy_from_slice(&signature[..32]);
    sig_2.copy_from_slice(&signature[32..]);

    let (program, _) = guest::preprocess_verify_eddsa();
    let (_, _, instruction_trace, _, _) =
    program.clone().trace::<Fr>();
    println!("Trace size {:?}", instruction_trace.len().next_power_of_two());




    
}
