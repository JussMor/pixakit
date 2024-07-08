
use ntex::web::{post, Error, HttpResponse};
use ntex::web::types::State;

use crate::providers::azure::AzureStorageError;
use crate::AppState;




#[post("/upload")]
async fn upload(state: State<AppState>) -> Result<HttpResponse, Error> {
    let container = String::from("pixakit");
    let blob_name = String::from("test_blob");

    let azure_storage = state.get_blob_client(&container, &blob_name);

    match azure_storage.put_block_blob("hello world").content_type("text/plain").await {
        Ok(_) => Ok(HttpResponse::Ok().body("File uploaded successfully")),
        Err(err) => Err(Error::from(AzureStorageError(err))),
    }
}
