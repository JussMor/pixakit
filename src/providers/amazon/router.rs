use ntex::web::{ServiceConfig, scope};

use super::resize;



pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/amazon")
                .service(resize::on_disk_subroute_get_handler)
                .service(resize::on_google_cloud_subroute_handler))
                .service(resize::on_disk_subroute_post_handler);
}