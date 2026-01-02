import { api } from "./client";
import type {Recipe, RecipeFilter} from "@/models/Recipe.ts";

export function getAllRecipes(
    scope = false,
    filters?: RecipeFilter
) {
    const params: Record<string, any> = {}

    let req: string = "/recipe/get_all";
    if (scope != null)
        req += scope ? `?scope=${scope}` : "";

    if (filters) {
        if (filters.search) params.search = filters.search
        if (filters.ingredient) params.ingredient = filters.ingredient
        if (filters.tags.length)
            params.tags = filters.tags.map(t => t.id).join(",")

        if (filters.minPrep != null) params.minPrep = filters.minPrep
        if (filters.maxPrep != null) params.maxPrep = filters.maxPrep
        if (filters.minCook != null) params.minCook = filters.minCook
        if (filters.maxCook != null) params.maxCook = filters.maxCook
        if (filters.minSteps != null) params.minSteps = filters.minSteps
        if (filters.maxSteps != null) params.maxSteps = filters.maxSteps
        if (filters.dateFrom) params.dateFrom = filters.dateFrom
        if (filters.dateTo) params.dateTo = filters.dateTo
    }

    return api<Recipe[]>(
        req,
        {params}
    )
}