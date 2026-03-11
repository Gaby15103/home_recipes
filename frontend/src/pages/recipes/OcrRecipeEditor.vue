<script setup lang="ts">
import { ref, onMounted } from "vue"
import { useI18n } from "vue-i18n"
import {
  Wand2, Save, Trash2, Edit3, Eye,
  Clock, Users, CheckCircle2, ChevronRight
} from "lucide-vue-next"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Separator } from '@/components/ui/separator'
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

// --- STATE ---
const storedOcr = ref<OcrRecipeResponse | null>(null)
const available_language = ref<Language[]>([])
const currentLang = ref("fr")
const editMode = ref<'preview' | 'edit'>('preview') // Toggle for the whole page

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

// --- HYDRATION ---
const hydrateRecipeFromOcr = () => {
  if (!storedOcr.value) return
  const ocr = storedOcr.value
  const lang = ocr.primary_language || "fr"

  recipe.value = {
    ...recipe.value,
    primary_language: lang,
    servings: ocr.detected_servings || 1,
    translations: [{ language_code: lang, title: ocr.title || "", description: "" }],
    ingredient_groups: (ocr.ingredient_groups || []).map(group => ({
      translations: [{ language_code: lang, name: group.name }],
      ingredients: (group.ingredients || []).map(ing => ({
        quantity: ing.quantity,
        unit_id: ing.unit?.lexicon_id || null,
        ingredient_id: ing.ingredient?.lexicon_id || null,
        note: [{ language_code: lang, text: ing.ingredient ? "" : `Original: ${ing.original_line}` }],
        translations: [{ language_code: lang, name: ing.ingredient?.term_fr || ing.original_line || "" }]
      }))
    })),
    step_groups: (ocr.step_groups || []).map(group => ({
      translations: [{ language_code: lang, name: group.name }],
      steps: (group.steps || []).map((step, index) => ({
        position: index,
        translations: [{ language_code: lang, text: step.raw_text }]
      }))
    }))
  }
}

const submit = async () => {
  try {
    const res = await createRecipe(recipe.value)
    localStorage.removeItem('pending-ocr-data')
    await router.push(ROUTES.ADMIN.RECIPE.VIEW(res.id))
  } catch (e) { console.error(e) }
}

onMounted(async () => {
  const [langs] = await Promise.all([getAllLanguage()])
  available_language.value = langs
  const rawData = localStorage.getItem('pending-ocr-data')
  if (rawData) {
    storedOcr.value = JSON.parse(rawData)
    currentLang.value = storedOcr.value?.primary_language || "fr"
    hydrateRecipeFromOcr()
  } else {
    router.push(ROUTES.ADMIN.RECIPE.CREATE)
  }
})

const getTranslation = (code: string) => {
  return recipe.value.translations.find(t => t.language_code === code) || { title: '', description: '' }
}
</script>

