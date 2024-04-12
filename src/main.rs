use ed25519_compact::KeyPair;



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


    let (prove_eddsa, verify_eddsa) = guest::build_verify_eddsa();

    println!("public_key: {:?}", public_key.len());
    println!("signature: {:?}", signature.len());
    println!("message: {:?}", message);

    let (output, proof) = prove_eddsa(public_key, padded_message, (sig_1, sig_2));




    let is_valid = verify_eddsa(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);




    
}
