use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controllers::recipes_controller::list,
        crate::controllers::recipes_controller::get,
        // add other controllers here
    ),
    components(schemas(
        crate::dto::recipe_dto::RecipeViewDto,
        crate::dto::recipe_dto::CreateRecipeInput,
        crate::dto::recipe_dto::RecipeTranslationInput,
        crate::dto::ingredient_group_dto::IngredientGroupInput,
        crate::dto::ingredient_group_dto::IngredientGroupInput,
        crate::dto::step_group_dto::StepGroupInput,
        crate::dto::step_dto::StepInput,
        crate::dto::tag_dto::InputTag
    )),
    tags(
        (name = "recipes", description = "Recipe API endpoints")
        // add other tags for other controllers
    )
)]
pub struct ApiDoc;
