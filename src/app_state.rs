use std::sync::Arc;
use moka::future::Cache;
use azure_storage_blobs::prelude::ClientBuilder;
use aws_sdk_s3::Client as Aws;
use google_cloud_storage::client::Client;


#[derive(Clone)]
pub struct AppState {
    pub cache: Arc<Cache<String, Vec<u8>>>,
    pub azure_client: Arc<ClientBuilder>,
    pub g_client: Arc<Client>,
    pub aws_client: Arc<Aws>
}

impl AppState {
    pub fn get_blob_client(&self, container: &str, blob_name: &str) -> azure_storage_blobs::prelude::BlobClient {
        <ClientBuilder as Clone>::clone(&self.azure_client).blob_client(container, blob_name)
    }
}
