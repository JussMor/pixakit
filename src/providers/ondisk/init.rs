use std::{env, fs};

pub fn init_ondisk_storage() {
    println!("Initializing on-disk storage");

    // Example: Create a directory for on-disk storage if it doesn't exist
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let storage_path = current_dir.join("ondisk_storage");

    if !storage_path.exists() {
        match fs::create_dir_all(&storage_path) {
            Ok(_) => println!("Created directory: {}", storage_path.display()),
            Err(e) => println!("Failed to create directory: {}", e),
        }
    }
}