use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use serde_derive::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct RegionDto {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub label: String,
    pub image_index: usize,
}

#[derive(Debug, MultipartForm)]
pub struct RegionOcrForm {
    #[multipart(rename = "images")]
    pub images: Vec<TempFile>,

    // We receive these as Text fields to handle the multipart string data
    pub regions: Text<String>,
    pub source_lang: Text<String>,
}