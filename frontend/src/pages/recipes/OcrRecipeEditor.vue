<script setup lang="ts">
import { ref, onMounted } from "vue"
import { useI18n } from "vue-i18n"
import {
  Wand2, Save, Trash2, ArrowRight, AlertCircle
} from "lucide-vue-next"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Separator } from '@/components/ui/separator'
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Label } from "@/components/ui/label"

import IngredientsEditor from "@/components/recipe/editor/IngredientsEditor.vue"
import StepsEditor from "@/components/recipe/editor/StepsEditor.vue"
import type { RecipeCreate } from "@/models/RecipeCreate.ts"
import type { OcrRecipeResponse } from "@/models/OcrResult.ts"
import type { Language } from "@/models/Language.ts"
import { createRecipe } from "@/api/recipe.ts"
import { getAllLanguage } from "@/api/Language.ts"
import router from "@/router"
import { ROUTES } from "@/router/routes.ts"

const { t } = useI18n()

const storedOcr = ref<OcrRecipeResponse | null>(null)
const available_language = ref<Language[]>([])
const currentLang = ref("fr")
const isReady = ref(false)

const recipe = ref<RecipeCreate>({
  translations: [],
  image_url: null,
  primary_language: "fr",
  servings: 1,
  prep_time_minutes: 0,
  cook_time_minutes: 0,
  author: null,
  author_id: null,
  is_private: false,
  tags: [],
  ingredient_groups: [],
  step_groups: []
})

const hydrateRecipeFromOcr = () => {
  if (!storedOcr.value) return
  const ocr = storedOcr.value

  recipe.value = {
    ...recipe.value,
    primary_language: ocr.primary_language || "fr",
    servings: ocr.detected_servings || 1,
    translations: [{
      language_code: ocr.primary_language || "fr",
      title: ocr.title || "",
      description: ""
    }],
    ingredient_groups: (ocr.ingredient_groups || []).map(group => ({
      translations: [{ language_code: ocr.primary_language || "fr", name: group.name }],
      ingredients: (group.ingredients || []).map(ing => ({
        quantity: ing.quantity,
        unit_id: ing.unit?.lexicon_id || null,
        ingredient_id: ing.ingredient?.lexicon_id || null,
        note: [{
          language_code: ocr.primary_language || "fr",
          text: ing.ingredient ? "" : `Original: ${ing.original_line}`
        }],
        translations: [{
          language_code: ocr.primary_language || "fr",
          name: ing.ingredient?.term_fr || ing.original_line || ""
        }]
      }))
    })),
    step_groups: (ocr.step_groups || []).map(group => ({
      translations: [{ language_code: ocr.primary_language || "fr", name: group.name }],
      steps: (group.steps || []).map((step, index) => ({
        position: index,
        translations: [{
          language_code: ocr.primary_language || "fr",
          text: step.raw_text
        }]
      }))
    }))
  }
}

const clearOcr = () => {
  if (confirm("Discard scan results?")) {
    localStorage.removeItem('pending-ocr-data')
    router.push(ROUTES.ADMIN.RECIPE.CREATE)
  }
}

const submit = async () => {
  try {
    const res = await createRecipe(recipe.value)
    localStorage.removeItem('pending-ocr-data')
    await router.push(ROUTES.ADMIN.RECIPE.VIEW(res.id))
  } catch (e) {
    console.error("Submit failed", e)
  }
}

onMounted(async () => {
  const [langs] = await Promise.all([getAllLanguage()])
  available_language.value = langs

  const rawData = localStorage.getItem('pending-ocr-data')
  if (rawData) {
    try {
      storedOcr.value = JSON.parse(rawData)
      currentLang.value = storedOcr.value?.primary_language || "fr"
      hydrateRecipeFromOcr()
      isReady.value = true
    } catch (err) {
      console.error("Corrupt OCR data", err)
      router.push(ROUTES.ADMIN.RECIPE.CREATE)
    }
  } else {
    router.push(ROUTES.ADMIN.RECIPE.CREATE)
  }
})

const getTranslation = (code: string) => {
  let trans = recipe.value.translations.find(t => t.language_code === code)
  if (!trans) {
    trans = { language_code: code, title: "", description: "" }
    recipe.value.translations.push(trans)
  }
  return trans
}
</script>

