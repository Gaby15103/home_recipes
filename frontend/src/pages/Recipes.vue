<script setup lang="ts">
import { ref, onMounted } from "vue";
import type { Recipe } from "@/models/Recipe.ts";
import { getAllRecipes } from "@/api/recipe";
import {RouterLink} from "vue-router";
import {Spinner} from "@/components/ui/spinner";
import { useI18n } from "vue-i18n"
const { t } = useI18n()
// State for recipes
const recipes = ref<Recipe[]>([]);
const loading = ref(true);

// Fetch recipes on mount
onMounted(async () => {
  try {
    recipes.value = await getAllRecipes();
  } catch (err) {
    console.error("Failed to fetch recipes:", err);
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <div class="container mx-auto p-4">
    <h1 class="text-3xl font-bold mb-6">{{ t('RecipeList.Title') }}</h1>

    <div v-if="loading" class="flex justify-center items-center h-32">
      <Spinner />
    </div>

    <div v-else class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
      <RouterLink
          v-for="recipe in recipes"
          :key="recipe.title + recipe.author_id"
          :to="`/recipe/${recipe.id}`"
      >
        <div

            class="border rounded shadow hover:shadow-lg transition p-4 flex flex-col"
        >
          <img :src="$apiUrl+recipe.image_url" class="h-40 w-full mb-4 flex items-center justify-center">

          <h2 class="text-xl font-semibold mb-2">{{ recipe.title }}</h2>
          <p class="text-gray-600 text-sm mb-2" v-if="recipe.description">
            {{ recipe.description }}
          </p>
          <p class="text-gray-500 text-xs mb-1">Servings: {{ recipe.servings }}</p>
          <p class="text-gray-500 text-xs">
            Prep: {{ recipe.prep_time_minutes }} min | Cook: {{ recipe.cook_time_minutes }} min
          </p>

          <div class="mt-2 flex flex-wrap gap-1">
          <span
              v-for="tag in recipe.tags"
              :key="tag.id"
              class="text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded"
          >
            {{ tag.name }}
          </span>
          </div>
        </div>
      </RouterLink>
    </div>
  </div>
</template>
