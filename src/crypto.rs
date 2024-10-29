use aead::{ Aead, KeyInit, OsRng };
use aead::generic_array::typenum::U32; // Assuming a 256-bit key size (32 bytes)
use aes_gcm::{ Aes256Gcm, Key, Nonce };
use aes_gcm::aead::generic_array::GenericArray;
use std::fs::File;
use std::io::{ self, Read, Write, BufReader, BufWriter };
use std::path::Path;

pub fn encrypt(plaintext: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(nonce); // 96-bits; unique per message
    cipher.encrypt(nonce, plaintext).expect("encryption failure!")
}

pub fn decrypt(ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(nonce); // 96-bits; unique per message
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
        let encrypted_chunk = encrypt(&buffer[..bytes_read], key, nonce);
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

pub fn generate_key_nonce() -> (Vec<u8>, Vec<u8>) {
    let key: &[u8] = GenericArray::generate(|_| OsRng); // Generate a random key
    let nonce = GenericArray::generate(|_| OsRng); // Generate a random nonce
    (key.to_vec(), nonce.to_vec())
}
