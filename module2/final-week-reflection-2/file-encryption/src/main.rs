use std::fs;
use std::io;
use std::env;

/// XOR-based encryption/decryption (EDUCATIONAL ONLY - NOT SECURE!)
/// In production, use proper encryption libraries like AES
fn xor_encrypt_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}

fn encrypt_file(input_path: &str, output_path: &str, key: &str) -> io::Result<()> {
    println!("Encrypting: {} -> {}", input_path, output_path);
    
    let data = fs::read(input_path)?;
    let encrypted = xor_encrypt_decrypt(&data, key.as_bytes());
    fs::write(output_path, encrypted)?;
    
    println!("Encrypted {} bytes", data.len());
    Ok(())
}

fn decrypt_file(input_path: &str, output_path: &str, key: &str) -> io::Result<()> {
    println!("Decrypting: {} -> {}", input_path, output_path);
    
    // XOR is symmetric, so encryption and decryption are the same operation
    let data = fs::read(input_path)?;
    let decrypted = xor_encrypt_decrypt(&data, key.as_bytes());
    fs::write(output_path, decrypted)?;
    
    println!("Decrypted {} bytes", data.len());
    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  file-encryption encrypt <input> <output> <key>");
    println!("  file-encryption decrypt <input> <output> <key>");
    println!();
    println!("Example:");
    println!("  file-encryption encrypt message.txt encrypted.bin mysecretkey123");
    println!("  file-encryption decrypt encrypted.bin decrypted.txt mysecretkey123");
    println!();
    println!("WARNING: This uses XOR encryption for educational purposes only.");
    println!("         For production use, implement AES-256 or similar.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 5 {
        print_usage();
        std::process::exit(1);
    }
    
    let operation = &args[1];
    let input = &args[2];
    let output = &args[3];
    let key = &args[4];
    
    if key.len() < 8 {
        eprintln!("Error: Key must be at least 8 characters long");
        std::process::exit(1);
    }
    
    let result = match operation.as_str() {
        "encrypt" => encrypt_file(input, output, key),
        "decrypt" => decrypt_file(input, output, key),
        _ => {
            eprintln!("Error: Operation must be 'encrypt' or 'decrypt'");
            print_usage();
            std::process::exit(1);
        }
    };
    
    match result {
        Ok(()) => println!("Success!"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
