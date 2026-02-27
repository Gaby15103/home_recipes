import {api} from "./client";
import {RecipeRoutes} from "./routes";
import type {RecipeCommentCreate, RecipeCreate} from "@/models/RecipeCreate";
import type {
    PaginatedRecipes,
    RecipeView,
    RecipeComment,
    RecipeFilter,
    RecipeRating,
    RecipeEditor
} from "@/models/Recipe.ts";
import {uploadSingleFile} from "@/api/upload.ts";


export function getAllRecipes(filters?: RecipeFilter, include_private?: boolean) {
    const params: Record<string, any> = { ...filters };
    if (filters?.search == "")
        filters.search = null
    if (include_private != null) {
        params.scope = include_private;
    }
    return api<RecipeView[]>(RecipeRoutes.all(), {method: "GET", params});
}

export function getAllRecipesByPage(
    page: number = 1,
    pageSize: number = 10,
    filters?: RecipeFilter
) {
    const params: Record<string, any> = {page, pageSize};
    if (filters?.search == "")
        filters.search = null
    params.scope = true;
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

export function getRecipeById(id: string, include_translations: boolean = false)
{
    // 1. Construct the path
    let path = RecipeRoutes.get(id);

    // 2. Append the query parameter
    // This turns true into "true" and handles the '?' prefix automatically
    const queryString = new URLSearchParams({
        include_translations: String(include_translations)
    }).toString();

    return api<RecipeView>(`${path}?${queryString}`, { method: "GET" });
}
// Proposed change to fetch a LIST of the latest recipes
export function getRecentRecipes(nb: number = 4, include_translations: boolean = false) {
    const queryString = new URLSearchParams({
        nb: String(nb),
        include_translations: String(include_translations)
    }).toString();

    const path = RecipeRoutes.get_last();

    return api<RecipeView[]>(`${path}?${queryString}`, { method: "GET" });
}
export function getRecipeByIdEditor(id: string, include_translations: boolean = false)
{
    // 1. Construct the path
    let path = RecipeRoutes.get(id);

    // 2. Append the query parameter
    // This turns true into "true" and handles the '?' prefix automatically
    const queryString = new URLSearchParams({
        include_translations: String(include_translations)
    }).toString();

    return api<RecipeEditor>(`${path}?${queryString}`, { method: "GET" });
}

export async function createRecipe(recipe: RecipeCreate): Promise<RecipeView> {
    if (recipe.image_url instanceof File){
        let res = await uploadSingleFile(recipe.image_url)
        recipe.image_url = res.temp_id
    }
    for(let step_group of recipe.step_groups){
        for(let step of step_group.steps){
            if (step.image_url instanceof File){
                let res = await uploadSingleFile(step.image_url)
                step.image_url = res.temp_id
            }
        }
    }

    return api<RecipeView>(RecipeRoutes.create(), {
        method: "POST",
        data: recipe,
    });
}

export async function updateRecipe(
    id: string,
    recipe: RecipeEditor
) {
    if (recipe.image_url instanceof File){
        let res = await uploadSingleFile(recipe.image_url)
        recipe.image_url = res.temp_id
    }
    for(let step_group of recipe.step_groups){
        for(let step of step_group.steps){
            if (step.image_url instanceof File){
                let res = await uploadSingleFile(step.image_url)
                step.image_url = res.temp_id
            }
        }
    }

    return api<RecipeView>(RecipeRoutes.update(id), {
        method: "PUT",
        data: recipe,
    });
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

export function getFavorites(): Promise<RecipeView[]> {
    return api<RecipeView[]>(RecipeRoutes.favorites(), {method: "GET"});
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
