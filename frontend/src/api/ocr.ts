import {api} from "./client";
import {OcrRoutes} from "./routes";
import type {RecipeCreate} from "@/models/RecipeCreate";

export async function createRecipeFromImages(recipe_image: File[]): Promise<RecipeCreate> {
    const form = new FormData();
    for (const file of recipe_image){
        form.append("images", file);
    }

    return api<RecipeCreate>(OcrRoutes.create(), {
        method: "POST",
        data: form,
    });
}