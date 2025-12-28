import { api } from "./client";
import type {Recipe} from "@/models/Recipe.ts";

export function getAllRecipes(is_private?: boolean): Promise<Recipe[]> {
    let req: string = "/recipe/get_all";
    if (is_private != null)
        req = is_private ? `?private=${is_private}` : "";

    return api<Recipe[]>(req ,{
        method: "get"
    });
}