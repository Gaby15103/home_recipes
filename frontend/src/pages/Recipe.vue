<script setup lang="ts">
import {ref, onMounted, watch, nextTick, useTemplateRef} from "vue";
import {useRoute} from "vue-router";
import {getRecipeById, getComments, addComment, favoriteRecipe, getFavorites, getRating} from "@/api/recipe";
import PrintModal from "@/components/printer/PrintModal.vue";
import {Separator} from "@/components/ui/separator";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card";
import {Badge} from "@/components/ui/badge";
import {Skeleton} from "@/components/ui/skeleton";
import {Button} from "@/components/ui/button";
import {Checkbox} from "@/components/ui/checkbox";
import {Switch} from "@/components/ui/switch";
import {Label} from '@/components/ui/label'
import type {Recipe, RecipeComment, RecipeRating} from "@/models/Recipe.ts";
import type {RecipeCommentCreate} from "@/models/RecipeCreate.ts";
import {useAuthStore} from "@/stores/auth.ts";
import CommentThread from "@/components/TextArea/CommentEditor/CommentThread.vue";
import { useI18n } from "vue-i18n"
const { t } = useI18n()
const authStore = useAuthStore();

const route = useRoute();
const highlighted = ref<string | null>(null);
const showStepImages = ref(true);

const recipe = ref<Recipe | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);

// Comments and rating/favorites state
const comments = ref<RecipeComment[]>([]);
const newComment = ref<RecipeCommentCreate>({
  recipe_id: "",
  content: "",
  user_id: null,
  parent_id: null,
});
const rating = ref<RecipeRating | null>(null)
const favorited = ref(false);
const favoriteLoading = ref(false);

onMounted(async () => {
  loading.value = true;
  try {
    recipe.value = await getRecipeById(route.params.id as string);

    if (recipe.value) {
      newComment.value.recipe_id = recipe.value.id;
      // Fetch comments
      comments.value = await getComments(recipe.value.id);

      // Fetch rating
      rating.value = await getRating(recipe.value.id);

      // Fetch favorite status
      const favs = await getFavorites();
      favorited.value = favs.some(f => f.id === recipe.value!.id);
    }
  } catch (err: any) {
    error.value = err.message || t("recipe.errors.fetch");
  } finally {
    loading.value = false;
  }
});

watch(
    () => route.hash,
    async (newHash) => {
      if (!newHash) return;
      const id = newHash.slice(1);
      const el = document.getElementById(id);
      if (el) {
        el.scrollIntoView({behavior: "smooth", block: "start"});
        highlighted.value = id;
        await nextTick();
        setTimeout(() => (highlighted.value = null), 1000);
      }
    },
    {immediate: true}
);

const printModal = useTemplateRef<typeof PrintModal>('printModal');

// Function to open modal
function openPrintModal() {
  printModal.value?.showModal();
}

function shareRecipe() {
  navigator.clipboard.writeText(window.location.href);
  alert(t("recipe.actions.copied"))
}

// Toggle favorite
async function toggleFavorite() {
  if (!recipe.value) return;
  favoriteLoading.value = true;
  try {
    await favoriteRecipe(recipe.value.id);
    favorited.value = !favorited.value;
  } finally {
    favoriteLoading.value = false;
  }
}

