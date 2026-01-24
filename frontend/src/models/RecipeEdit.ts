import type { InputTag } from "@/models/Tag"
import { IngredientUnit } from "@/models/Recipe"

export interface RecipeEdit {
    id: string
    title: string
    description: string | null
    image_url: string | null
    servings: number
    prep_time_minutes: number
    cook_time_minutes: number
    author: string | null
    author_id: string | null
    is_private: boolean

    tags: InputTag[]

    ingredient_groups: IngredientGroupEdit[]
    step_groups: StepGroupEdit[]
}

export interface IngredientGroupEdit {
    id?: string
    _uid: string
    title: string
    position: number
    ingredients: IngredientEdit[]
}

export interface IngredientEdit {
    id?: string
    _uid: string
    name: string
    quantity: number
    unit: IngredientUnit
    note: string | null
    position: number
}

export interface StepGroupEdit {
    id?: string
    _uid: string
    title: string
    position: number
    steps: StepEdit[]
}

export interface StepEdit {
    id?: string
    _uid: string
    step_group_id?: string
    position: number
    instruction: string
    image_url: string | null
    duration_minutes: number | null
}

export interface RecipeUpdatePayload {
    id: string
    title: string
    description: string | null
    image_url: string | null
    servings: number
    prep_time_minutes: number
    cook_time_minutes: number
    author: string | null
    author_id: string | null
    is_private: boolean

    tags: (
        | { type: "existing"; id: string }
        | { type: "new"; name: string }
        )[]

    ingredient_groups: {
        id?: string
        title: string
        position: number
        ingredients: {
            id?: string
            name: string
            quantity: number
            unit: IngredientUnit
            note: string | null
            position: number
        }[]
    }[]

    step_groups: {
        id?: string
        title: string
        position: number
        steps: {
            id?: string
            step_group_id?: string
            position: number
            instruction: string
            image_url: string | null
            duration_minutes: number | null
        }[]
    }[]
}
