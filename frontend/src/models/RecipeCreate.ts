// models/RecipeCreate.ts
import {IngredientUnit} from "./Recipe"

export type StepCreateUI = StepCreate & {
    _uid: string
}


export interface StepImage {
    step_position: number
    group_position: number
    image_file: File
    image_preview: string
}

export interface StepCreate {
    position: number
    instruction: string
    duration_minutes: number | null
}

export interface StepGroupCreate {
    title: string
    position: number
    steps: StepCreate[]
}

export interface IngredientGroupCreate {
    title: string;
    position: number;
    ingredients: IngredientCreate[];
}
export interface IngredientCreate {
    name: string;
    quantity: number;
    unit: IngredientUnit;
    note: string|null;
    position: number;
}

export type InputTag =
    | { type: 'existing'; id: string }
    | { type: 'new'; name: string }


export interface RecipeCreate {
    title: string
    description: string | null
    servings: number
    prep_time_minutes: number
    cook_time_minutes: number
    author: null
    author_id: null
    is_private: boolean
    tags: InputTag[]
    ingredient_groups: IngredientGroupCreate[]
    step_groups: StepGroupCreate[]
}

export interface RecipeImport extends Omit<
    RecipeCreate,
    never
> {}