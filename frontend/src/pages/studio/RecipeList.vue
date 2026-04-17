<script setup lang="ts">
import RecipeActions from './RecipeActions.vue'
import {Badge} from '@/components/ui/badge'
import type {RecipeView} from "@/models/Recipe.ts"
import {Clock, Globe, Lock, Utensils} from 'lucide-vue-next'
import {onMounted} from "vue";

const props = defineProps<{ recipes: RecipeView[] }>()


/**
 * Format date for Arch-style technical look
 */
function formatDate(dateStr?: string) {
  if (!dateStr) return '---'
  const date = new Date(dateStr)
  return date.toLocaleDateString('en-CA', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  })
}
onMounted(()=>{
  console.log(props.recipes)
})
</script>

<template>
  <div class="rounded-3xl border border-neutral-800 bg-[#0a0a0a] overflow-hidden">
    <table class="w-full text-left border-collapse">
      <thead>
      <tr class="text-[10px] uppercase font-black text-neutral-500 tracking-widest bg-neutral-900/50">
        <th class="px-6 py-4">Recipe</th>
        <th class="px-6 py-4">Configuration</th>
        <th class="px-6 py-4">Visibility</th>
        <th class="px-6 py-4">Time</th>
        <th class="px-6 py-4 text-right">Options</th>
      </tr>
      </thead>

      <tbody class="divide-y divide-neutral-900">
      <tr v-if="recipes.length === 0">
        <td colspan="5" class="px-6 py-20 text-center text-neutral-600 italic text-sm">
          No recipes found in your collection.
        </td>
      </tr>

      <tr v-for="recipe in recipes" :key="recipe.id" class="group hover:bg-white/2 transition-colors">
        <td class="px-6 py-4">
          <div class="flex items-center gap-4">
            <div v-if="recipe.image_url" class="h-10 w-10 rounded-xl overflow-hidden border border-neutral-800 shrink-0">
              <img :src="$apiUrl + recipe.image_url" class="h-full w-full object-cover" alt="recipe" />
            </div>
            <div v-else class="h-10 w-10 rounded-xl bg-neutral-900 border border-neutral-800 flex items-center justify-center shrink-0">
              <Utensils class="h-4 w-4 text-neutral-700" />
            </div>
            <div class="flex flex-col">
                <span class="font-bold text-neutral-200 text-sm truncate max-w-75">
                  {{ recipe.title }}
                </span>
            </div>
          </div>
        </td>

        <td class="px-6 py-4">
          <div class="flex gap-2">
              <span class="text-[10px] font-bold text-neutral-400 bg-neutral-900 border border-neutral-800 px-2 py-0.5 rounded uppercase">
                {{ recipe.nb_steps }} Steps
              </span>
            <span class="text-[10px] font-bold text-neutral-400 bg-neutral-900 border border-neutral-800 px-2 py-0.5 rounded uppercase">
                {{ recipe.nb_ingredients }} Ingredients
              </span>
          </div>
        </td>

        <td class="px-6 py-4">
          <Badge
              variant="outline"
              :class="[
                  'capitalize text-[10px] font-black border-none px-0 flex items-center gap-2',
                  recipe.is_private ? 'text-amber-500/80' : 'text-primary'
                ]"
          >
            <component :is="recipe.is_private ? Lock : Globe" class="h-3 w-3" />
            {{ recipe.is_private ? 'Private / Draft' : 'Public' }}
          </Badge>
        </td>

        <td class="px-6 py-4">
          <div class="flex flex-col gap-1">
            <div class="flex items-center gap-2 text-neutral-500">
              <Clock class="h-3 w-3" />
              <span class="text-xs">{{ recipe.prep_time_minutes + recipe.cook_time_minutes }}m total</span>
            </div>
          </div>
        </td>

        <td class="px-6 py-4 text-right">
          <RecipeActions :id="recipe.id" />
        </td>
      </tr>
      </tbody>
    </table>
  </div>
</template>