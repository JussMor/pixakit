use ntex::web::{self};
use ntex::web::{scope, ServiceConfig};
use ntex_files as files;

use super::hoisted;
use super::fonts;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        files::Files::new("/", "./apps/pixakit.ui/dist")
            .index_file("index.html")
            .default_handler(web::to(|| async {
                web::HttpResponse::Ok().body("404 - Not Found")
            })),
    )
    .service(scope("/_astro").service(hoisted::astro_files))
    .service( scope("/fonts").service(fonts::serve_fonts));
}
