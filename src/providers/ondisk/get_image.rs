use ntex::web::{self, get, HttpResponse,  Error};
use ntex::web::types::{Path, Query, State};
use std::fs;
use std::path::PathBuf;
use std::sync::Once;
use serde::Deserialize;
use magick_rust::{MagickWand, magick_wand_genesis};


use crate::AppState;

#[derive(Debug,Deserialize)]
struct ImageProcessing {
    width: Option<usize>,
    height: Option<usize>,
}

static START: Once = Once::new();

#[get("/images/{tail}*")]
async fn get_image(state: State<AppState>, tail: Path<String>, params: Query<ImageProcessing>) ->  Result<HttpResponse, Error>  {

    START.call_once(|| {
        magick_wand_genesis();
    });

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
        let mut wand = MagickWand::new();
        let mime_type = mime_guess::from_path(&full_path).first_or_octet_stream();
        let file_content = fs::read(&full_path).map_err(|e| {
            web::error::ErrorInternalServerError(format!("Failed to read file: {}", e))
        })?;

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
        // Convert the image to WEBP format
        wand.set_image_format("WEBP").map_err(|e| {
            eprintln!("Failed to set image format: {:?}", e);
            web::error::ErrorInternalServerError("Failed to set image format")
        })?;

        // Get the processed image data
        let processed_image_data = wand.write_image_blob("WEBP").map_err(|e| {
            eprintln!("Failed to write image blob: {:?}", e);
            web::error::ErrorInternalServerError("Failed to write image blob")
        })?;

        // Store the processed image in the cache
        state.cache.insert(base_url.clone(), processed_image_data.clone()).await;

        Ok(HttpResponse::Ok().content_type(mime_type.to_string()).body(processed_image_data))
    } else {
        Err(web::error::ErrorNotFound("File not found").into())
    }
}