<template>
  <div v-if="storedOcr" class="min-h-screen bg-background pb-20">
    <nav class="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div class="max-w-4xl mx-auto px-4 h-16 flex items-center justify-between">
        <div class="flex items-center gap-3">
          <Badge variant="outline" class="hidden sm:flex gap-1 items-center border-primary/30 text-primary">
            <Wand2 class="w-3 h-3" /> OCR Review
          </Badge>
          <Tabs v-model="editMode" class="w-[200px]">
            <TabsList class="grid w-full grid-cols-2">
              <TabsTrigger value="preview"><Eye class="w-3 h-3 mr-2" /> View</TabsTrigger>
              <TabsTrigger value="edit"><Edit3 class="w-3 h-3 mr-2" /> Edit</TabsTrigger>
            </TabsList>
          </Tabs>
        </div>

        <div class="flex items-center gap-2">
          <Button variant="ghost" size="sm" @click="router.push(ROUTES.ADMIN.RECIPE.CREATE)" class="text-muted-foreground">
            Cancel
          </Button>
          <Button size="sm" @click="submit" class="font-bold">
            Approve & Save
          </Button>
        </div>
      </div>
    </nav>

    <main class="max-w-3xl mx-auto px-4 py-8 space-y-12">

      <section class="text-center space-y-4">
        <div v-if="editMode === 'edit'" class="max-w-xl mx-auto">
          <Label class="text-[10px] uppercase font-bold text-muted-foreground">Title</Label>
          <Input v-model="getTranslation(currentLang).title" class="text-center text-2xl font-bold h-14" />
        </div>
        <h1 v-else class="text-4xl font-black tracking-tight text-foreground">
          {{ getTranslation(currentLang).title || 'Untitled Recipe' }}
        </h1>

        <div class="flex items-center justify-center gap-6 text-muted-foreground">
          <div class="flex items-center gap-1.5">
            <Users class="w-4 h-4" />
            <span v-if="editMode === 'preview'" class="text-sm font-medium">{{ recipe.servings }} servings</span>
            <Input v-else type="number" v-model="recipe.servings" class="w-16 h-8 text-xs" />
          </div>
          <div class="flex items-center gap-1.5">
            <Clock class="w-4 h-4" />
            <span class="text-sm font-medium">{{ recipe.prep_time_minutes + recipe.cook_time_minutes }} mins</span>
          </div>
        </div>
      </section>

      <Card :class="editMode === 'edit' ? 'border-primary shadow-md' : 'border-none shadow-none bg-transparent'">
        <CardHeader class="px-0 flex flex-row items-center justify-between">
          <CardTitle class="text-xl font-bold flex items-center gap-2">
            Ingredients
          </CardTitle>
        </CardHeader>
        <CardContent class="px-0">
          <div v-if="editMode === 'preview'" class="grid grid-cols-1 md:grid-cols-2 gap-x-12 gap-y-3">
            <div v-for="group in recipe.ingredient_groups" :key="group.translations[0].name" class="col-span-full">
              <h4 v-if="group.translations[0].name" class="font-bold text-sm uppercase tracking-widest text-primary mb-4 mt-2">
                {{ group.translations[0].name }}
              </h4>
              <div v-for="(ing, idx) in group.ingredients" :key="idx" class="flex items-center justify-between py-2 border-b border-border/50">
                <div class="flex items-center gap-3">
                  <div class="h-1.5 w-1.5 rounded-full bg-primary/40" />
                  <span class="text-sm font-medium text-foreground">
                    {{ ing.translations[0].name }}
                  </span>
                </div>
                <Badge variant="secondary" class="font-mono text-[11px]">
                  {{ ing.quantity }} {{ ing.unit_id || '' }}
                </Badge>
              </div>
            </div>
          </div>
          <IngredientsEditor
              v-else
              v-model="recipe.ingredient_groups"
              :current-lang="currentLang"
              :available-languages="available_language"
          />
        </CardContent>
      </Card>

      <Separator />

      <Card :class="editMode === 'edit' ? 'border-primary shadow-md' : 'border-none shadow-none bg-transparent'">
        <CardHeader class="px-0">
          <CardTitle class="text-xl font-bold">Preparation</CardTitle>
        </CardHeader>
        <CardContent class="px-0">
          <div v-if="editMode === 'preview'" class="space-y-8">
            <div v-for="group in recipe.step_groups" :key="group.translations[0].name">
              <h4 v-if="group.translations[0].name" class="font-bold text-sm uppercase tracking-widest text-primary mb-6">
                {{ group.translations[0].name }}
              </h4>
              <div class="space-y-6">
                <div v-for="(step, sIdx) in group.steps" :key="sIdx" class="flex gap-4">
                  <span class="flex-shrink-0 flex h-8 w-8 items-center justify-center rounded-full bg-muted text-muted-foreground font-bold text-xs">
                    {{ sIdx + 1 }}
                  </span>
                  <p class="text-sm leading-relaxed text-foreground/80 pt-1.5">
                    {{ step.translations[0].text }}
                  </p>
                </div>
              </div>
            </div>
          </div>
          <StepsEditor
              v-else
              v-model="recipe.step_groups"
              :current-lang="currentLang"
              :available-languages="available_language"
          />
        </CardContent>
      </Card>

      <div v-if="editMode === 'edit'" class="fixed bottom-6 right-6 z-40 max-w-xs">
        <Card class="shadow-2xl border-primary/50 bg-card/90 backdrop-blur">
          <CardHeader class="py-2 px-4 border-b">
            <span class="text-[10px] font-black uppercase">OCR Reference</span>
          </CardHeader>
          <div class="max-h-[200px] overflow-y-auto p-3 text-[10px] text-muted-foreground italic">
            <div v-for="group in storedOcr.step_groups" :key="group.name">
              <div v-for="step in group.steps" :key="step.raw_text" class="mb-2 border-b pb-1 last:border-0">
                {{ step.raw_text }}
              </div>
            </div>
          </div>
        </Card>
      </div>
    </main>
  </div>
</template>