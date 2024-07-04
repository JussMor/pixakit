use ntex::web::{ types::{Json, Query}, get, post,  HttpResponse};
use serde::{Deserialize, Serialize};



#[derive(Deserialize)]
struct OnDiskQuery {
    param: Option<String>,
}


#[derive(Deserialize)]
struct GoogleCloudQuery {
    param: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct OnDiskPayload {
    key: String,
    value: String,
}

#[get("/on-disk")]
async fn on_disk_subroute_get_handler(query: Query<OnDiskQuery>) -> HttpResponse {
    if let Some(param) = query.param.clone() {
        HttpResponse::Ok().body(format!("Hello, on disk subroute with param: {}", param))
    } else {
        HttpResponse::Ok().body("Hello, on disk subroute without param")
    }
}

#[post("/on-disk")]
async fn on_disk_subroute_post_handler(payload: Json<OnDiskPayload>) -> HttpResponse {
    let payloads = payload.into_inner();
    HttpResponse::Ok().json(&payloads)
}

#[get("/on-disk-cloud")]
async fn on_google_cloud_subroute_handler(query: Query<GoogleCloudQuery>) -> HttpResponse {
    if let Some(param) = query.param.clone() {
        HttpResponse::Ok().body(format!("Hello, Google Cloud subroute with param: {}", param))
    } else {
        HttpResponse::Ok().body("Hello, Google Cloud subroute without param")
    }
}
