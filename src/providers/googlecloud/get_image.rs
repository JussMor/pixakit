use futures_util::StreamExt;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use ntex::web::{ self, get, HttpResponse, Error};
use ntex::web::types::{Path, State, Query};
use serde::Deserialize;
use magick_rust::{MagickWand, magick_wand_genesis};
use std::sync::Once;



use crate::providers::googlecloud::GoogleStorageError;
use crate::AppState;
use ntex_bytes::BytesMut;

#[derive(Debug,Deserialize)]
struct PathParams {
  name: String,
  container: String,
}


#[derive(Debug,Deserialize)]
struct ImageProcessing {
    width: Option<usize>,
    height: Option<usize>,
}


static START: Once = Once::new();

#[get("/images/{container}/{name}")]
async fn get_images(state: State<AppState>, path: Path<PathParams>, params: Query<ImageProcessing>) -> Result<HttpResponse, Error> {
    
    START.call_once(|| {
        magick_wand_genesis();
    });

    let container = &path.container;
    let name = &path.name;
    let width = params.width;
    let height = params.height;

    let mut base_url = format!("/images/{}/{}", container, name);
    let mut query_params = vec![];
    if let Some(w) = width {
        query_params.push(format!("width={}", w));
    }
    if let Some(h) = height {
        query_params.push(format!("height={}", h));
    }
    if !query_params.is_empty() {
        base_url.push('?');
        base_url.push_str(&query_params.join("&"));
    }

    // Check if the processed image is in the cache
    if let Some(cached_image) = state.cache.get(&base_url).await {
        return Ok(HttpResponse::Ok().content_type("image/webp").body(cached_image));
    }


    let result = state.g_client.download_streamed_object(
        &GetObjectRequest {
            bucket: container.to_string(),
            object: name.to_string(),
            ..Default::default()
        },
        &Range::default(),
    ).await;

    let mut body = BytesMut::new();

    match result {
        Ok(mut stream) => {
            while let Some(item) = stream.next().await {
                match item {
                    Ok(bytes) => {
                        body.extend_from_slice(&bytes);
                    }
                    Err(err) => {
                        return Err(Error::from(GoogleStorageError(err)));
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error downloading image: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Failed to download image"));
        }
    }

        let mut wand = MagickWand::new();
    let file_content = body.to_vec();

    if let Err(err) = wand.read_image_blob(&file_content) {
        eprintln!("Failed to read image: {:?}", err);
        return Ok(HttpResponse::InternalServerError().finish());
    }

    if let (Some(w), Some(h)) = (width, height) {
        if let Err(err) = wand.resize_image(w, h, magick_rust::FilterType::Lanczos) {
            eprintln!("Failed to resize image: {:?}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    }

    wand.set_image_format("WEBP").map_err(|e| {
        eprintln!("Failed to set image format: {:?}", e);
        web::error::ErrorInternalServerError("Failed to set image format")
    })?;

    let processed_image_data = wand.write_image_blob("WEBP").map_err(|e| {
        eprintln!("Failed to write image blob: {:?}", e);
        web::error::ErrorInternalServerError("Failed to write image blob")
    })?;

    // Store the processed image in the cache
    state.cache.insert(base_url.clone(), processed_image_data.clone()).await;

    Ok(HttpResponse::Ok().content_type("image/webp").body(processed_image_data))
}