import type { Recipe } from "@/models/Recipe"
import type { RecipeEdit, RecipeUpdatePayload } from "@/models/RecipeEdit"

export function recipeToEdit(recipe: Recipe): RecipeEdit {
    return {
        id: recipe.id,
        title: recipe.title,
        description: recipe.description,
        image_url: recipe.image_url,
        servings: recipe.servings,
        prep_time_minutes: recipe.prep_time_minutes,
        cook_time_minutes: recipe.cook_time_minutes,
        author: recipe.author,
        author_id: recipe.author_id,
        is_private: recipe.is_private,

        tags: recipe.tags.map(t => ({
            type: "existing",
            id: t.id,
        })),

        ingredient_groups: recipe.ingredient_groups.map(g => ({
            id: g.id,
            _uid: crypto.randomUUID(),
            title: g.title,
            position: g.position,
            ingredients: g.ingredients.map(i => ({
                id: i.id,
                _uid: crypto.randomUUID(),
                name: i.name,
                quantity: i.quantity,
                unit: i.unit,
                note: i.note,
                position: i.position,
            })),
        })),

        step_groups: recipe.step_groups.map(g => ({
            id: g.id,
            _uid: crypto.randomUUID(),
            title: g.title,
            position: g.position,
            steps: g.steps.map(s => ({
                id: s.id,
                _uid: crypto.randomUUID(),
                step_group_id: s.step_group_id,
                position: s.position,
                instruction: s.instruction,
                image_url: s.image_url,
                duration_minutes: s.duration_minutes,
            })),
        })),
    }
}

export function editToUpdatePayload(edit: RecipeEdit): RecipeUpdatePayload {
    return {
        id: edit.id,
        title: edit.title,
        description: edit.description,
        image_url: edit.image_url,
        servings: edit.servings,
        prep_time_minutes: edit.prep_time_minutes,
        cook_time_minutes: edit.cook_time_minutes,
        author: edit.author,
        author_id: edit.author_id,
        is_private: edit.is_private,

        tags: edit.tags.map(t =>
            t.type === "existing"
                ? { type: "existing", id: t.id }
                : { type: "new", name: t.name }
        ),

        ingredient_groups: edit.ingredient_groups.map(g => ({
            id: g.id,
            title: g.title,
            position: g.position,
            ingredients: g.ingredients.map(i => ({
                id: i.id,
                name: i.name,
                quantity: Number(i.quantity),
                unit: i.unit,
                note: i.note,
                position: i.position,
            })),
        })),

        step_groups: edit.step_groups.map(g => ({
            id: g.id,
            title: g.title,
            position: g.position,
            steps: g.steps.map(s => ({
                id: s.id,
                step_group_id: g.id,
                position: s.position,
                instruction: s.instruction,
                image_url: s.image_url ?? null,
                duration_minutes: s.duration_minutes,
            })),
        })),
    }
}
