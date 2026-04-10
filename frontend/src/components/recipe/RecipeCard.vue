<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ROUTES } from "@/router/routes.ts"
import { Clock, Flame, Utensils } from 'lucide-vue-next'
import type { RecipeView } from '@/models/Recipe'

const { t } = useI18n()

defineProps<{
  recipe: RecipeView
}>()
</script>

<template>
  <RouterLink
      :to="ROUTES.RECIPE(recipe.id)"
      class="group flex flex-col bg-card rounded-xl border shadow-sm transition-all hover:shadow-md hover:-translate-y-1 overflow-hidden"
  >
    <div class="aspect-video w-full overflow-hidden bg-muted relative">
      <img
          v-if="recipe.image_url"
          :src="$apiUrl + recipe.image_url"
          class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-110"
          alt="Recipe thumbnail"
      >
      <div v-else class="h-full w-full flex items-center justify-center text-muted-foreground/20">
        <Utensils class="h-12 w-12" />
      </div>

      <div class="absolute top-2 right-2 flex gap-1">
           <span class="bg-black/50 backdrop-blur-md text-white text-[10px] px-2 py-0.5 rounded-full uppercase font-bold">
             {{ recipe.servings }} {{ t('recipe.meta.servings') }}
           </span>
      </div>
    </div>

    <div class="p-4 flex flex-col flex-grow">
      <h2 class="text-lg font-bold mb-2 group-hover:text-primary transition-colors line-clamp-1">
        {{ recipe.title }}
      </h2>

      <p v-if="recipe.description" class="text-muted-foreground text-sm mb-4 line-clamp-2 h-10 italic">
        {{ recipe.description }}
      </p>

      <div class="mt-auto pt-4 border-t flex items-center justify-between text-xs text-muted-foreground">
        <div class="flex items-center gap-3">
          <span class="flex items-center gap-1">
            <Clock class="h-3 w-3" /> {{ (recipe.prep_time_minutes || 0) + (recipe.cook_time_minutes || 0) }}m
          </span>
          <span class="flex items-center gap-1">
            <Flame class="h-3 w-3 text-orange-500" /> {{ recipe.difficulty || t('Home.DifficultyEasy') }}
          </span>
        </div>
      </div>

      <div class="mt-3 flex flex-wrap gap-1">
          <span
              v-for="tag in recipe.tags.slice(0, 3)"
              :key="tag.id"
              class="text-[10px] font-medium bg-secondary text-secondary-foreground px-2 py-0.5 rounded-md"
          >
            #{{ tag.name }}
          </span>
        <span v-if="recipe.tags.length > 3" class="text-[10px] text-muted-foreground">
            +{{ recipe.tags.length - 3 }}
          </span>
      </div>
    </div>
  </RouterLink>
</template>