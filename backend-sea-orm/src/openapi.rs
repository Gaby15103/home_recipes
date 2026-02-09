use utoipa::OpenApi;
use crate::dto::recipe_dto::RecipeResponseDto;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controllers::recipes_controller::list,
        crate::controllers::recipes_controller::get
        // add other controllers here
    ),
    components(schemas(
        RecipeResponseDto
        // add other DTOs here
    )),
    tags(
        (name = "recipes", description = "Recipe API endpoints")
        // add other tags for other controllers
    )
)]
pub struct ApiDoc;
