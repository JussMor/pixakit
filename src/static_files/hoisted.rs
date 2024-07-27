use std::path::PathBuf;
use ntex::web::{ self, get, HttpResponse, HttpRequest};


#[get("{filename}*")]
async fn astro_files(req: HttpRequest) -> Result<HttpResponse, web::Error> {

    let path: PathBuf = req.match_info().query("filename").parse().unwrap();


    let file_path = format!("./apps/pixakit.ui/dist/_astro/{}", path.display());

    if let Ok(contents) = std::fs::read(&file_path) {
        let mime_type = mime_guess::from_path(&file_path).first_or_octet_stream();
        Ok(HttpResponse::Ok()
            .content_type(mime_type.as_ref())
            .body(contents))
    } else {
        Ok(HttpResponse::NotFound().body("File not found"))
    }
}