<template>
  <div v-if="storedOcr" class="max-w-[1600px] mx-auto p-6 bg-background">

    <div class="flex items-center justify-between mb-6 p-4 border rounded-xl bg-card shadow-sm">
      <div class="flex items-center gap-4">
        <h1 class="text-2xl font-bold flex items-center gap-2">
          <Wand2 class="w-6 h-6 text-primary" />
          {{ t('Admin.recipe.ocrReviewTitle') || 'Review Scan' }}
        </h1>
        <Badge variant="secondary" class="font-mono">{{ currentLang }}</Badge>
      </div>

      <div class="flex items-center gap-3">
        <Button variant="ghost" @click="clearOcr" class="text-destructive hover:bg-destructive/10">
          <Trash2 class="w-4 h-4 mr-2" /> Discard
        </Button>
        <Separator orientation="vertical" class="h-8" />
        <Button @click="submit" class="px-6 font-bold shadow-lg">
          <Save class="w-4 h-4 mr-2" /> Finish & Create
        </Button>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start">

      <aside class="lg:col-span-4 space-y-4 lg:sticky lg:top-6">
        <Card>
          <div class="px-4 py-2 border-b bg-muted/50 rounded-t-xl">
            <span class="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">Original Scanned Text</span>
          </div>
          <div class="max-h-[70vh] overflow-y-auto p-4 space-y-6">
            <Alert v-if="storedOcr.unparsed_segments?.length" variant="destructive" class="bg-destructive/5">
              <AlertCircle class="h-4 w-4" />
              <AlertDescription class="text-xs font-medium">Some text was not categorized.</AlertDescription>
            </Alert>

            <div v-for="(group, idx) in storedOcr.ingredient_groups" :key="idx" class="space-y-3">
              <h4 class="text-[10px] font-black text-primary uppercase px-2 py-1 bg-primary/10 rounded w-fit">{{ group.name }}</h4>
              <div v-for="(ing, iIdx) in group.ingredients" :key="iIdx" class="bg-muted/30 border rounded-lg p-3 text-sm">
                <p class="text-[10px] text-muted-foreground italic mb-2">"{{ ing.original_line }}"</p>
                <div class="flex items-center gap-2">
                  <Badge variant="outline" class="text-[10px]">{{ ing.quantity }} {{ ing.unit?.term_fr }}</Badge>
                  <ArrowRight class="w-3 h-3 text-muted-foreground" />
                  <span class="font-bold text-xs" :class="ing.ingredient ? 'text-foreground' : 'text-destructive'">
                    {{ ing.ingredient?.term_fr || 'Unmatched' }}
                  </span>
                </div>
              </div>
            </div>

            <div class="space-y-3">
              <h4 class="text-[10px] font-black text-primary uppercase px-2 py-1 bg-primary/10 rounded w-fit">Raw Steps</h4>
              <div v-for="sg in storedOcr.step_groups" :key="sg.name">
                <div v-for="(step, stIdx) in sg.steps" :key="stIdx" class="mb-2 p-3 bg-card border border-dashed rounded text-[11px] leading-relaxed">
                  {{ step.raw_text }}
                </div>
              </div>
            </div>
          </div>
        </Card>
      </aside>

      <main class="lg:col-span-8 space-y-6 pb-20">
        <Card>
          <CardContent class="p-6 space-y-8">
            <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
              <div class="md:col-span-3 space-y-2">
                <Label class="text-xs font-bold uppercase text-muted-foreground">Recipe Title</Label>
                <Input
                    v-model="getTranslation(recipe.primary_language).title"
                    class="text-xl font-bold h-12 bg-transparent"
                    placeholder="Enter recipe name..."
                />
              </div>
              <div class="space-y-2">
                <Label class="text-xs font-bold uppercase text-muted-foreground">Servings</Label>
                <Input type="number" v-model="recipe.servings" class="h-12 text-lg font-bold" />
              </div>
            </div>

            <Separator />

            <div class="space-y-8">
              <div class="space-y-4">
                <h3 class="text-lg font-bold flex items-center gap-3">
                  <span class="flex h-7 w-7 items-center justify-center rounded-full bg-primary text-xs text-primary-foreground">1</span>
                  Ingredients List
                </h3>
                <IngredientsEditor
                    v-model="recipe.ingredient_groups"
                    :current-lang="currentLang"
                    :available-languages="available_language"
                />
              </div>

              <Separator />

              <div class="space-y-4">
                <h3 class="text-lg font-bold flex items-center gap-3">
                  <span class="flex h-7 w-7 items-center justify-center rounded-full bg-primary text-xs text-primary-foreground">2</span>
                  Preparation Steps
                </h3>
                <StepsEditor
                    v-model="recipe.step_groups"
                    :current-lang="currentLang"
                    :available-languages="available_language"
                />
              </div>
            </div>
          </CardContent>
        </Card>
      </main>
    </div>
  </div>

  <div v-else class="flex flex-col items-center justify-center h-screen gap-4 bg-background text-muted-foreground">
    <div class="h-10 w-10 border-4 border-primary border-t-transparent rounded-full animate-spin"></div>
    <p class="font-medium animate-pulse text-lg">Loading review context...</p>
  </div>
</template>