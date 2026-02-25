<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";

// UI Components
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";

// API & Logic
import { getRecipeById, deleteRecipe } from "@/api/recipe.ts";
import { ROUTES } from "@/router/routes.ts";
import type { RecipeView } from "@/models/Recipe.ts";
import RecipeDisplay from "@/components/recipe/RecipeDisplay.vue";
import {Badge} from "@/components/ui/badge";

const { t, locale } = useI18n();
const route = useRoute();
const router = useRouter();

const recipe = ref<RecipeView | null>(null);
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

watch(locale, () => fetchRecipe());
onMounted(fetchRecipe);

function goToEdit() {
  if (!recipe.value) return;
  router.push(ROUTES.ADMIN.RECIPE.EDIT(recipe.value.id));
}

async function removeRecipe() {
  if (!recipe.value) return;
  const confirmed = confirm(t('Admin.recipe.confirmDelete'));
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

    <div v-if="loading" class="space-y-6">
      <Skeleton class="h-[400px] w-full rounded-xl" />
      <div class="space-y-2">
        <Skeleton class="h-10 w-1/2" />
        <Skeleton class="h-4 w-full" />
      </div>
    </div>

    <div v-else-if="error" class="text-center py-12">
      <p class="text-red-500 mb-4">{{ error }}</p>
      <Button @click="fetchRecipe">{{ t('common.retry') }}</Button>
    </div>

    <div v-else-if="recipe" class="space-y-8">

      <RecipeDisplay
          :recipe="recipe"
          :is-admin="true"
          :multiplier="1"
      >
        <template #rating>
          <div class="flex flex-col gap-1 text-xs text-muted-foreground">
            <div>
              <strong>ID:</strong> {{ recipe.id }}
            </div>
            <div>
              <strong>{{ t('Admin.recipe.view.author') }}:</strong>
              {{ recipe.author }} <span class="opacity-70">({{ recipe.author_id }})</span>
            </div>
          </div>
        </template>

        <template #header-actions>
          <Button @click="goToEdit" class="bg-blue-600 hover:bg-blue-700">
            ✏️ {{ t('Admin.recipe.editTitle') }}
          </Button>
          <Button variant="destructive" @click="removeRecipe">
            🗑 {{ t('Admin.common.delete') }}
          </Button>
        </template>

        <template #ingredient-toolbar>
          <Badge variant="outline" class="ml-auto font-mono text-[10px]">
            UID: {{ recipe.id.split('-')[0] }}...
          </Badge>
        </template>

      </RecipeDisplay>

    </div>
  </div>
</template>