use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::ClientBuilder;
use dotenv::dotenv;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use ntex_cors::Cors;
use std::env;
use std::fs;
use std::path::PathBuf;
use ntex::web::{ App, HttpServer};
use moka::future::Cache;
use std::sync::Arc;


use google_cloud_storage::client::{ClientConfig, Client};
use google_cloud_storage::client::google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::http::buckets::get::GetBucketRequest;

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
    #[allow(dead_code)]
    bucket: String,
    #[allow(dead_code)]
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

#[derive(Clone)]
pub struct AppState {
    cache: Arc<Cache<String, Vec<u8>>>,
    azure_client: Arc<ClientBuilder>,
    g_storage: Arc<Client>
}

impl AppState {
    pub fn get_blob_client(&self, container: &str, blob_name: &str) -> azure_storage_blobs::prelude::BlobClient {
        <ClientBuilder as Clone>::clone(&self.azure_client).blob_client(container, blob_name)
    }
}


#[ntex::main]
async fn main() -> std::io::Result<()>  {
    dotenv().ok();

    // OnDisk
    
    let storage_provider = env::var("STORAGE_PROVIDER").unwrap_or_else(|_| "ONDISK".to_string());
    let cache = Arc::new(Cache::new(100));

    // Azure

    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let azure_client = Arc::new(ClientBuilder::new(account, storage_credentials));

    // Google Cloud

    let key_file = std::fs::read_to_string("credentials/pixakit-key.json").expect("Failed to read key file");
    let credentials = CredentialsFile::new_from_str(&key_file).await.unwrap();

    let config = ClientConfig::default().with_credentials(credentials).await.unwrap();
    let g_storage = Arc::new(Client::new(config));


    // Amazon S3



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



    let state = AppState {
        cache,
        azure_client,
        g_storage
    };

    HttpServer::new(move || {
        App::new()
        .state(state.clone())
        .wrap(Cors::new().allowed_origin("*").finish())
            .configure(providers::ondisk::router::config)
            .configure(providers::googlecloud::router::config)
            .configure(providers::azure::router::config)
            .configure(providers::amazon::router::config)
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await

}
