use ntex::web::{ServiceConfig, scope};

use super::upload;
use super::get_images;


pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/azure")
                .service(get_images::get_images)
                .service(upload::upload));
}