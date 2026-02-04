<script setup lang="ts">
import {ref, onMounted, watch} from "vue";
import type {Recipe, RecipeFilter} from "@/models/Recipe.ts";
import { getAllRecipes } from "@/api/recipe";
import Filter from "@/components/Recipe/Filter.vue"
import { debounce } from "lodash-es"
import {Spinner} from "@/components/ui/spinner";
import {useI18n} from "vue-i18n";
const { t } = useI18n()
const recipes = ref<Recipe[]>([]);
const loading = ref(true);

onMounted(() => {
  applyFilters()
})


const filters = ref<RecipeFilter>({
  search: "",
  ingredient: "",
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

const debouncedApply = debounce(applyFilters, 400)

watch(filters, debouncedApply, { deep: true })


async function applyFilters() {
  loading.value = true
  try {
    recipes.value = await getAllRecipes(true, filters.value)
  } catch (err) {
    console.error("Failed to fetch recipes:", err)
  } finally {
    loading.value = false
  }
}


</script>

<template>
  <div class="container mx-auto p-4">
    <h1 class="text-3xl font-bold mb-6">{{ t('Admin.recipe.list.title') }}</h1>

    <Filter v-model="filters" />

    <div v-if="loading" class="flex justify-center items-center h-32">
      <Spinner/>
    </div>

    <div v-else class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
      <div
          v-for="recipe in recipes"
          :key="recipe.title + recipe.author_id"
          class="border rounded shadow hover:shadow-lg transition p-4 flex flex-col"
      >
        <img :src="$apiUrl+recipe.image_url" class="h-40 w-full mb-4 flex items-center justify-center">

        <h2 class="text-xl font-semibold mb-2">{{ recipe.title }}</h2>
        <p class="text-gray-600 text-sm mb-2" v-if="recipe.description">
          {{ recipe.description }}
        </p>
        <p class="text-gray-500 text-xs mb-1">{{ t('Admin.recipe.list.servings') }}: {{ recipe.servings }}</p>
        <p class="text-gray-500 text-xs">
          {{ t('Admin.recipe.list.prep') }}: {{ recipe.prep_time_minutes }} min | {{ t('Admin.recipe.list.cook') }}: {{ recipe.cook_time_minutes }} min
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
    </div>
  </div>
</template>