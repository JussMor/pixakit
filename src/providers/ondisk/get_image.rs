use ntex::web::{self, get, HttpResponse,  Error};
use ntex::web::types::{Path, Query, State};
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

use crate::AppState;

#[derive(Debug,Deserialize)]
struct ImageProcessing {
    width: Option<u32>,
    height: Option<u32>,
}


#[get("/images/{tail}*")]
async fn get_image(state: State<AppState>, tail: Path<String>, params: Query<ImageProcessing>) ->  Result<HttpResponse, Error>  {

    let tail_path = tail.into_inner();
    let width = params.width;
    let height = params.height;

    println!("Tail path: {}", &tail_path);
    println!("Width: {:?}", &params);
        // Construct the base URL
    let mut base_url = format!("/images/{}", &tail_path);

    // Construct the query string
    let mut query_params = vec![];
    if let Some(w) = width {
        query_params.push(format!("width={}", w));
    }
    if let Some(h) = height {
        query_params.push(format!("height={}", h));
    }

    // Append query parameters to the base URL if they exist
    if !query_params.is_empty() {
        base_url.push('?');
        base_url.push_str(&query_params.join("&"));
    }

    // Check if the image is already in the cache
    if let Some(cached_image) = state.cache.get(&base_url).await {
        println!("Serving cached image.");
        return Ok(HttpResponse::Ok().content_type("image/webp").body(cached_image));
    }

    let full_path = PathBuf::from("ondisk_storage").join(&tail_path);

    if full_path.exists() && full_path.is_file() {
        let mime_type = mime_guess::from_path(&full_path).first_or_octet_stream();
        let file_content = fs::read(&full_path).map_err(|e| {
            web::error::ErrorInternalServerError(format!("Failed to read file: {}", e))
        })?;

        // Store the processed image in the cache
        state.cache.insert(base_url.clone(), file_content.clone()).await;

        Ok(HttpResponse::Ok().content_type(mime_type.to_string()).body(file_content))
    } else {
        Err(web::error::ErrorNotFound("File not found").into())
    }
}