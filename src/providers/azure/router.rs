use ntex::web::{ServiceConfig, scope};

use super::resize;



pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/azure")
                .service(resize::subroute)
                .service(resize::sub));
}