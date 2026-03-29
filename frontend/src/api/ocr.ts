import {api} from "./client";
import {OcrRoutes} from "./routes";
import type {RecipeCreate} from "@/models/RecipeCreate";
import {uploadSingleFile} from "@/api/upload.ts";
import type {RecipeView} from "@/models/Recipe.ts";
import type {OcrRecipeResponse} from "@/models/OcrResult.ts";

export async function createRecipeFromImages(recipe_image: File[]): Promise<RecipeCreate> {
    const form = new FormData();
    for (const file of recipe_image) {
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
): Promise<OcrRecipeResponse> {
    const form = new FormData();

    // Append all raw image files
    files.forEach(file => form.append('images', file));

    // Append the metadata
    form.append('regions', JSON.stringify(regions));
    form.append('source_lang', lang);

    // Use your specific endpoint for regional processing
    return api<OcrRecipeResponse>(OcrRoutes.process_regions(), {
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
export async function confirmOcrRecipe(recipe: RecipeCreate): Promise<RecipeView> {
    if (recipe.image_url instanceof File) {
        const res = await uploadSingleFile(recipe.image_url);
        recipe.image_url = res.temp_id;
    }

    for (const step_group of recipe.step_groups) {
        for (const step of step_group.steps) {
            if (step.image_url instanceof File) {
                const res = await uploadSingleFile(step.image_url);
                step.image_url = res.temp_id;
            }
        }
    }

    const wrapper = {
        modified_recipe: recipe,
        lexicon_feedback: []
    };

    return api<any>(OcrRoutes.create(), {
        method: "POST",
        data: wrapper,
    });
}