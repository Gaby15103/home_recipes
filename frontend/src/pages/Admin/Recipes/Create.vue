<script setup lang="ts">
import {ref} from "vue"
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Textarea} from "@/components/ui/textarea"
import {Switch} from "@/components/ui/switch"
import {Label} from "@/components/ui/label"
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card"
import {Separator} from '@/components/ui/separator'
import IngredientsEditor from "@/pages/Admin/Recipes/IngredientsEditor.vue";
import StepsEditor from "@/pages/Admin/Recipes/StepsEditor.vue";
import type {RecipeCreate, StepImage} from "@/models/RecipeCreate.ts";
import {createRecipe} from "@/api/recipe.ts"
import {router} from "@/router";
import JsonImporter from "@/components/json/JsonImporter.vue";

const recipe = ref<RecipeCreate>({
  title: "",
  description: null,
  servings: 1,
  prep_time_minutes: 0,
  cook_time_minutes: 0,
  author: null,
  author_id: null,
  is_private: false,
  tags: [],
  ingredient_groups: [],
  step_groups: [],
})
const stepImages = ref<StepImage[]>([])

const mainImageFile = ref<File | null>(null)
const mainImagePreview = ref<string | null>(null)

function onMainImageChange(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0] ?? null

  if (mainImagePreview.value) {
    URL.revokeObjectURL(mainImagePreview.value)
  }

  mainImageFile.value = file
  mainImagePreview.value = file ? URL.createObjectURL(file) : null
}


const submitting = ref(false)

async function submit() {
  console.log("Recipe payload:", recipe.value)
  try {
    const res = await createRecipe(recipe.value, mainImageFile.value, stepImages.value)
    await router.push("/recipes/" + res.id)
  } catch (e: any) {
    console.error(e)
  }
}

</script>

<template>
  <div class="max-w-4xl mx-auto p-6 space-y-6">
    <h1 class="text-3xl font-bold">Create Recipe</h1>
    <JsonImporter v-model="recipe" />
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
              v-if="mainImageFile"
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
        Create recipe
      </Button>
    </div>
  </div>
</template>

