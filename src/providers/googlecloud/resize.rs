use ntex::web::{get, HttpResponse, Responder};


#[get("/images")]
async fn subroute() -> impl Responder {
    HttpResponse::Ok().body("Hello, on disk subroute without param")
}


#[get("/img")]
async fn sub() -> impl Responder {
    HttpResponse::Ok().body("Hello, ")
}