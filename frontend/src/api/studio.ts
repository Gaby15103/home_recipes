import {api} from "./client";

// Aligning with your existing RecipeView and PaginatedRecipes
import type {RecipeView} from "@/models/Recipe";
import {StudioRoutes} from "@/api/routes.ts";

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