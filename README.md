# Rust File Encryptor

A command-line application written in Rust that allows users to encrypt and decrypt files using the AES-256-GCM encryption standard. This project demonstrates basic file handling, encryption, and decryption techniques in Rust.

## Features
- **Encrypt files**: Securely encrypt any file and save it with a `.enc` extension.
- **Decrypt files**: Restore an encrypted file to its original content.
- **AES-256-GCM encryption**: Uses a robust encryption standard for security.
- **Random nonce generation**: Ensures cryptographic strength with a unique nonce for each operation.

## Requirements
- [Rust](https://www.rust-lang.org/) (version 1.65.0 or later)

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Cod-e-Codes/rust-file-encryptor.git
   cd rust-file-encryptor
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the project:
   ```bash
   cargo run
   ```

## Usage
When you run the application, you will be presented with a menu:

```
File Encryptor
Choose an option:
1. Encrypt a file
2. Decrypt a file
3. Quit
```

### Encrypt a File
1. Select `1` from the menu.
2. Enter the path to the file you want to encrypt.
3. The application will create a new encrypted file with the `.enc` extension in the same directory.

### Decrypt a File
1. Select `2` from the menu.
2. Enter the path to the encrypted file (with the `.enc` extension).
3. The application will create a new decrypted file with the `.dec` extension in the same directory.

### Quit
1. Select `3` from the menu to exit the application.

## Example Workflow
### Encrypt a File
Input:
```
Choose an option: 1
Enter the path of the file to encrypt: sample.txt
```
Output:
```
File encrypted and saved as sample.txt.enc
```

### Decrypt a File
Input:
```
Choose an option: 2
Enter the path of the file to decrypt: sample.txt.enc
```
Output:
```
File decrypted and saved as sample.txt.dec
```

## Security Notes
- The encryption key in this example is hardcoded for simplicity (`[0u8; 32]`). For real-world applications, always generate and securely manage keys.
- Ensure you use a secure method to store and retrieve keys when deploying the application in production.

## File Structure
- **`main.rs`**: Contains the program logic for encryption and decryption.

## Dependencies
- `aes-gcm`: Provides AES-GCM encryption functionality.
- `aes`: Low-level AES implementation.
- `rand_core`: Used for generating random nonces.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

## Future Enhancements
- Add support for user-provided encryption keys.
- Implement secure key storage and retrieval.
- Improve error handling and logging.

