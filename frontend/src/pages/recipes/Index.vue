<script setup lang="ts">
import {ref, onMounted, watch} from "vue";
import type {RecipeFilter, RecipeView} from "@/models/Recipe.ts";
import { getAllRecipes } from "@/api/recipe.ts";
import {RouterLink} from "vue-router";
import {Spinner} from "@/components/ui/spinner";
import { useI18n } from "vue-i18n"
import Filter from "@/components/recipe/forms/Filter.vue";
import {Button} from "@/components/ui/button";
import {ROUTES} from "@/router/routes.ts";
const { t, locale } = useI18n()
// State for recipes
const recipes = ref<RecipeView[]>([]);
const loading = ref(true);
watch(locale, () => {
  applyFilters();
});

const filters = ref<RecipeFilter>({
  search: null,
  ingredient: [],
  tags: [],
  minPrep: null,
  maxPrep: null,
  minCook: null,
  maxCook: null,
  minSteps: null,
  maxSteps: null,
  dateFrom: null,
  dateTo: null,
})
function resetFilters() {
  filters.value = {
    search: "",
    ingredient: [],
    tags: [],
    minPrep: null,
    maxPrep: null,
    minCook: null,
    maxCook: null,
    minSteps: null,
    maxSteps: null,
    dateFrom: null,
    dateTo: null,
  }
  applyFilters()
}

async function applyFilters() {
  loading.value = true
  try {
    recipes.value = await getAllRecipes(filters.value, false)
  } catch (err) {
    console.error("Failed to fetch recipes:", err)
  } finally {
    loading.value = false
  }
}
// Fetch recipes on mount
onMounted( () => {
  applyFilters()
});
</script>

<template>
  <div class="container mx-auto p-4 space-y-8">
    <div class="flex flex-col gap-2">
      <h1 class="text-3xl font-bold tracking-tight md:text-4xl">{{ t('RecipeList.Title') }}</h1>
      <p class="text-muted-foreground">{{ t('RecipeList.Subtitle') || 'Discover culinary secrets from around the world.' }}</p>
    </div>

    <div class="bg-card rounded-xl border p-4 shadow-sm">
      <Filter v-model="filters" @search="applyFilters" />
    </div>

    <div v-if="loading" class="flex flex-col justify-center items-center h-64 gap-4">
      <Spinner class="h-8 w-8 text-primary" />
      <p class="text-sm text-muted-foreground animate-pulse">{{ t('Common.Loading') }}</p>
    </div>

    <div v-else-if="recipes.length > 0" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
      <RouterLink
          v-for="recipe in recipes"
          :key="recipe.id"
          :to="ROUTES.RECIPE(recipe.id)"
          class="group flex flex-col bg-card rounded-xl border shadow-sm transition-all hover:shadow-md hover:-translate-y-1 overflow-hidden"
      >
        <div class="aspect-video w-full overflow-hidden bg-muted relative">
          <img
              v-if="recipe.image_url"
              :src="$apiUrl + recipe.image_url"
              class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-110"
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

          <p class="text-muted-foreground text-sm mb-4 line-clamp-2 h-10 italic" v-if="recipe.description">
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
    </div>

    <div v-else class="flex flex-col items-center justify-center py-20 text-center border rounded-xl bg-muted/10">
      <Search class="h-12 w-12 text-muted-foreground mb-4 opacity-20" />
      <h3 class="text-xl font-semibold">{{ t('RecipeList.NoResults') || 'No recipes found' }}</h3>
      <p class="text-muted-foreground max-w-xs mx-auto mt-2">
        {{ t('RecipeList.NoResultsDesc') || 'Try adjusting your filters or search terms.' }}
      </p>
      <Button variant="outline" class="mt-6" @click="resetFilters">
        {{ t('Admin.filters.reset') }}
      </Button>
    </div>
  </div>
</template>
