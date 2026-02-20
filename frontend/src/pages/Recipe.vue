<script setup lang="ts">
import { ref, onMounted, watch, nextTick, useTemplateRef } from "vue";
import { useRoute } from "vue-router";
import { useI18n } from "vue-i18n";
import axios from "axios";

// UI Components
import PrintModal from "@/components/printer/PrintModal.vue";
import CommentThread from "@/components/TextArea/CommentEditor/CommentThread.vue";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Separator } from "@/components/ui/separator";

// API & Stores
import {
  getRecipeById,
  getComments,
  addComment,
  favoriteRecipe,
  getFavorites,
  getRating
} from "@/api/recipe";
import { useAuthStore } from "@/stores/auth.ts";

// Types
import type { RecipeView, RecipeComment, RecipeRating } from "@/models/Recipe.ts";
import type { RecipeCommentCreate } from "@/models/RecipeCreate.ts";
import type { Unit } from "@/models/Recipe";
import RecipeDisplay from "@/components/Recipe/RecipeDisplay.vue";

const { t, locale } = useI18n();
const route = useRoute();
const authStore = useAuthStore();

// --- State ---
const recipe = ref<RecipeView | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);

// Scaling & Units
const multiplier = ref(1);
const allUnits = ref<Unit[]>([]);
const unitOverrides = ref<Record<string, string>>({});

// Social/Interaction
const comments = ref<RecipeComment[]>([]);
const rating = ref<RecipeRating | null>(null);
const favorited = ref(false);
const favoriteLoading = ref(false);
const newComment = ref<RecipeCommentCreate>({
  recipe_id: "",
  content: "",
  user_id: null,
  parent_id: null,
});

// --- Actions ---

async function loadRecipe() {
  loading.value = true;
  try {
    recipe.value = await getRecipeById(route.params.id as string);
  } catch (err: any) {
    error.value = err.message || t("recipe.errors.fetch");
  } finally {
    loading.value = false;
  }
}

async function fetchUnits() {
  try {
    const res = await axios.get("/units");
    allUnits.value = res.data;
  } catch (err) {
    console.error("Could not load units for converter", err);
  }
}

async function loadMeta() {
  if (!recipe.value) return;
  try {
    const [ratingRes, favsRes, commentsRes] = await Promise.all([
      getRating(recipe.value.id),
      getFavorites(),
      getComments(recipe.value.id)
    ]);
    rating.value = ratingRes;
    favorited.value = favsRes.some(f => f.id === recipe.value!.id);
    comments.value = commentsRes;
  } catch (err) {
    console.warn("Meta data partially failed to load", err);
  }
}

function handleUnitOverride(ingredientId: string, unitId: string) {
  unitOverrides.value[ingredientId] = unitId;
}

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

async function postComment() {
  if (!recipe.value || !newComment.value.content.trim() || !authStore.user) return;
  newComment.value.user_id = authStore.user.id;
  newComment.value.recipe_id = recipe.value.id;
  try {
    const added = await addComment(recipe.value.id, newComment.value);
    comments.value.push(added);
    newComment.value.content = "";
  } catch (err) {
    alert(t("comments.errors.post"));
  }
}

function shareRecipe() {
  navigator.clipboard.writeText(window.location.href);
  alert(t("recipe.actions.copied"));
}

const printModal = useTemplateRef<typeof PrintModal>('printModal');
function openPrintModal() {
  printModal.value?.showModal();
}

// --- Lifecycle & Watchers ---

onMounted(async () => {
  await Promise.all([loadRecipe(), fetchUnits()]);
  await loadMeta();
});

watch(locale, () => loadRecipe());

watch(
    () => route.hash,
    async (newHash) => {
      if (!newHash) return;
      await nextTick();
      const id = newHash.slice(1);
      const el = document.getElementById(id);
      if (el) el.scrollIntoView({ behavior: "smooth", block: "start" });
    },
    { immediate: true }
);
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 py-8 space-y-12">
    <div v-if="loading" class="animate-pulse space-y-8">
      <Skeleton class="h-[400px] w-full rounded-xl" />
      <div class="space-y-2">
        <Skeleton class="h-10 w-1/2" />
        <Skeleton class="h-4 w-full" />
      </div>
    </div>

    <div v-else-if="error" class="text-center py-20">
      <p class="text-destructive text-lg font-semibold">{{ error }}</p>
      <Button variant="link" @click="loadRecipe">{{ t("common.retry") }}</Button>
    </div>

    <div v-else-if="recipe" class="animate-in fade-in duration-500">
      <RecipeDisplay
          :recipe="recipe"
          :multiplier="multiplier"
          :all-units="allUnits"
          :unit-overrides="unitOverrides"
          @update:unit-override="handleUnitOverride"
      >
        <template #rating>
          <div v-if="rating" class="flex items-center gap-1.5">
            <template v-for="i in 5" :key="i">
              <span class="text-xl" :class="i <= Math.round(rating.average) ? 'text-yellow-400' : 'text-gray-300'">
                {{ i <= Math.round(rating.average) ? "★" : "☆" }}
              </span>
            </template>
            <span class="text-sm text-muted-foreground ml-2">({{ rating.count }} {{ t("recipe.ratings") }})</span>
          </div>
        </template>

        <template #header-actions>
          <Button
              @click="toggleFavorite"
              :variant="favorited ? 'default' : 'outline'"
              :disabled="favoriteLoading"
              class="transition-all"
          >
            {{ favorited ? `💖 ${t("recipe.actions.favorited")}` : `🤍 ${t("recipe.actions.favorite")}` }}
          </Button>

          <PrintModal ref="printModal" :recipe="recipe" />
          <Button variant="outline" @click="openPrintModal">
            🖨 {{ t("recipe.actions.print") }}
          </Button>

          <Button variant="outline" @click="shareRecipe">
            🔗 {{ t("recipe.actions.share") }}
          </Button>
        </template>

        <template #ingredient-toolbar>
          <div class="flex items-center gap-2 bg-muted p-1 rounded-lg">
            <span class="text-xs font-medium px-2 text-muted-foreground hidden sm:inline">
              {{ t("recipe.meta.servings") }}
            </span>
            <Button
                v-for="m in [0.5, 1, 2, 3]"
                :key="m"
                variant="ghost"
                size="sm"
                class="h-7 px-3 text-xs"
                :class="{ 'bg-background shadow-sm': multiplier === m }"
                @click="multiplier = m"
            >
              {{ m }}x
            </Button>
          </div>
        </template>
      </RecipeDisplay>

      <div class="mt-16 space-y-8">
        <Separator />
        <div class="max-w-4xl mx-auto">
          <h3 class="text-2xl font-bold mb-6">{{ t("recipe.comments.title") }}</h3>

          <div v-if="authStore.user" class="space-y-4 mb-10">
            <textarea
                v-model="newComment.content"
                :placeholder="t('recipe.comments.placeholder')"
                class="w-full min-h-[100px] border rounded-xl p-4 focus:ring-2 focus:ring-primary outline-none transition-all"
            />
            <div class="flex justify-end">
              <Button @click="postComment" :disabled="!newComment.content.trim()">
                {{ t("recipe.comments.post") }}
              </Button>
            </div>
          </div>
          <p v-else class="text-sm text-muted-foreground mb-10 italic">
            {{ t("recipe.comments.loginToPost") }}
          </p>

          <CommentThread
              :comments="comments"
              :recipe-id="recipe.id"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Ensure checkboxes match brand colors */
:deep(.accent-primary) {
  accent-color: hsl(var(--primary));
}
</style>