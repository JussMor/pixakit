use aws_sdk_s3::config::Credentials;
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::ClientBuilder;
use dotenv::dotenv;
use ntex_cors::Cors;
use std::env;
use ntex::web::{ App, HttpServer};
use moka::future::Cache;
use std::sync::Arc;
use aws_sdk_s3::{config::Region, Client as Aws};
use google_cloud_storage::client::{ClientConfig, Client};
use google_cloud_storage::client::google_cloud_auth::credentials::CredentialsFile;
// use ntex_files as files;


mod providers;
mod app_state;
use app_state::AppState;
mod static_files;



#[ntex::main]
async fn main() -> std::io::Result<()>  {
    dotenv().ok();

    // OnDisk
    
    let storage_provider = env::var("STORAGE_PROVIDER").unwrap_or_else(|_| "ONDISK".to_string());
    let cache = Arc::new(Cache::new(100));
    if storage_provider.to_lowercase() == "ondisk" {
        providers::ondisk::init::init_ondisk_storage();
    }

    // Azure

    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let azure_client = Arc::new(ClientBuilder::new(account, storage_credentials));

    // Google Cloud

    let key_file = std::fs::read_to_string("credentials/pixakit-key.json").expect("Failed to read key file");
    let credentials = CredentialsFile::new_from_str(&key_file).await.unwrap();

    let config = ClientConfig::default().with_credentials(credentials).await.unwrap();
    let g_client = Arc::new(Client::new(config));

    // Amazon S3

    let key_id = env::var("AWS_ACCESS_KEY").expect("Missing AWS_ACCESS_KEY");
	let key_secret = env::var("AWS_SECRET_ACCESS_KEY").expect("Missing AWS_SECRET_ACCESS_KEY");

    let sdk_config = aws_config::load_from_env().await;
    let cred = Credentials::new(key_id, key_secret, None, None, "loaded-from-custom-env");
	let region = Region::new("us-east-1".to_string());
    let conf_builder = aws_sdk_s3::config::Builder::from(&sdk_config)
        .region(region)
        .credentials_provider(cred)
        .build();

	let aws_client = Arc::new(Aws::from_conf(conf_builder));

    // App State
    let state = AppState {
        cache,
        azure_client,
        g_client,
        aws_client
    };

    HttpServer::new(move || {
        App::new()
        .state(state.clone())
        .wrap(Cors::new().allowed_origin("*").finish())
            .configure(providers::ondisk::router::config)
            .configure(providers::googlecloud::router::config)
            .configure(providers::azure::router::config)
            .configure(providers::amazon::router::config)
            .configure(static_files::router::config)
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await

}
