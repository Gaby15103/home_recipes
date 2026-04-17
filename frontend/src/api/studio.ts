import {api} from "./client";

// Aligning with your existing RecipeView and PaginatedRecipes
import type {PaginatedRecipes, RecipeFilter, RecipeView} from "@/models/Recipe";
import {RecipeRoutes, StudioRoutes} from "@/api/routes.ts";

export interface DashboardStats {
    total_recipes: number;
    public_recipes: number;
    private_recipes: number;
    total_views: number;
}

export async function getStudioStats() {
    return api<DashboardStats>(StudioRoutes.stats, { method: "GET" });
}

export async function getRecentRecipes(limit = 5, include_translations: boolean = false) {
    const queryString = new URLSearchParams({
        nb: String(limit),
        include_translations: String(include_translations)
    }).toString();

    const path = StudioRoutes.recent;

    return api<RecipeView[]>(`${path}?${queryString}`, { method: "GET" });
}
export async function getStudioRecipes(
    page: number = 1,
    per_page: number = 10,
    filters?: RecipeFilter
) {
    const params: Record<string, any> = {
        page,
        per_page,
        include_private: true
    };
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

    return api<RecipeView[]>(StudioRoutes.getByFilter, {method: "GET", params});
}