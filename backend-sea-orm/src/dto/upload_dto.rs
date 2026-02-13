use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;

#[derive(Debug, actix_multipart::form::MultipartForm)]
pub struct SingleImageForm {
    pub image: TempFile,
}