<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Checkbox } from "@/components/ui/checkbox";
import { Switch } from "@/components/ui/switch";
import { Label } from '@/components/ui/label';
import type { RecipeView } from "@/models/Recipe.ts";

const props = defineProps<{
  recipe: RecipeView;
  isAdmin?: boolean;
  multiplier: number;
}>();

const { t } = useI18n();
const showStepImages = ref(true);

/**
 * Extracts the symbol string from the unit object or returns the string directly.
 */
function getBaseUnitSymbol(ingUnit: any): string {
  if (ingUnit && typeof ingUnit === 'object') {
    return ingUnit.symbol;
  }
  return ingUnit || "";
}

/**
 * Simply scales the quantity by the multiplier.
 */
function getScaledQuantity(ing: any) {
  const quantity = ing.quantity * props.multiplier;
  return Number(quantity.toFixed(2)).toString();
}
</script>

<template>
  <div class="space-y-12">
    <Card class="shadow-md overflow-hidden">
      <CardContent class="flex flex-col md:flex-row gap-6 p-0 md:p-6">
        <div class="shrink-0 md:w-1/2 h-64 md:h-auto overflow-hidden">
          <img :src="$apiUrl + recipe.image_url" :alt="recipe.title" class="w-full h-full object-cover" />
        </div>
        <div class="flex-1 flex flex-col justify-between p-6 md:p-0 space-y-4">
          <div class="space-y-3">
            <div class="flex justify-between items-start gap-2">
              <h1 class="text-3xl md:text-4xl font-serif font-bold leading-tight">{{ recipe.title }}</h1>
              <Badge v-if="isAdmin" :variant="recipe.is_private ? 'destructive' : 'outline'" class="whitespace-nowrap">
                {{ recipe.is_private ? t('Admin.recipe.view.private') : t('Admin.recipe.view.public') }}
              </Badge>
            </div>

            <slot name="rating" />

            <ul class="flex flex-wrap gap-x-6 gap-y-2 text-sm text-muted-foreground">
              <li><strong>{{ t("recipe.meta.prep") }}:</strong> {{ recipe.prep_time_minutes }}m</li>
              <li><strong>{{ t("recipe.meta.cook") }}:</strong> {{ recipe.cook_time_minutes }}m</li>
              <li><strong>{{ t("recipe.meta.servings") }}:</strong> {{ recipe.servings }}</li>
            </ul>

            <p class="text-gray-700 dark:text-gray-300 leading-relaxed">{{ recipe.description }}</p>
          </div>

          <div class="flex flex-wrap gap-3 pt-2">
            <slot name="header-actions" />
          </div>
        </div>
      </CardContent>
    </Card>

    <Card class="shadow-md">
      <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-4">
        <CardTitle class="text-2xl font-bold">{{ t("recipe.ingredients.title") }}</CardTitle>
        <slot name="ingredient-toolbar" />
      </CardHeader>
      <CardContent class="space-y-8">
        <div v-for="group in recipe.ingredient_groups" :key="group.position">
          <h3 v-if="group.title" class="font-bold text-lg mb-4 text-primary/80 border-b pb-1">
            {{ group.title }}
          </h3>

          <ul class="grid grid-cols-1 gap-y-4">
            <li v-for="ing in group.ingredients" :key="ing.id" class="flex items-center justify-between group border-b border-gray-100 dark:border-gray-800 pb-2 last:border-0">
              <div class="flex items-center gap-4">
                <Checkbox :id="ing.id" class="accent-primary h-5 w-5" />

                <label :for="ing.id" class="text-base leading-snug cursor-pointer">
                  <span class="inline-block min-w-20 font-bold text-primary mr-2">
                    {{ getScaledQuantity(ing) }} {{ getBaseUnitSymbol(ing.unit) }}
                  </span>

                  <span class="text-gray-900 dark:text-gray-100">{{ ing.name }}</span>

                  <span v-if="ing.note" class="text-destructive font-bold ml-1" :title="ing.note">*</span>

                  <span v-if="isAdmin" class="text-[10px] text-gray-400 ml-2 italic underline">({{ ing.id }})</span>
                </label>
              </div>
            </li>
          </ul>
        </div>
      </CardContent>
    </Card>

    <Card class="shadow-md">
      <CardHeader class="flex flex-row items-center justify-between">
        <CardTitle class="text-2xl font-bold">{{ t("recipe.steps.title") }}</CardTitle>
        <div class="flex items-center gap-2">
          <Label class="text-xs hidden sm:block">{{ t("recipe.steps.showImages") }}</Label>
          <Switch v-model="showStepImages" />
        </div>
      </CardHeader>
      <CardContent class="space-y-12">
        <div v-for="group in recipe.step_groups" :key="group.position">
          <h3 v-if="group.title" class="font-bold text-lg mb-6 text-primary/80 border-b pb-1">{{ group.title }}</h3>
          <div class="space-y-8">
            <div v-for="(step, index) in group.steps" :key="step.position" class="relative pl-8">
              <div class="absolute left-0 top-0 w-6 h-6 rounded-full bg-primary/10 text-primary flex items-center justify-center text-xs font-bold">
                {{ index + 1 }}
              </div>
              <p class="text-gray-700 dark:text-gray-300 mb-4">{{ step.instruction }}</p>
              <div v-if="step.image_url && showStepImages" class="rounded-xl overflow-hidden border shadow-sm max-w-2xl">
                <img :src="$apiUrl + step.image_url" class="w-full h-auto object-cover" />
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>