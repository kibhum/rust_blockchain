use bs58;
use p256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
use rand_core::OsRng;
use ripemd160::{Digest as RipemdDigest, Ripemd160};
use sha2::{Digest, Sha256};

pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);
        let mut address = String::new();
        let mut gen_address = || {
            let key_points = verifying_key.to_encoded_point(false);
            if let (Some(x), Some(y)) = (key_points.x(), key_points.y()) {
                let mut pub_key_bytes = Vec::with_capacity(x.len() + y.len());
                pub_key_bytes.extend_from_slice(x);
                pub_key_bytes.extend_from_slice(y);
                // sha256 on the public key
                let hash = Sha256::digest(&pub_key_bytes);
                // Ripemd160 on the hash
                let mut hasher = Ripemd160::new();
                hasher.update(&hash);
                let mut hash_result = hasher.finalize().to_vec();
                // Add byte version in front of the Ripemd160 hash result (0x00 for mainnet)
                hash_result.insert(0, 0x00);
                // Do Sha256 on the previous result
                let hash2 = Sha256::digest(&hash_result);
                // Do Sha256 on the previous result
                let hash3 = Sha256::digest(&hash2);
                // Take the first 4 bytes of the previous result as checksum
                let checksum = &hash3[0..4];
                // Add the checksum to the end of the extended ripemd160 hash result
                let full_hash = [hash_result, checksum.to_vec()].concat();
                // Base 58 encoding
                address = bs58::encode(full_hash).into_string();
            } else {
                // Do nothing
            }
        };
        gen_address();
        Self {
            signing_key,
            verifying_key,
            address,
        }
    }

    pub fn private_key_str(&self) -> String {
        // 1. Convert private key into hex string
        hex::encode(self.signing_key.to_bytes())
    }

    pub fn public_key_str(&self) -> String {
        // 1. Getting the key points
        let key_points = self.verifying_key.to_encoded_point(false);
        if let (Some(x), Some(y)) = (key_points.x(), key_points.y()) {
            let pub_str = hex::encode(x) + hex::encode(y).as_str();
            pub_str
        } else {
            String::new()
        }
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }
}
