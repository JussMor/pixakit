use dotenv::dotenv;
use std::env;
use std::fs;
use std::path::PathBuf;
use ntex::web::{self, types::{Json, Query}, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

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


// Define a struct to represent query parameters for "ondisk/subroute"
// Define a struct to represent the query parameters for "ondisk/subroute"
#[derive(Deserialize)]
struct OnDiskQuery {
    param: Option<String>,
}

// Define a struct to represent the query parameters for "googlecloud/subroute"
#[derive(Deserialize)]
struct GoogleCloudQuery {
    param: Option<String>,
}

// Define a struct to represent the JSON payload for the POST request
#[derive(Deserialize, Serialize)]
struct OnDiskPayload {
    key: String,
    value: String,
}

// Handler for "ondisk/subroute" GET request
async fn on_disk_subroute_get_handler(query: Query<OnDiskQuery>) -> HttpResponse {
    if let Some(param) = query.param.clone() {
        HttpResponse::Ok().body(format!("Hello, on disk subroute with param: {}", param))
    } else {
        HttpResponse::Ok().body("Hello, on disk subroute without param")
    }
}

// Handler for "ondisk/subroute" POST request
async fn on_disk_subroute_post_handler(payload: Json<OnDiskPayload>) -> HttpResponse {
    //transofrm my pyaload in json format
    let payloads = payload.into_inner();
    HttpResponse::Ok().json(&payloads)
}

// Handler for "googlecloud/subroute" GET request
async fn on_google_cloud_subroute_handler(query: Query<GoogleCloudQuery>) -> HttpResponse {
    if let Some(param) = query.param.clone() {
        HttpResponse::Ok().body(format!("Hello, Google Cloud subroute with param: {}", param))
    } else {
        HttpResponse::Ok().body("Hello, Google Cloud subroute without param")
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
            // Define "ondisk" routes
            .service(
                web::scope("/ondisk")
                    .route("", web::get().to(|| async { HttpResponse::Ok().body("Hello, on disk!") }))
                    .route("/subroute", web::get().to(on_disk_subroute_get_handler))
                    .route("/subroute", web::post().to(on_disk_subroute_post_handler)),
            )
            // Define "googlecloud" routes
            .service(
                web::scope("/googlecloud")
                    .route("", web::get().to(|| async { HttpResponse::Ok().body("Hello, Google Cloud!") }))
                    .route("/subroute", web::get().to(on_google_cloud_subroute_handler)),
            )
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await

}
