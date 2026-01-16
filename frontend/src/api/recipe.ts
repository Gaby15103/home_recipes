import { api } from "./client";
import { RecipeRoutes } from "./routes";
import { formDataFromObject } from "./apiHelpers";
import type { RecipeCreate, StepImage } from "@/models/RecipeCreate";
import type {PaginatedRecipes, Recipe, RecipeFilter} from "@/models/Recipe.ts";

export function getAllRecipes(filters?: RecipeFilter) {
    const params: Record<string, any> = { ...filters };
    return api<Recipe[]>(RecipeRoutes.all(), { method: "GET", params });
}
export function getAllRecipesByPage(
    page: number = 1,
    pageSize: number = 10,
    filters?: RecipeFilter
) {
    const params: Record<string, any> = { page, pageSize };

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

    return api<PaginatedRecipes>(RecipeRoutes.byPage(), { method: "GET", params });
}

export function getRecipeById(id: string) {
    return api<Recipe>(RecipeRoutes.get(id), { method: "GET" });
}

export async function createRecipe(recipe: RecipeCreate, mainImage?: File, stepImages?: StepImage[]) {
    const payload: any = { recipe };
    if (mainImage) payload.mainImage = mainImage;
    if (stepImages) payload.stepImages = stepImages;

    return api(RecipeRoutes.create(), {
        method: "POST",
        data: formDataFromObject(payload),
        headers: { "Content-Type": "multipart/form-data" },
    });
}

export function updateRecipe(id: string, recipe: Partial<RecipeCreate>) {
    return api(RecipeRoutes.update(id), { method: "PUT", data: recipe });
}

export function deleteRecipe(id: string) {
    return api(RecipeRoutes.delete(id), { method: "DELETE" });
}

export function getAnalytics(id: string) {
    return api(RecipeRoutes.analytics(id), { method: "GET" });
}

export function trackView(id: string) {
    return api(RecipeRoutes.trackView(id), { method: "POST" });
}

export function favoriteRecipe(id: string) {
    return api(RecipeRoutes.favorite(id), { method: "POST" });
}

export function getFavorites() {
    return api(RecipeRoutes.favorites(), { method: "GET" });
}

export function rateRecipe(id: string, rating: number) {
    return api(RecipeRoutes.rate(id), { method: "POST", data: { rating } });
}

export function unrateRecipe(id: string) {
    return api(RecipeRoutes.unrate(id), { method: "DELETE" });
}

export function getRating(id: string) {
    return api(RecipeRoutes.getRating(id), { method: "GET" });
}

export function getComments(id: string) {
    return api(RecipeRoutes.getComments(id), { method: "GET" });
}

export function addComment(id: string, comment: string) {
    return api(RecipeRoutes.addComment(id), { method: "POST", data: { comment } });
}

export function restoreVersion(recipeId: string, versionId: string) {
    return api(RecipeRoutes.restoreVersion(recipeId, versionId), { method: "POST" });
}
