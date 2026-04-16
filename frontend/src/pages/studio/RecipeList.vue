<script setup lang="ts">
import RecipeActions from './RecipeActions.vue'
import { Badge } from '@/components/ui/badge'

interface Recipe {
  id: number
  title: string
  status: 'draft' | 'published' | 'private'
  updated_at: string
  views: number
}

defineProps<{ recipes: Recipe[] }>()
</script>

<template>
  <div class="rounded-3xl border border-neutral-800 bg-[#0a0a0a] overflow-hidden">
    <table class="w-full text-left border-collapse">
      <thead>
      <tr class="text-[10px] uppercase font-black text-neutral-500 tracking-widest bg-neutral-900/50">
        <th class="px-6 py-4">Content</th>
        <th class="px-6 py-4">Visibility</th>
        <th class="px-6 py-4">Last Updated</th>
        <th class="px-6 py-4">Stats</th>
        <th class="px-6 py-4 text-right">Options</th>
      </tr>
      </thead>

      <tbody class="divide-y divide-neutral-900">
      <tr v-if="recipes.length === 0">
        <td colspan="5" class="px-6 py-20 text-center text-neutral-600 italic text-sm">
          No recipes found in your studio.
        </td>
      </tr>

      <tr v-for="recipe in recipes" :key="recipe.id" class="group hover:bg-white/[0.02] transition-colors">
        <td class="px-6 py-4">
          <div class="flex items-center gap-4">
            <div class="h-10 w-10 rounded-xl bg-neutral-800 border border-neutral-700 flex items-center justify-center shrink-0">
              <span class="text-xs font-black text-neutral-600">{{ recipe.title[0] }}</span>
            </div>
            <span class="font-bold text-neutral-200 text-sm truncate max-w-[200px]">
                {{ recipe.title }}
              </span>
          </div>
        </td>

        <td class="px-6 py-4">
          <Badge
              variant="outline"
              :class="[
                'capitalize text-[10px] font-black border-none px-0',
                recipe.status === 'published' ? 'text-green-500' : 'text-neutral-500'
              ]"
          >
            <div :class="['h-1.5 w-1.5 rounded-full mr-2', recipe.status === 'published' ? 'bg-green-500' : 'bg-neutral-600']" />
            {{ recipe.status }}
          </Badge>
        </td>

        <td class="px-6 py-4 text-xs text-neutral-500">
          {{ recipe.updated_at }}
        </td>

        <td class="px-6 py-4">
            <span class="text-[10px] font-bold text-neutral-400 bg-neutral-800 px-2 py-1 rounded">
              {{ recipe.views }} Views
            </span>
        </td>

        <td class="px-6 py-4 text-right">
          <RecipeActions :id="recipe.id" />
        </td>
      </tr>
      </tbody>
    </table>
  </div>
</template>