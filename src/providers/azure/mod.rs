use ntex::web::WebResponseError;

pub mod router;
pub mod upload;
pub mod get_images;


// Define a new type to wrap the azure_storage::Error
#[derive(Debug)]
struct AzureStorageError(azure_storage::Error);

// Implement the WebResponseError trait for the new type
impl WebResponseError for AzureStorageError {}

// Implement the std::fmt::Display trait for the new type
impl std::fmt::Display for AzureStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}