use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use pbkdf2::{pbkdf2, hmac::Hmac};
use sha2::Sha256;
use rand::{rngs::OsRng, RngCore};

// AES-256 CBC mode with PKCS7 padding
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

const PBKDF2_ITERATIONS: u32 = 100_000;
const KEY_SIZE: usize = 32; // 32 bytes for AES-256 key
const IV_SIZE: usize = 16; // 16 bytes for AES-CBC IV

// Derive a key from the password using PBKDF2
fn derive_key_from_password(password: &str, salt: &[u8]) -> [u8; KEY_SIZE] {
    let mut key = [0u8; KEY_SIZE]; // Buffer for the key
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
    key
}

// Encrypt the plaintext using a password-derived key
pub fn encrypt_with_password(plaintext: &str, password: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    // Generate a random salt for key derivation
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    // Generate a random IV for AES-CBC
    let mut iv = [0u8; IV_SIZE];
    OsRng.fill_bytes(&mut iv);

    // Derive the encryption key from the password and salt
    let key = derive_key_from_password(password, &salt);

    // Create AES-CBC cipher
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    // Encrypt the plaintext
    let ciphertext = cipher.encrypt_vec(plaintext.as_bytes());

    // Return the salt, IV, and ciphertext
    (salt.to_vec(), iv.to_vec(), ciphertext)
}

// Decrypt the ciphertext using a password-derived key
pub fn decrypt_with_password(ciphertext: &[u8], password: &str, salt: &[u8], iv: &[u8]) -> String {
    // Derive the encryption key from the password and salt
    let key = derive_key_from_password(password, &salt);

    // Create AES-CBC cipher
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    // Decrypt the ciphertext
    let decrypted_text = cipher.decrypt_vec(ciphertext).expect("decryption failure!");

    // Return the decrypted text as a UTF-8 string
    String::from_utf8(decrypted_text).expect("utf8 error")
}
