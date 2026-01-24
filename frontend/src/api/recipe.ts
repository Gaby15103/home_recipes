import {api} from "./client";
import {RecipeRoutes} from "./routes";
import {formDataFromObject} from "./apiHelpers";
import type {RecipeCommentCreate, RecipeCreate, StepImage} from "@/models/RecipeCreate";
import type {PaginatedRecipes, Recipe, RecipeComment, RecipeFilter, RecipeRating} from "@/models/Recipe.ts";
import type {RecipeEdit} from "@/models/RecipeEdit.ts";
import {editToUpdatePayload} from "@/mappers/recipe.mapper.ts";

export function getAllRecipes(filters?: RecipeFilter) {
    const params: Record<string, any> = {...filters};
    return api<Recipe[]>(RecipeRoutes.all(), {method: "GET", params});
}

export function getAllRecipesByPage(
    page: number = 1,
    pageSize: number = 10,
    filters?: RecipeFilter
) {
    const params: Record<string, any> = {page, pageSize};

    params.include_private = true;

    if (filters) {
        if (filters.search) params.search = filters.search;
        if (filters.ingredient) params.ingredient = filters.ingredient;
        if (filters.tags?.length) params.tags = filters.tags.map(t => t.id).join(",");
        if (filters.minPrep != null) params.minPrep = filters.minPrep;
        if (filters.maxPrep != null) params.maxPrep = filters.maxPrep;
        if (filters.minCook != null) params.minCook = filters.minCook;
        if (filters.maxCook != null) params.maxCook = filters.maxCook;
        if (filters.minSteps != null) params.minSteps = filters.minSteps;
        if (filters.maxSteps != null) params.maxSteps = filters.maxSteps;
        if (filters.dateFrom) params.dateFrom = filters.dateFrom;
        if (filters.dateTo) params.dateTo = filters.dateTo;
    }

    return api<PaginatedRecipes>(RecipeRoutes.byPage(), {method: "GET", params});
}

export function getRecipeById(id: string) {
    return api<Recipe>(RecipeRoutes.get(id), {method: "GET"});
}

export async function createRecipe(recipe: RecipeCreate, mainImage?: File, stepImages?: StepImage[]) {
    const payload: any = {recipe};
    if (mainImage) payload.mainImage = mainImage;
    if (stepImages) payload.stepImages = stepImages;

    return api(RecipeRoutes.create(), {
        method: "POST",
        data: formDataFromObject(payload),
        headers: {"Content-Type": "multipart/form-data"},
    });
}

export async function updateRecipe(
    id: string,
    recipe: RecipeEdit,
    stepImages: StepImage[],
    mainImageFile?: File | null
) {
    const form = new FormData()
    const payload = editToUpdatePayload(recipe)

    form.append("recipe", new Blob([JSON.stringify({ recipe: payload })], { type: "application/json" }))

    if (mainImageFile) {
        form.append("main_image", mainImageFile)
    }

    if (stepImages.length > 0) {
        stepImages.forEach((img) => form.append("step_images[]", img.image_file))
    }

    form.append(
        "step_images_meta",
        new Blob([JSON.stringify(
            stepImages.length > 0
                ? stepImages.map((img, i) => ({
                    group_position: img.group_position,
                    step_position: img.step_position,
                    index: i,
                }))
                : []
        )], { type: "application/json" })
    )

    return api(RecipeRoutes.update(id), {
        method: "PUT",
        data: form,
        headers: {
            "Content-Type": "multipart/form-data",
        },
    })
}

export function deleteRecipe(id: string) {
    return api(RecipeRoutes.delete(id), {method: "DELETE"});
}

export function getAnalytics(id: string) {
    return api(RecipeRoutes.analytics(id), {method: "GET"});
}

export function trackView(id: string) {
    return api(RecipeRoutes.trackView(id), {method: "POST"});
}

export function favoriteRecipe(id: string) {
    return api(RecipeRoutes.favorite(id), {method: "POST"});
}

export function getFavorites(): Promise<Recipe[]> {
    return api<Recipe[]>(RecipeRoutes.favorites(), {method: "GET"});
}

export function rateRecipe(id: string, rating: number) {
    return api(RecipeRoutes.rate(id), {method: "POST", data: {rating}});
}

export function unrateRecipe(id: string) {
    return api(RecipeRoutes.unrate(id), {method: "DELETE"});
}

export function getRating(id: string): Promise<RecipeRating> {
    return api<RecipeRating>(RecipeRoutes.getRating(id), {method: "GET"});
}

export function getComments(id: string): Promise<RecipeComment[]> {
    return api<RecipeComment[]>(RecipeRoutes.getComments(id), {method: "GET"});
}

export function addComment(id: string, comment: RecipeCommentCreate): Promise<RecipeComment> {
    return api<RecipeComment>(RecipeRoutes.addComment(id), {method: "POST", data: comment});
}

export function restoreVersion(recipeId: string, versionId: string) {
    return api(RecipeRoutes.restoreVersion(recipeId, versionId), {method: "POST"});
}
