import { api } from "./client";
import type {Recipe} from "@/models/Recipe.ts";

export function getAllRecipes() {
    return api<Recipe[]>("/recipe/get_all",{
        method: "get"
    });
}