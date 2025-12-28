export interface Recipe {
    title: string;
    description: string|null;
    image_url: string;
    serving: number;
    prep_time_minutes: number;
    cook_time_minutes: number;
    author: string|null;
    author_id: string|null;
    is_private: boolean;
    tags: Tag[];
    ingredients_groups: IngredientGroup[];
    step_group: StepGroup[];
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
    unite: IngredientUnit;
    note: string|null;
    position: number;
}

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
    instructions: string;
    image_url: string|null;
    duration_minutes: number|null;
}