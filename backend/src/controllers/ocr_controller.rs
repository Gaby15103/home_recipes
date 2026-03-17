use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpResponse};
use crate::app::state::AppState;
use crate::errors::Error;
use crate::domain::user::{AuthenticatedUser, Role};
use crate::dto::upload_dto::{MultiImageForm, RegionDto, RegionOcrForm}; // New DTO for multiple files
use crate::dto::recipe_dto::CreateRecipeInput;
use crate::dto::recipe_ocr::{OcrConfirmInput, OcrCorrectionWrapper};
use crate::services::{ocr_service, recipe_service};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ocr")
            .app_data(web::PayloadConfig::new(20 * 1024 * 1024)) // Increased for multiple images
            // Phase 1: Upload images, get back structured suggestions for calibration
            .route("/process", web::post().to(get_ocr_suggestions))
            .route("/process_regions", web::post().to(recipe_from_regions))
            // Phase 2: Accept the final calibrated data to create the actual recipe
            .route("/confirm", web::post().to(confirm_ocr_recipe))
    );
}


pub async fn recipe_from_regions(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
    MultipartForm(form): MultipartForm<RegionOcrForm>,
) -> Result<HttpResponse, Error> {
    auth.require_roles(&[Role::Admin, Role::Moderator, Role::Superuser])?;

    let regions: Vec<RegionDto> = serde_json::from_str(&form.regions.0)?;

    let source_lang = form.source_lang.0;

    // Pass the Vec<TempFile> to the service
    let suggestions = ocr_service::recipe_from_regions(
        form.images,
        regions,
        source_lang,
        &state.db,
        &state.dict_db
    ).await?;

    Ok(HttpResponse::Ok().json(suggestions))
}
/// POST /ocr/suggest
/// Takes images, returns OcrResultResponse (Bridge DTO)
pub async fn get_ocr_suggestions(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
    MultipartForm(form): MultipartForm<MultiImageForm>,
)-> Result<HttpResponse, Error> {
    auth.require_roles(&[Role::Admin, Role::Moderator, Role::Superuser])?;

    // Pass the Vec<TempFile> to the service
    let suggestions = ocr_service::recipe_from_files(
        form.images,
        &state.db,
        &state.dict_db
    ).await?;

    Ok(HttpResponse::Ok().json(suggestions))
}

/// POST /ocr/confirm
/// Takes the final edited CreateRecipeInput and saves it to the main DB
pub async fn confirm_ocr_recipe(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
    web::Json(payload): web::Json<OcrCorrectionWrapper>, // Changed from OcrConfirmInput
) -> Result<HttpResponse, Error> {
    auth.require_roles(&[Role::Admin, Role::Moderator, Role::Superuser])?;

    let created_recipe = ocr_service::process_ocr_confirmation(
        payload,
        &state.db,
        &state.dict_db,
        &auth.user.preferences.language
    ).await?;

    Ok(HttpResponse::Created().json(created_recipe))
}