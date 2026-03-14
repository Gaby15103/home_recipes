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
export async function createRecipeFromRegions(
    files: File[],
    regions: any[],
    lang: string
): Promise<RecipeCreate> {
    const form = new FormData();

    // Append all raw image files
    files.forEach(file => form.append('images', file));

    // Append the metadata
    form.append('regions', JSON.stringify(regions));
    form.append('source_lang', lang);

    // Use your specific endpoint for regional processing
    return api<RecipeCreate>(OcrRoutes.create(), {
        method: "POST",
        data: form,
    });
}