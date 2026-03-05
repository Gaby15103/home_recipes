use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;

#[derive(Debug, actix_multipart::form::MultipartForm)]
pub struct SingleImageForm {
    #[multipart(rename = "image")]
    pub image: TempFile,
}
#[derive(Debug, MultipartForm)]
pub struct MultiImageForm {
    #[multipart(rename = "images")]
    pub images: Vec<TempFile>,
}