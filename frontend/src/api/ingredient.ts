import {api} from "./client";
import {IngredientRoutes} from "./routes";
import type { IngredientView} from "@/models/Recipe.ts";

export function getIngredients(search: string, limit: number): Promise<IngredientView[]> {
    const params: Record<string, any> = { search, limit };

    return api<IngredientView[]>(IngredientRoutes.list(), {method: "GET", params});
}