// models/RecipeCreate.ts
import {IngredientUnit} from "./Recipe"
import type {InputTag} from "@/models/Tag.ts";

export type StepCreateUI = StepCreate & {
    _uid: string
}


export interface StepImage {
    step_position: number
    group_position: number
    image_file: File
    image_preview: string
}

export interface IngredientGroupTranslationCreate {
    language_code: string;
    title: string;
}

export interface StepGroupTranslationCreate {
    language_code: string;
    title: string;
}
export interface  StepTranslationCreate {
    language_code: string
    instruction: string
}
export interface StepCreate {
    position: number
    image_url: string | File | null
    translations: StepTranslationCreate[]
    duration_minutes: number | null
}

export interface StepGroupCreate {
    translations: StepGroupTranslationCreate
    position: number
    steps: StepCreate[]
}

export interface IngredientGroupCreate {
    translations: IngredientGroupTranslationCreate[]
    position: number;
    ingredients: IngredientCreate[];
}
export interface IngredientTranslationCreate {
    language_code: string
    title: string
}
export interface IngredientCreate {
    translations: IngredientTranslationCreate[];
    quantity: number;
    unit: IngredientUnit;
    note_translations: IngredientNoteTranslation[] | null;
    position: number;
}
export interface IngredientNoteTranslation {
    language_code: string
    note: string
}

export interface RecipeTranslation{
    language_code: string
    title: string
    description: string
}

export interface RecipeCreate {
    primary_language: string
    translations: RecipeTranslation[]
    image_url: string | File | null
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
export interface RecipeCommentCreate {
    recipe_id: string;
    user_id: string | null;
    parent_id?: string | null;
    content: string;
}