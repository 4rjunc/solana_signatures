use std::str::FromStr;

use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
};

// Signing function
fn sign_message(message: &[u8], keypair: &Keypair) -> String {
    let signature = keypair.sign_message(message);

    signature.to_string()
}

// Verify Signature
fn verify_signature(
    message: &[u8],
    signature_str: &str,
    public_key_str: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let signature = Signature::from_str(signature_str)?;

    let public_key = Pubkey::from_str(public_key_str)?;

    Ok(signature.verify(&public_key.as_ref(), message))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Sign a message
    let message = b"Im a retarded dev";
    let keypair = Keypair::new();

    let signature = sign_message(message, &keypair);
    println!("Signature: {}", signature);

    // Verifying
    let verification_result = verify_signature(message, &signature, &keypair.pubkey().to_string())?;
    println!("Signature Verification Result: {}", verification_result);

    Ok(())
}
