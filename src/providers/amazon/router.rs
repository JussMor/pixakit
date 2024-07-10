use ntex::web::{ServiceConfig, scope};

use super::get_image;



pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/amazon")
                .service(get_image::get_images));
}