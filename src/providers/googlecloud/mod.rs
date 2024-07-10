use ntex::web::WebResponseError;
pub mod router;
pub mod get_image;


#[derive(Debug)]
struct GoogleStorageError(google_cloud_storage::http::Error);


impl WebResponseError for GoogleStorageError {}

// Implement the std::fmt::Display trait for the new type
impl std::fmt::Display for GoogleStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}