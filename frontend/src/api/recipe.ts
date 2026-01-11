import {api} from "./client";
import type {Recipe, RecipeFilter} from "@/models/Recipe.ts";
import type {RecipeCreate, StepImage} from "@/models/RecipeCreate.ts";
import axios from "axios";
import {ref} from "vue";

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

export function getRecipeById(id: string) {
    const recipe = ref<Recipe|null>(null);
    const loading = ref(false);
    const error = ref("");

    async function fetchRecipe() {
        loading.value = true;
        try {
            recipe.value = await api<Recipe>(`/recipe/${id}`);
        } catch (err: any) {
            console.error(err);
            error.value = err.message || "Failed to load recipe";
        } finally {
            loading.value = false;
        }
    }

    return { recipe, loading, error, fetchRecipe };
}

export async function createRecipe(
    recipe: RecipeCreate,
    mainImage: File | null,
    stepImages: StepImage[],
): Promise<any> {
    const formData = new FormData();

    formData.append(
        "recipe",
        new Blob([JSON.stringify({ recipe })], { type: "application/json" }),
        "blob"
    );


    // 2️⃣ Main image
    if (mainImage) {
        formData.append("main_image", mainImage);
    }

    // 3️⃣ Step images and metadata
    const meta: { group_position: number; step_position: number; index: number }[] = [];

    stepImages.forEach((img, idx) => {
        if (!img.image_file) return;

        // Append each step image
        formData.append("step_images", img.image_file);

        // Add meta for this image
        meta.push({
            group_position: img.group_position,
            step_position: img.step_position,
            index: idx, // matches the order in FormData
        });
    });

    formData.append(
        "step_images_meta",
        new Blob([JSON.stringify(meta)], { type: "application/json" }),
        "blob"
    );


    // 5️⃣ Send request
    return axios.post(`${import.meta.env.VITE_API_URL}/recipe/create`, formData, {
        withCredentials: true,
        timeout: 30000, // large images might need more time
    });
}

export async function updateRecipe(
    id: string,
    recipe: Recipe,
    stepImages: StepImage[],
    mainImageFile?: File | null
) {
    const form = new FormData()

    form.append("recipe", JSON.stringify({
        recipe: {
            ...recipe,
            id,
        }
    }))

    if (mainImageFile) {
        form.append("main_image", mainImageFile)
    }

    const step_image_meta = stepImages.map((img, index) => {
        form.append("step_images[]", img.image_file)

        return {
            group_position: img.group_position,
            step_position: img.step_position,
            index,
        }
    })

    if (step_image_meta.length > 0) {
        form.append("step_image_meta", JSON.stringify(step_image_meta))
    }
    return api(`/recipes/${id}`, {
        method: "PATCH",
        data: form,
        headers: {
            "Content-Type": "multipart/form-data",
        },
    })
}
