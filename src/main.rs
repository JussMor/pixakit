use dotenv::dotenv;
use ntex_cors::Cors;
use std::env;
use std::fs;
use std::path::PathBuf;
use ntex::web::{self, types::{Json, Query}, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

mod providers;

#[derive(Debug)]
enum Provider {
    GoogleCloud,
    Azure,
    OnDisk,
}

#[derive(Debug)]
struct Storage {
    provider: Provider,
    bucket: String,
    region: String,
    path: Option<PathBuf>,
}

fn init_ondisk(storage: &Storage) {
    println!("Initializing on-disk storage");
    
    // Example: Create a directory for on-disk storage if it doesn't exist
    if let Some(path) = &storage.path {
        if !path.exists() {
            match fs::create_dir_all(path) {
                Ok(_) => println!("Created directory: {}", path.display()),
                Err(e) => println!("Failed to create directory: {}", e),
            }
        } 
        // else {
        //     println!("Directory already exists: {}", path.display());
        // }
    }
}

fn init_google_cloud() {
    println!("Initializing Google Cloud storage");
}

fn init_azure() {
    println!("Initializing Azure storage");
}

fn init_provider(storage: &Storage) {
    match storage.provider {
        Provider::GoogleCloud => init_google_cloud(),
        Provider::Azure => init_azure(),
        Provider::OnDisk => init_ondisk(storage),
    }
}



#[ntex::main]
async fn main() -> std::io::Result<()>  {
    dotenv().ok();

    let storage_provider = env::var("STORAGE_PROVIDER").unwrap_or_else(|_| "ONDISK".to_string());

    let current_dir = env::current_dir().expect("Failed to get current directory");

    let provider = match storage_provider.to_lowercase().as_str() {
        "googlecloud" => Storage {
            provider: Provider::GoogleCloud,
            bucket: "mybucket".to_string(),
            region: "us-west-1".to_string(),
            path: None,
        },
        "azure" => Storage {
            provider: Provider::Azure,
            bucket: "mybucket".to_string(),
            region: "us-west-1".to_string(),
            path: None,
        },
        "ondisk" => Storage {
            provider: Provider::OnDisk,
            bucket: "".to_string(),
            region: "".to_string(),
            path: Some(current_dir.join("ondisk_storage")), // Specify a directory within the current directory
        },
        _ => Storage {
            provider: Provider::OnDisk,
            bucket: "".to_string(),
            region: "".to_string(),
            path: Some(current_dir.join("ondisk_storage")), // Default to on-disk storage with a specified directory
        },
    };

    init_provider(&provider);

    HttpServer::new(|| {
        App::new()
        .wrap(Cors::new().finish())
            .configure(providers::ondisk::router::config)
            .configure(providers::googlecloud::router::config)
            .configure(providers::azure::router::config)
            .configure(providers::amazon::router::config)
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await

}
