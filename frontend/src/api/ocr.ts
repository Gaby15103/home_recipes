import {api} from "./client";
import {OcrRoutes} from "./routes";
import type {RecipeCreate} from "@/models/RecipeCreate";

export async function createRecipeFromImage(recipe_image: File): Promise<RecipeCreate> {
    const form = new FormData();
    form.append("image", recipe_image);

    return api<RecipeCreate>(OcrRoutes.create(), {
        method: "POST",
        data: form,
    });
}