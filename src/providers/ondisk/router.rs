use ntex::web::{ServiceConfig, scope};


use super::fs;
use super::upload;


pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/ondisk")
                .service(fs::get_path)
                .service(fs::get_files_and_folders)
                .service(upload::upload));
}