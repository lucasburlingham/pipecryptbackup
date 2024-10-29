use aead::{ Aead, KeyInit };
use aes_gcm::{ Aes256Gcm };
use std::fs::File;
use std::io::{ self, Read, Write, BufReader, BufWriter };
use std::path::Path;
use rand::rngs::OsRng;
use rand::RngCore;
use typenum::{U32, U12};

use aead::generic_array;
use generic_array::GenericArray;

// Get the key and nonse if it exists, otherwise generate a new one
pub fn get_key_nonce(key_path: &Path, nonce_path: &Path) -> (GenericArray<u8, U32>, GenericArray<u8, U12>) {
	if key_path.exists() && nonce_path.exists() {
		let (key, nonce) = load_key_nonce(key_path).expect("Failed to load key and nonce");
		(GenericArray::clone_from_slice(&key), GenericArray::clone_from_slice(&nonce))
	} else {
		let (key, nonce) = generate_key_nonce();
		store_key_nonce(&key, &nonce, key_path).expect("Failed to store key and nonce");
		(key, nonce)
	}
}



// Use the `U32` and `U12` from `typenum` as the size for key and nonce
pub fn encrypt(plaintext: &[u8], key: &GenericArray<u8, U32>, nonce: &GenericArray<u8, U12>) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key);
    cipher.encrypt(nonce, plaintext).expect("encryption failure!")
}

pub fn decrypt(ciphertext: &[u8], key: &GenericArray<u8, U32>, nonce: &GenericArray<u8, U12>) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key);
    cipher.decrypt(nonce, ciphertext).expect("decryption failure!")
}

pub fn encrypt_file(
    input_path: &Path,
    output_path: &Path,
    key: &[u8],
    nonce: &[u8]
) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    let mut buffer = [0; 4096]; // Read in chunks of 4KB
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        let encrypted_chunk = encrypt(&buffer[..bytes_read], key.into(), nonce.into());
        writer.write_all(&encrypted_chunk)?;
    }

    writer.flush()?;
    Ok(())
}

pub fn store_key_nonce(key: &[u8], nonce: &[u8], path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(key)?;
    file.write_all(nonce)?;
    Ok(())
}

pub fn load_key_nonce(path: &Path) -> io::Result<(Vec<u8>, Vec<u8>)> {
    let mut file = File::open(path)?;
    let mut key = vec![0u8; 32]; // AES-256 key size
    let mut nonce = vec![0u8; 12]; // Nonce size for AES-GCM
    file.read_exact(&mut key)?;
    file.read_exact(&mut nonce)?;
    Ok((key, nonce))
}

pub fn generate_key_nonce() -> (GenericArray<u8, U32>, GenericArray<u8, U12>) {
    let mut key = GenericArray::default(); // Default to zeroed array
    let mut nonce = GenericArray::default();

    OsRng.fill_bytes(key.as_mut()); // Fill key with random bytes
    OsRng.fill_bytes(nonce.as_mut()); // Fill nonce with random bytes

    (key, nonce)
}