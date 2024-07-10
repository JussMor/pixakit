use futures_util::StreamExt;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use ntex::web::{ self, get, HttpResponse, Error};
use ntex::web::types::{Path, State, Query};
use serde::Deserialize;
use magick_rust::{MagickWand, magick_wand_genesis};
use std::sync::Once;



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

  let object_from_aws = state.aws_client.list_objects_v2().prefix("").bucket("pixakit".to_string());

    let req = object_from_aws.send().await;

    let keys = req
            .iter()
            .filter_map(|o| o.key_count.as_ref())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

    println!("key {:?}", keys);

  Ok(HttpResponse::Ok().body("File uploaded successfully"))
}