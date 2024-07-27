use std::path::PathBuf;
use ntex::web::{self, get, HttpResponse, HttpRequest};
use mime_guess::from_path;

#[get("{fonts}*")]
async fn serve_fonts(req: HttpRequest) -> Result<HttpResponse, web::Error> {
    let path: PathBuf = req.match_info().query("fonts").parse().unwrap();
    let file_path = format!("./apps/pixakit.ui/dist/fonts/{}", path.display());
    
    if let Ok(contents) = std::fs::read(&file_path) {
        let mime_type = from_path(&file_path).first_or_octet_stream();
        Ok(HttpResponse::Ok()
            .content_type(mime_type.as_ref())
            .body(contents))
    } else {
        Ok(HttpResponse::NotFound().body("Font file not found"))
    }
}