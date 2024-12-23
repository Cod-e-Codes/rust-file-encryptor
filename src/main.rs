use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES256-GCM mode
use rand_core::RngCore;
use std::fs::{self, File};
use std::io::{self, Write};

fn main() {
    println!("File Encryptor");

    loop {
        println!("\nChoose an option:");
        println!("1. Encrypt a file");
        println!("2. Decrypt a file");
        println!("3. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => encrypt_file(),
            "2" => decrypt_file(),
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn encrypt_file() {
    let file_path = get_input("Enter the path of the file to encrypt: ");
    let content = fs::read(&file_path).expect("Failed to read the file.");

    // Generate a random 256-bit key
    let key = Key::<Aes256Gcm>::from_slice(&[0u8; 32]);
    let cipher = Aes256Gcm::new(&key);

    // Generate a random 96-bit nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, content.as_ref())
        .expect("Encryption failed");

    let mut encrypted_file =
        File::create(format!("{}.enc", file_path)).expect("Failed to create encrypted file.");
    encrypted_file
        .write_all(&nonce_bytes)
        .expect("Failed to write nonce.");
    encrypted_file
        .write_all(&ciphertext)
        .expect("Failed to write ciphertext.");

    println!("File encrypted and saved as {}.enc", file_path);
}

fn decrypt_file() {
    let file_path = get_input("Enter the path of the file to decrypt: ");
    let encrypted_content = fs::read(&file_path).expect("Failed to read the file.");

    // Assume the first 12 bytes are the nonce
    let nonce = Nonce::from_slice(&encrypted_content[..12]);
    let ciphertext = &encrypted_content[12..];

    // Use the same key as used for encryption
    let key = Key::<Aes256Gcm>::from_slice(&[0u8; 32]); // Replace with the actual key
    let cipher = Aes256Gcm::new(&key);

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("Decryption failed");

    let mut decrypted_file =
        File::create(file_path.replace(".enc", ".dec")).expect("Failed to create decrypted file.");
    decrypted_file
        .write_all(&plaintext)
        .expect("Failed to write decrypted content.");

    println!(
        "File decrypted and saved as {}",
        file_path.replace(".enc", ".dec")
    );
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
