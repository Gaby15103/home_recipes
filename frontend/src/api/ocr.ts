import {api} from "./client";
import {OcrRoutes} from "./routes";
import type {RecipeCreate} from "@/models/RecipeCreate";

export async function createRecipeFromImages(recipe_image: File[]): Promise<RecipeCreate> {
    const form = new FormData();
    for (const file of recipe_image){
        form.append("images", file);
    }

    return api<RecipeCreate>(OcrRoutes.process(), {
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
    return api<RecipeCreate>(OcrRoutes.process_regions(), {
        method: "POST",
        data: form,
    });
}

export async function suggestRecipeFromFiles(files: File[]): Promise<RecipeCreate> {
    const form = new FormData();

    for (const file of files) {
        form.append("images", file);
    }

    return api<RecipeCreate>(OcrRoutes.process(), {
        method: "POST",
        data: form,
    });
}
// Add this to your OCR api file
export async function confirmOcrRecipe(payload: any): Promise<any> {
    return api<any>(OcrRoutes.create(), {
        method: "POST",
        data: payload,
    });
}