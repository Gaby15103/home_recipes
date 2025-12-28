<script setup lang="ts">
import { ref, onMounted } from "vue";
import type { Recipe } from "@/models/Recipe.ts";
import { getAllRecipes } from "@/api/recipe";

const recipes = ref<Recipe[]>([]);
const loading = ref(true);

onMounted(async () => {
  try {
    recipes.value = await getAllRecipes(true);
  } catch (err) {
    console.error("Failed to fetch recipes:", err);
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <div class="container mx-auto p-4">
    <h1 class="text-3xl font-bold mb-6">All Recipes</h1>

    <!-- Loading spinner / placeholder -->
    <div v-if="loading" class="flex justify-center items-center h-32">
      <div class="loader"></div>
    </div>

    <!-- Recipes grid -->
    <div v-else class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
      <div
          v-for="recipe in recipes"
          :key="recipe.title + recipe.author_id"
          class="border rounded shadow hover:shadow-lg transition p-4 flex flex-col"
      >
        <!-- Temporary placeholder image -->
        <div class="bg-gray-200 h-40 w-full mb-4 flex items-center justify-center text-gray-500">
          Image
        </div>

        <!-- Recipe info -->
        <h2 class="text-xl font-semibold mb-2">{{ recipe.title }}</h2>
        <p class="text-gray-600 text-sm mb-2" v-if="recipe.description">
          {{ recipe.description }}
        </p>
        <p class="text-gray-500 text-xs mb-1">Servings: {{ recipe.serving }}</p>
        <p class="text-gray-500 text-xs">
          Prep: {{ recipe.prep_time_minutes }} min | Cook: {{ recipe.cook_time_minutes }} min
        </p>

        <!-- Tags -->
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
    </div>
  </div>
</template>

<style scoped>
.loader {
  border: 4px solid #f3f3f3;
  border-top: 4px solid #3490dc;
  border-radius: 50%;
  width: 36px;
  height: 36px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>