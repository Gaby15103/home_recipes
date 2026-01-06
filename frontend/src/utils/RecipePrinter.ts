import type { Recipe } from "@/models/Recipe.ts";

/**
 * Opens a printable window with a recipe.
 * Prompts user if they want step images included.
 */
export function printRecipe(recipe: Recipe, options: {
    includeMainImage?: boolean,
    includeStepImages?: boolean,
    includeTags?: boolean
} = {}) {
    const { includeMainImage = true, includeStepImages = true, includeTags = true } = options;

    const recipeHtml = `
    <div style="font-family:sans-serif; color:#000;">
      <h1 style="font-size:2rem; margin-bottom:1rem;">${recipe.title}</h1>

      ${includeMainImage && recipe.image_url ? `
        <div style="margin-bottom:1rem;">
          <img src="${import.meta.env.VITE_STATIC_URL + recipe.image_url}" style="max-width:100%;height:auto;" />
        </div>
      ` : ""}

      <!-- Ingredients -->
      <h2>Ingredients</h2>
      ${recipe.ingredient_groups.map(group => `
        <div style="margin-bottom:1rem;">
          ${group.title ? `<strong>${group.title}</strong><br/>` : ""}
          <ul>${group.ingredients.map(ing => `<li>${ing.quantity} ${ing.unit} ${ing.name}</li>`).join("")}</ul>
        </div>
      `).join("")}

      <!-- Steps -->
      <h2>Steps</h2>
      ${recipe.step_groups.map(group => `
        <div style="margin-bottom:1rem;">
          ${group.title ? `<strong>${group.title}</strong><br/>` : ""}
          <ol>
            ${group.steps.map(step => `
              <li>
                ${step.instruction} ${step.duration_minutes ? `(${step.duration_minutes} min)` : ""}
                ${includeStepImages && step.image_url ? `<div><img src="${import.meta.env.VITE_STATIC_URL + step.image_url}" style="max-width:100%;height:auto;" /></div>` : ""}
              </li>
            `).join("")}
          </ol>
        </div>
      `).join("")}

      ${includeTags && recipe.tags?.length ? `
        <h2>Tags</h2>
        <ul style="display:flex; gap:0.5rem; flex-wrap:wrap;">
          ${recipe.tags.map(t => `<li style="border:1px solid #000; padding:0.25rem 0.5rem; border-radius:0.25rem;">${t.name}</li>`).join("")}
        </ul>
      ` : ""}
    </div>
  `;

    const printWindow = window.open("", "_blank", "width=800,height=600");
    if (!printWindow) return;
    printWindow.document.write(`<html><head><title>${recipe.title}</title></head><body>${recipeHtml}</body></html>`);
    printWindow.document.close();
    printWindow.focus();
    printWindow.print();
}

