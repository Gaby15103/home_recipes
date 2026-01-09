export interface Recipe {
    id: string;
    title: string;
    description: string|null;
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
}

export interface Tag {
    id: string;
    name: string;
}
export interface IngredientGroup {
    id: string;
    title: string;
    position: number;
    ingredients: Ingredient[];
}
export interface Ingredient {
    id: string;
    name: string;
    quantity: number;
    unit: IngredientUnit;
    note: string|null;
    position: number;
}

//TODO add pinch, a can, unique like 1 oignon or an other where you put with our without mesure
export enum IngredientUnit {
    Gram = "Gram",
    Kilogram = "Kilogram",
    Milliliter = "Milliliter",
    Liter = "Liter",
    Piece = "Piece",
    Teaspoon = "Teaspoon",
    Tablespoon = "Tablespoon",
    Cup = "Cup",
}
export interface StepGroup {
    id: string;
    title: string;
    position: number;
    steps: Step[];
}
export interface Step {
    id: string;
    step_group_id: string;
    position: number;
    instruction: string;
    image_url: string|null;
    duration_minutes: number|null;
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