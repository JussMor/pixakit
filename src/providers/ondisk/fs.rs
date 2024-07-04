use ntex::web::{get, HttpResponse, Responder};
use std::fs;
use std::env;
use std::path::PathBuf;
use serde::Serialize;

#[derive(Serialize)]
struct FileList {
    data: Vec<String>,
}


#[get("/get-all-files")]
async fn get_path() -> impl Responder {
    fn list_files_recursively(dir: PathBuf) -> Vec<String> {
        let mut file_paths = Vec::new();

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        file_paths.extend(list_files_recursively(path));
                    } else if let Some(filename) = path.file_name() {
                        if let Some(filename_str) = filename.to_str() {
                            file_paths.push(filename_str.to_string());
                        }
                    }
                }
            }
        }

        file_paths
    }

    let current_dir = env::current_dir();
    let mut files = Vec::new();
    
    if let Ok(current_dir) = current_dir {
        let storage_path = current_dir.join("ondisk_storage");

        if storage_path.is_dir() {
            files = list_files_recursively(storage_path);
        }
    }


   let response = FileList { data: files };
    HttpResponse::Ok().json(&response)
}



#[derive(Serialize)]
struct Folder {
    level: usize,
    folder: String,
    files: Vec<String>,
    subfolders: Vec<Folder>,
}

#[derive(Serialize)]
struct ApiResponse {
    data: Vec<Folder>,
}

#[get("/get-files-and-folders")]
async fn get_files_and_folders() -> impl Responder {
    fn list_files_and_folders_recursively(dir: PathBuf, level: usize) -> Folder {
        let mut folder_name = String::new();
        if let Some(name) = dir.file_name() {
            if let Some(name_str) = name.to_str() {
                folder_name = name_str.to_string();
            }
        }

        let mut files = Vec::new();
        let mut subfolders = Vec::new();

        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        subfolders.push(list_files_and_folders_recursively(path, level + 1));
                    } else if let Some(filename) = path.file_name() {
                        if let Some(filename_str) = filename.to_str() {
                            files.push(filename_str.to_string());
                        }
                    }
                }
            }
        }

        Folder {
            level,
            folder: folder_name,
            files,
            subfolders,
        }
    }

    let current_dir = env::current_dir();
    let mut response_data = Vec::new();
    
    if let Ok(current_dir) = current_dir {
        let storage_path = current_dir.join("ondisk_storage");

        if storage_path.is_dir() {
            let root_folder = list_files_and_folders_recursively(storage_path, 1);
            response_data.push(root_folder);
        }
    }

    let response = ApiResponse { data: response_data };
    HttpResponse::Ok().json(&response)
}