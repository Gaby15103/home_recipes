<script setup lang="ts">
import { onMounted, ref, watch } from "vue"
import { useRoute, useRouter } from "vue-router"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Switch } from "@/components/ui/switch"
import { Label } from "@/components/ui/label"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Separator } from "@/components/ui/separator"

import IngredientsEditor from "@/pages/Admin/Recipes/IngredientsEditor.vue"
import StepsEditor from "@/pages/Admin/Recipes/StepsEditor.vue"
import TagsMultiSelect from "@/components/Recipe/TagsMultiSelect.vue"
import JsonImporter from "@/components/json/JsonImporter.vue"

import type { StepImage } from "@/models/RecipeCreate"
import type { RecipeEdit } from "@/models/RecipeEdit"

import { getRecipeById, updateRecipe } from "@/api/recipe"
import { recipeToEdit, editToUpdatePayload } from "@/mappers/recipe.mapper"
import {ROUTES} from "@/router/routes.ts";

const apiUrl = import.meta.env.VITE_STATIC_URL
const route = useRoute()
const router = useRouter()

const recipe = ref<RecipeEdit | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

/* ========= LOAD ========= */

onMounted(async () => {
  loading.value = true
  try {
    const apiRecipe = await getRecipeById(route.params.id as string)
    recipe.value = recipeToEdit(apiRecipe)
  } catch (err: any) {
    error.value = err.message ?? "Failed to fetch recipe"
  } finally {
    loading.value = false
  }
})

/* ========= IMAGES ========= */

const stepImages = ref<StepImage[]>([])
const mainImageFile = ref<File | null>(null)
const mainImagePreview = ref<string | null>(null)

watch(recipe, r => {
  if (!r) return
  if ((r as any).image_url) {
    mainImagePreview.value = apiUrl + (r as any).image_url
  }
})

function onMainImageChange(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0] ?? null

  if (mainImagePreview.value?.startsWith("blob:")) {
    URL.revokeObjectURL(mainImagePreview.value)
  }

  mainImageFile.value = file
  mainImagePreview.value = file ? URL.createObjectURL(file) : null
}

/* ========= SUBMIT ========= */

const submitting = ref(false)

async function submit() {
  if (!recipe.value) return

  submitting.value = true
  try {

    const updated = await updateRecipe(
        recipe.value.id,
        recipe.value,
        stepImages.value,
        mainImageFile.value
    )

    await router.push(ROUTES.ADMIN.RECIPE.VIEW(updated.id))
  } catch (e) {
    console.error(e)
  } finally {
    submitting.value = false
  }
}
</script>


<template>
  <div class="max-w-4xl mx-auto p-6 space-y-6">
    <h1 class="text-3xl font-bold">Edit Recipe</h1>
    <div v-if="recipe" class="max-w-4xl mx-auto p-6 space-y-6">
      <JsonImporter v-model="recipe"/>
      <!-- Basic info -->
      <Card>
        <CardHeader>
          <CardTitle>Basic information</CardTitle>
        </CardHeader>

        <CardContent class="space-y-4">
          <div class="space-y-2">
            <Label for="title">Title</Label>
            <Input id="title" v-model="recipe.title" placeholder="Chocolate cake"/>
          </div>

          <div class="space-y-2">
            <Label for="description">Description</Label>
            <Textarea
                id="description"
                v-model="recipe.description"
                placeholder="Short description (optional)"
            />
          </div>

          <div class="space-y-2">
            <Label>Recipe image</Label>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <Input type="file" accept="image/*" @change="onMainImageChange"/>
              <div class="ml-auto">
                <Label>Private recipe</Label>
                <Switch v-model:checked="recipe.is_private"/>
              </div>
            </div>

            <img
                v-if="mainImagePreview"
                :src="mainImagePreview"
                class="h-40 rounded border object-cover"
            />
          </div>
        </CardContent>
      </Card>

      <!-- Numbers -->
      <Card>
        <CardHeader>
          <CardTitle>Details</CardTitle>
        </CardHeader>

        <CardContent class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div class="space-y-2">
            <Label>Servings</Label>
            <Input type="number" min="1" v-model.number="recipe.servings"/>
          </div>

          <div class="space-y-2">
            <Label>Prep time (min)</Label>
            <Input type="number" min="0" v-model.number="recipe.prep_time_minutes"/>
          </div>

          <div class="space-y-2">
            <Label>Cook time (min)</Label>
            <Input type="number" min="0" v-model.number="recipe.cook_time_minutes"/>
          </div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          Tags
        </CardHeader>
        <CardContent class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <TagsMultiSelect v-model:model-value="recipe.tags"/>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Recipe</CardTitle>
        </CardHeader>

        <CardContent class="grid grid-cols-1 ">
          <div class="space-y-2">
            <IngredientsEditor v-model="recipe.ingredient_groups"/>
          </div>
          <separator orientation="horizontal" class="mb-8"/>
          <div class="space-y-2">
            <StepsEditor
                v-model:model-value="recipe.step_groups"
                v-model:images="stepImages"
            />
          </div>
        </CardContent>
      </Card>

      <!-- Actions -->
      <div class="flex justify-end gap-3">
        <Button variant="outline">Cancel</Button>
        <Button :disabled="submitting" @click="submit">
          Save changes
        </Button>
      </div>
    </div>
    <div v-else class="p-10 text-center">
      Loading recipe...
    </div>
  </div>
</template>

