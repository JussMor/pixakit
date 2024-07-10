use aws_sdk_s3::primitives::ByteStreamError;
use ntex::web::WebResponseError;
pub mod router;
pub mod get_image;


#[derive(Debug)]
struct S3Error(ByteStreamError);

impl WebResponseError for S3Error {}

impl std::fmt::Display for S3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}