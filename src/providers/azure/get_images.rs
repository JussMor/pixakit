use futures_util::StreamExt;
use ntex::web::{self, get, HttpResponse, Error};
use ntex::web::types::{Path, State};
use crate::providers::azure::AzureStorageError;
use crate::AppState;
use ntex_bytes::BytesMut;

#[get("/images/{name}")]
async fn get_images(state: State<AppState>, name: Path<String>) -> Result<HttpResponse, Error> {
    let azure = state.get_blob_client("pixakit", &name);

    let mut stream = azure.get().into_stream();


    let mut body = BytesMut::new();

    while let Some(item) = stream.next().await {
        match item {
            Ok(response) => {
                let bytes = response.data.collect().await.map_err(|e| {
                    Error::from(AzureStorageError(e.into()))
                })?;
                body.extend_from_slice(&bytes);
            },
            Err(err) => {
                return Err(Error::from(AzureStorageError(err)));
            }
        }
    }

    Ok(HttpResponse::Ok().content_type("image/webp").body(body))
}
