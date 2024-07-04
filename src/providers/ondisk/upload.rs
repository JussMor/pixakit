
use futures_util::{StreamExt, TryStreamExt};
use ntex::web::{self, post, Error, HttpResponse};
use ntex_multipart::Multipart;
use serde::Deserialize;
use std::io::Write;
use std::path::PathBuf;
use mime;

#[derive(Deserialize)]
struct UploadParams {
    path: Option<String>,
}


fn extract_filename(headers: &ntex::http::HeaderMap) -> Option<String> {
    if let Some(content_disposition) = headers.get(ntex::http::header::CONTENT_DISPOSITION) {
        if let Ok(content_disposition) = content_disposition.to_str() {
            if let Some(filename) = content_disposition.split("filename=").nth(1) {
                let filename = filename.trim_matches('"');
                return Some(filename.to_string());
            }
        }
    }
    None
}

async fn save_file(mut field: ntex_multipart::Field, base_path: PathBuf) -> Result<(), Error> {
    // Extract filename and create the full file path
    let headers = field.headers();
    let filename = extract_filename(headers).ok_or_else(|| {
        web::error::ErrorBadRequest("Filename not found in content disposition")
    })?;
    let filepath = base_path.join(filename);

    // Check if the file is an image
    let content_type = field.content_type();
    if content_type.type_() != mime::IMAGE {
        return Err(web::error::ErrorBadRequest("Only image files are allowed").into());
    }

    // Create the file and write the contents
    let mut f = std::fs::File::create(filepath)?;
    while let Some(chunk) = field.next().await {
        let data = chunk?;
        f.write_all(&data)?;
    }

    Ok(())
}


#[post("/upload")]
async fn upload(
    query: web::types::Query<UploadParams>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let base_path = if let Some(ref path) = query.path {
        PathBuf::from("ondisk_storage").join(path)
    } else {
        PathBuf::from("ondisk_storage")
    };

    // Ensure the base directory exists
    if let Err(e) = std::fs::create_dir_all(&base_path) {
        return Err(web::error::ErrorInternalServerError(format!("Failed to create directory: {}", e)).into());
    }

    // Process each field in the multipart stream
    while let Some(item) = payload.try_next().await? {
        let field = item;
        save_file(field, base_path.clone()).await?;
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}