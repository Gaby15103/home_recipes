import type {Tag} from "@/models/Tag.ts";

export interface Recipe {
    id: string;
    image_url: string;
    servings: number;
    prep_time_minutes: number;
    cook_time_minutes: number;
    author: string|null;
    author_id: string|null;
    is_private: boolean;
    tags: Tag[];
    ingredient_groups: IngredientGroup[];
    step_groups: StepGroup[];
    translations: RecipeTranslation[];
}
export interface RecipeTranslation{
    language_code: String;
    title: String;
    description: String;
}
export interface IngredientGroup {
    id: string;
    position: number;
    ingredients: Ingredient[];
    translations: IngredientGroupTranslation[]
}
export interface IngredientGroupTranslation{
    language_code: String;
    title: String;
}
export interface Ingredient {
    id: string;
    quantity: number;
    unit: IngredientUnit;
    note: string|null;
    position: number;
    translations: IngredientTranslation[];
}
export interface IngredientTranslation {
    language_code: String;
    name: String;
}


//TODO add pinch, a can, unique like 1 oignon or an other where you put with our without mesure
// @ts-ignore
export enum IngredientUnit {
    Gram = "gram",
    Kilogram = "kilogram",
    Milliliter = "milliliter",
    Liter = "liter",
    Piece = "piece",
    Teaspoon = "teaspoon",
    Tablespoon = "tablespoon",
    Cup = "cup",
}
export interface StepGroup {
    id: string;
    position: number;
    steps: Step[];
    translations: StepGroupTranslation[];
}
export interface StepGroupTranslation {
    language_code: String;
    title: String;
}
export interface Step {
    id: string;
    step_group_id: string;
    position: number;
    image_url: string|null;
    duration_minutes: number|null;
    translations: StepTranslation[];
}

export interface StepTranslation {
    language_code: String;
    instruction: String;
}

export interface RecipeFilter {
    search: string | null,            // name / description
    ingredient: string | null,
    tags: Tag[],
    minPrep: number | null,
    maxPrep: number | null,
    minCook: number | null,
    maxCook: number | null,
    minSteps: number | null,
    maxSteps: number | null,
    dateFrom: string | null,
    dateTo: string | null,
}
// Params interface matching backend filter
export interface GetRecipesParams {
    page?: number;
    perPage?: number;
    scope?: string;
    search?: string;
    ingredient?: string;
    tags?: string;
    minPrep?: number;
    maxPrep?: number;
    minCook?: number;
    maxCook?: number;
    minSteps?: number;
    maxSteps?: number;
    dateFrom?: string; // "YYYY-MM-DD"
    dateTo?: string;   // "YYYY-MM-DD"
    sort?: string;
}
export interface PaginatedRecipes {
    data: Recipe[]
    total: number
    page: number
    perPage: number
}
export interface RecipeComment {
    id: string;
    user_id: string;
    username: string;
    content: string;
    created_at: string; // ISO string
    parent_id?: string | null; // for tree comments
    children: RecipeComment[];
}

export interface RecipeRating {
    average: number
    count: number
    user_rating: number | null
}