// Post comment
async function postComment() {
  if (!recipe.value || !newComment.value.content.trim() || !authStore.user) return;
  newComment.value.user_id = authStore.user.id;
  console.log("fuck you")
  const added = await addComment(recipe.value.id, newComment.value);
  comments.value.push(added);
  newComment.value = {
    recipe_id: recipe.value.id,
    content: "",
    user_id: null,
    parent_id: null,
  };
}
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 py-8 space-y-12">
    <!-- Loading Skeleton -->
    <div v-if="loading" class="animate-pulse space-y-6">
      <Skeleton class="h-96 w-full rounded-xl"/>
      <Skeleton class="h-10 w-1/2"/>
      <Skeleton class="h-6 w-1/3"/>
    </div>

    <!-- Error -->
    <p v-else-if="error" class="text-red-500 text-center">{{ error }}</p>

    <div v-else-if="recipe" class="space-y-12">
      <!-- Header -->
      <Card class="shadow-md">
        <CardContent class="flex flex-col md:flex-row gap-6">
          <div class="shrink-0 md:w-1/2 rounded-xl overflow-hidden border dark:border-gray-700">
            <img
                :src="$apiUrl + recipe.image_url"
                :alt="recipe.title"
                class="w-full h-full object-cover"
            />
          </div>
          <div class="flex-1 flex flex-col justify-between space-y-4">
            <div class="space-y-2">
              <h1 class="text-4xl font-serif font-bold leading-tight">{{ recipe.title }}</h1>

              <!-- Rating -->
              <div v-if="rating" class="flex items-center gap-2">
                <template v-for="i in 5" :key="i">
                  <span class="text-yellow-400">
                    {{ i <= Math.round(rating.average) ? "★" : "☆" }}
                  </span>
                </template>
                <span class="text-gray-500 dark:text-gray-400">
                  ({{ rating.count }})
                </span>
              </div>


              <!-- Times & Servings -->
              <ul class="flex flex-wrap gap-4 text-sm text-gray-700 dark:text-gray-300">
                <li>
                  {{ t("recipe.meta.prep") }}:
                  {{ recipe.prep_time_minutes }} {{ t("recipe.meta.minutes") }}
                </li>

                <li>
                  {{ t("recipe.meta.cook") }}:
                  {{ recipe.cook_time_minutes }} {{ t("recipe.meta.minutes") }}
                </li>

                <li>
                  {{ t("recipe.meta.servings") }}:
                  {{ recipe.servings }}
                </li>

              </ul>

              <!-- Description -->
              <p class="text-gray-700 dark:text-gray-300">{{ recipe.description }}</p>

              <!-- Tags -->
              <div v-if="recipe.tags?.length" class="mt-2">
                <h3 class="font-semibold text-gray-800 dark:text-gray-200 mb-1">
                  {{ t("recipe.tags") }}:
                </h3>
                <div class="flex flex-wrap gap-2">
                  <Badge
                      v-for="tag in recipe.tags"
                      :key="tag.id"
                      variant="outline"
                      class="text-xs px-3 py-1 rounded-full border-primary dark:border-primary font-semibold"
                  >
                    {{ tag.name }}
                  </Badge>
                </div>
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="flex flex-wrap gap-3 mt-4">
              <Button @click="toggleFavorite" :disabled="favoriteLoading">
                {{ favorited
                  ? `💖 ${t("recipe.actions.favorited")}`
                  : `🤍 ${t("recipe.actions.favorite")}`
                }}
              </Button>
              <PrintModal ref="printModal" :recipe="recipe"/>
              <Button @click="openPrintModal">
                🖨 {{ t("recipe.actions.print") }}
              </Button>
              <Button @click="shareRecipe">
                🔗 {{ t("recipe.actions.share") }}
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- Ingredients Section -->
      <Card class="shadow-md">
        <CardHeader>
          <CardTitle class="text-2xl">
            {{ t("recipe.ingredients.title") }}
          </CardTitle>
        </CardHeader>
        <CardContent class="space-y-10 mb-10">
          <template v-for="group in recipe.ingredient_groups" :key="group.position">
            <div v-if="group.title" class="font-semibold text-gray-800 dark:text-gray-200 mb-1">
              {{ group.title }}
            </div>
            <ul class="space-y-2">
              <li
                  v-for="ing in group.ingredients"
                  :key="ing.position"
                  class="flex items-start gap-3"
              >
                <Checkbox
                    type="checkbox"
                    :id="`ingredient-${group.position}-${ing.position}`"
                    class="mt-1 accent-primary scale-150 m-2"/>
                <label :for="`ingredient-${group.position}-${ing.position}`" class="text-gray-700 dark:text-gray-300">
                  <span class="font-medium">{{ ing.quantity }} {{ ing.unit }}</span>
                  {{ ing.name }}
                  <span v-if="ing.note" class="text-red-500 font-bold ml-1">*</span>
                </label>
              </li>
            </ul>
          </template>
        </CardContent>

        <Separator class="max-w-[95%] mx-auto min-h-0.5 rounded-xl"/>

        <CardHeader class="flex justify-between">
          <CardTitle class="text-2xl">
            {{ t("recipe.steps.title") }}
          </CardTitle>
          <div class="grid grid-cols-1 gap-6 justify-items-end">
            <Label for="showStepImagesId">
              {{ t("recipe.steps.showImages") }}
            </Label>
            <Switch if="showStepImagesId" v-model="showStepImages"/>
          </div>
        </CardHeader>
        <CardContent class="space-y-10 mb-10">
          <template v-for="group in recipe.step_groups" :key="group.position">
            <div v-if="group.title" class="font-semibold text-gray-800 dark:text-gray-200 mb-1">
              {{ group.title }}
            </div>
            <ul class="pl-5 space-y-4">
              <li v-for="step in group.steps" :key="step.position" class="space-y-3">
                <div class="flex flex-col gap-1 text-gray-700 dark:text-gray-300">
                  <div>
                    <Checkbox class="scale-150 m-2"/>
                    {{ step.instruction }}
                  </div>
                  <span v-if="step.duration_minutes" class="text-sm text-gray-500 dark:text-gray-400 ml-1">
                      ({{ step.duration_minutes }} min)
                    </span>
                </div>
                <div v-if="step.image_url && showStepImages"
                     class="rounded-lg overflow-hidden border dark:border-gray-700">
                  <img
                      :src="$apiUrl + step.image_url"
                      alt="Step image"
                      class="w-full max-h-96 object-cover"
                  />
                </div>
              </li>
            </ul>
          </template>
        </CardContent>
      </Card>

      <Separator/>
      <Card class="shadow-md">
        <CardHeader>
          <CardTitle>
            {{ t("recipe.comments.title") }}
          </CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="space-y-2">
        <textarea
            v-model="newComment.content"
            :placeholder="t('recipe.comments.placeholder')"
            class="w-full border rounded-lg p-2"
        />
            <Button @click="postComment">
              {{ t("recipe.comments.post") }}
            </Button>
          </div>
          <div class="space-y-4">
            <CommentThread
                :comments="comments"
                :recipe-id="recipe!.id"
            />
          </div>
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
