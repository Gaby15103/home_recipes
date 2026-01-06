<script setup lang="ts">
import {ref, onMounted, watch, nextTick, useTemplateRef} from "vue";
import {useRoute} from "vue-router";
import {getRecipeById} from "@/api/recipe";
import { printRecipe } from "@/utils/RecipePrinter.ts";
import PrintModal from "@/components/printer/PrintModal.vue";
import {Separator} from "@/components/ui/separator";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card";
import {Badge} from "@/components/ui/badge";
import {Skeleton} from "@/components/ui/skeleton";
import {Button} from "@/components/ui/button";
import {Checkbox} from "@/components/ui/checkbox";
import {Switch} from "@/components/ui/switch";
import {Label} from '@/components/ui/label'

const route = useRoute();
const highlighted = ref<string | null>(null);
const showStepImages = ref(true);

const {recipe, loading, error, fetchRecipe} = getRecipeById(route.params.id as string);

onMounted(fetchRecipe);

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
  alert("Recipe URL copied to clipboard!");
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
          <div class="flex-shrink-0 md:w-1/2 rounded-xl overflow-hidden border dark:border-gray-700">
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
              <div class="flex items-center gap-2">
                <template v-for="i in 5" :key="i">
                  <span class="text-yellow-400" v-html="i <= 3 ? '★' : '☆'"/>
                </template>
                <span class="text-gray-500 dark:text-gray-400">(3)</span>
              </div>

              <!-- Times & Servings -->
              <ul class="flex flex-wrap gap-4 text-sm text-gray-700 dark:text-gray-300">
                <li>Prep: {{ recipe.prep_time_minutes }} min</li>
                <li>Cook: {{ recipe.cook_time_minutes }} min</li>
                <li>Servings: {{ recipe.serving }}</li>
              </ul>

              <!-- Description -->
              <p class="text-gray-700 dark:text-gray-300">{{ recipe.description }}</p>

              <!-- Tags -->
              <div v-if="recipe.tags?.length" class="mt-2">
                <h3 class="font-semibold text-gray-800 dark:text-gray-200 mb-1">Tags:</h3>
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
              <Button>❤️ Add to Favorites</Button>
              <PrintModal ref="printModal" :recipe="recipe" />
              <Button @click="openPrintModal">🖨 Print</Button>
              <Button @click="shareRecipe">🔗 Share</Button>
            </div>
          </div>
        </CardContent>
      </Card>

      <Separator/>

      <!-- Ingredients Section -->
      <Card class="shadow-md">
        <CardHeader>
          <CardTitle class="text-2xl">Ingredients</CardTitle>
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

        <Separator/>
        <CardHeader class="flex justify-between">
          <CardTitle class="text-2xl">Preparations</CardTitle>
          <div>
            <Label for="showStepImagesId" >Show Step Images:</Label>
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

      <!-- Comments Section -->
      <Card class="shadow-md">
        <CardHeader>
          <CardTitle>Comments</CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          <!-- Comment form -->
          <div class="space-y-2">
            <textarea
                placeholder="Write a comment..."
                class="w-full border rounded-lg p-2 text-gray-800 dark:text-gray-200 dark:bg-gray-800 border-gray-300 dark:border-gray-700"
            ></textarea>
            <Button class="shadcn-button shadcn-button-primary">Post Comment</Button>
          </div>

          <!-- Placeholder for comments -->
          <div class="space-y-2">
            <div v-for="i in 3" :key="i"
                 class="p-3 border rounded-lg bg-gray-50 dark:bg-gray-900 border-gray-200 dark:border-gray-700">
              <p class="font-semibold text-gray-800 dark:text-gray-200">User {{ i }}</p>
              <p class="text-gray-700 dark:text-gray-300">This is a sample comment. Comments will appear here once
                implemented.</p>
            </div>
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
