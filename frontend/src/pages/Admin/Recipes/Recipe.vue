<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { getRecipeById, deleteRecipe } from "@/api/recipe";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import type { Recipe } from "@/models/Recipe";
import { ROUTES } from "@/router/routes.ts";

const route = useRoute();
const router = useRouter();

const recipe = ref<Recipe | null>(null);
const loading = ref(true);
const error = ref<string | null>(null);

async function fetchRecipe() {
  loading.value = true;
  try {
    recipe.value = await getRecipeById(route.params.id as string);
  } catch (err: any) {
    error.value = err.message ?? "Failed to fetch recipe";
  } finally {
    loading.value = false;
  }
}

onMounted(fetchRecipe);

function goToEdit() {
  if (!recipe.value) return;
  router.push(ROUTES.ADMIN.RECIPE.EDIT(recipe.value.id));
}

async function removeRecipe() {
  if (!recipe.value) return;
  const confirmed = confirm("Are you sure you want to delete this recipe?");
  if (!confirmed) return;

  try {
    await deleteRecipe(recipe.value.id);
    router.push(ROUTES.ADMIN.RECIPE.LIST);
  } catch (err: any) {
    alert(err.message ?? "Failed to delete recipe");
  }
}
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 py-8 space-y-8">

    <!-- Loading skeleton -->
    <div v-if="loading" class="space-y-6">
      <Skeleton class="h-80 w-full rounded-xl" />
      <Skeleton class="h-10 w-1/2" />
    </div>

    <!-- Error -->
    <p v-else-if="error" class="text-red-500 text-center">{{ error }}</p>

    <!-- Recipe content -->
    <div v-else-if="recipe" class="space-y-8">

      <!-- Header / Main Info -->
      <Card>
        <CardContent class="flex flex-col md:flex-row gap-6">
          <img
              v-if="recipe.image_url"
              :src="$apiUrl + recipe.image_url"
              alt="Recipe Image"
              class="w-full md:w-1/2 rounded-xl object-cover border"
          />

          <div class="flex-1 space-y-4">
            <h1 class="text-3xl font-bold">{{ recipe.title }}</h1>
            <p class="text-gray-600 dark:text-gray-300">{{ recipe.description }}</p>

            <!-- Metadata -->
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div><strong>Author:</strong> {{ recipe.author }} (ID: {{ recipe.author_id }})</div>
              <div><strong>Servings:</strong> {{ recipe.servings }}</div>
              <div><strong>Prep:</strong> {{ recipe.prep_time_minutes }} min</div>
              <div><strong>Cook:</strong> {{ recipe.cook_time_minutes }} min</div>
              <div>
                <strong>Visibility:</strong>
                <Badge :variant="recipe.is_private ? 'destructive' : 'outline'">
                  {{ recipe.is_private ? "Private" : "Public" }}
                </Badge>
              </div>
              <div>
                <strong>Tags:</strong>
                <div class="flex gap-1 flex-wrap">
                  <Badge v-for="tag in recipe.tags" :key="tag.id" variant="secondary">{{ tag.name ?? tag.id }}</Badge>
                </div>
              </div>
            </div>

            <!-- Admin actions -->
            <div class="flex gap-3 pt-4">
              <Button @click="goToEdit">✏️ Edit Recipe</Button>
              <Button variant="destructive" @click="removeRecipe">🗑 Delete Recipe</Button>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- Ingredients -->
      <Card>
        <CardHeader>
          <CardTitle>Ingredients</CardTitle>
        </CardHeader>
        <CardContent class="space-y-6">
          <template v-for="group in recipe.ingredient_groups" :key="group.id">
            <h3 class="font-semibold">
              {{ group.title || "Ingredients" }} (ID: {{ group.id }})
            </h3>
            <ul class="space-y-2">
              <li v-for="ing in group.ingredients" :key="ing.id" class="flex gap-2 items-start">
                <Checkbox disabled />
                <span>
                  <strong>{{ ing.quantity }} {{ ing.unit }}</strong> {{ ing.name }}
                  <span class="text-gray-400 ml-1">(ID: {{ ing.id }})</span>
                  <span v-if="ing.note" class="text-red-500 ml-1">*</span>
                </span>
              </li>
            </ul>
          </template>
        </CardContent>
      </Card>

      <!-- Steps -->
      <Card>
        <CardHeader>
          <CardTitle>Preparation Steps</CardTitle>
        </CardHeader>
        <CardContent class="space-y-6">
          <template v-for="group in recipe.step_groups" :key="group.id">
            <h3 class="font-semibold">
              {{ group.title || "Steps" }} (ID: {{ group.id }})
            </h3>
            <ol class="space-y-4 list-decimal pl-5">
              <li v-for="step in group.steps" :key="step.id">
                <p>{{ step.instruction }}</p>
                <p v-if="step.duration_minutes" class="text-sm text-gray-500">{{ step.duration_minutes }} min</p>
                <img
                    v-if="step.image_url"
                    :src="$apiUrl + step.image_url"
                    alt="Step image"
                    class="rounded-lg border mt-2 max-h-64 object-cover"
                />
                <p class="text-gray-400 text-xs mt-1">ID: {{ step.id }}</p>
              </li>
            </ol>
          </template>
        </CardContent>
      </Card>

    </div>
  </div>
</template>

<style scoped>
.accent-primary {
  accent-color: hsl(var(--primary));
}
</style>
