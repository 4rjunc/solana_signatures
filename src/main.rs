use ed25519_dalek::{
    Signature as DalekSignature, Signer as DalekSigner, SigningKey, Verifier, VerifyingKey,
};
use solana_sdk::signature::{Keypair, Signer};

struct MessageSigner {
    keypair: Keypair,
}

impl MessageSigner {
    fn new() -> Self {
        Self {
            keypair: Keypair::new(),
        }
    }

    fn sign_message_detailed(&self, message: &[u8]) -> Vec<u8> {
        // Convert Solana keypair's secret to ed25519_dalek signing key
        let secret_bytes = self.keypair.secret().to_bytes();
        let signing_key = SigningKey::from_bytes(&secret_bytes);

        // Sign the message using ed25519 algorithm
        let dalek_signature: DalekSignature = signing_key.sign(message);

        // Return raw signature bytes
        dalek_signature.to_bytes().to_vec()
    }

    fn verify_signature(&self, message: &[u8], signature_bytes: &[u8]) -> bool {
        // Get the public key from the keypair
        let pubkey = self.keypair.pubkey();

        // Convert public key to verifying key
        let verifying_key =
            VerifyingKey::from_bytes(&pubkey.to_bytes()).expect("Invalid public key");

        // Convert signature bytes to a fixed-size array
        let signature_array: [u8; 64] = signature_bytes
            .try_into()
            .expect("Signature bytes must be exactly 64 bytes long");

        // Convert signature array to Ed25519 signature
        let signature = DalekSignature::from_bytes(&signature_array);

        // Verify the signature
        verifying_key.verify(message, &signature).is_ok()
    }
}

fn main() {
    let signer = MessageSigner::new();

    // Example message signing
    let message = b"Solana Message Signing Demo";

    // Perform detailed signing
    let signature_bytes = signer.sign_message_detailed(message);

    // Verify the signature
    let is_valid = signer.verify_signature(message, &signature_bytes);

    println!("Signature Valid: {}", is_valid);
    println!("Raw Signature Bytes: {:?}", signature_bytes);
}
