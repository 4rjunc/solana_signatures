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

    fn sign_message(&self, message: &[u8]) -> Vec<u8> {
        let secret_bytes = self.keypair.secret().to_bytes();
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let dalek_signature: DalekSignature = signing_key.sign(message);

        dalek_signature.to_bytes().to_vec()
    }

    fn verify_signature(&self, message: &[u8], signature_bytes: &[u8]) -> bool {
        let pubkey = &self.keypair.pubkey().to_bytes();
        let verifying_key = VerifyingKey::from_bytes(&pubkey).expect("Invalid public key");

        let signature_array: [u8; 64] = signature_bytes
            .try_into()
            .expect("Signature bytes must be exactly 64 bytes long");

        let signature = DalekSignature::from_bytes(&signature_array);

        verifying_key.verify(message, &signature).is_ok()
    }
}

fn main() {
    let signer = MessageSigner::new();
    let message = b"Who Am I";
    let signature_bytes = signer.sign_message(message);
    let is_valid = signer.verify_signature(message, &signature_bytes);

    println!("Signature Valid: {}", is_valid);
    println!("Raw Signature Bytes: {:?}", signature_bytes);
}
