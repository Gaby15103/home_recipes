<script setup lang="ts">
import {computed, onMounted, ref} from "vue"
import {ChevronLeft, ScrollText} from "lucide-vue-next"
import {Button} from "@/components/ui/button"
import {Badge} from "@/components/ui/badge"

import IngredientsEditor from "@/components/recipe/editor/OcrIngredientsEditor.vue"
import StepsEditor from "@/components/recipe/editor/OcrStepsEditor.vue"
import RecipeDisplay from "@/components/recipe/RecipeDisplay.vue"

import type {RecipeCreate} from "@/models/RecipeCreate"
import type {OcrRecipeResponse} from "@/models/OcrResult"
import {confirmOcrRecipe} from "@/api/ocr"
import router from "@/router"
import {ROUTES} from "@/router/routes"
import type {Language} from "@/models/Language.ts"
import {getAllLanguage} from "@/api/Language.ts"

const storedOcr = ref<OcrRecipeResponse | null>(null)
const currentEditLang = ref("fr")
const available_languages = ref<Language[]>([])
const viewMode = ref<'editor' | 'preview' | 'split'>('split')

const recipe = ref<RecipeCreate>({
  author: null,
  author_id: null,
  image_url: null,
  translations: [
    {language_code: "fr", title: "", description: ""},
    {language_code: "en", title: "", description: ""}
  ],
  primary_language: "fr",
  servings: 1,
  prep_time_minutes: 0,
  cook_time_minutes: 0,
  ingredient_groups: [],
  step_groups: [],
  is_private: false,
  tags: []
})

const hydrateFromBackend = () => {
  if (!storedOcr.value) return
  const data = storedOcr.value
  const pLang = data.primary_language || "fr"

  recipe.value.primary_language = pLang
  recipe.value.servings = data.detected_servings || 1

  const titleTrans = recipe.value.translations.find(t => t.language_code === pLang)
  if (titleTrans) titleTrans.title = data.title

  recipe.value.ingredient_groups = data.ingredient_groups.map(group => ({
    translations: [
      {language_code: "fr", name: pLang === "fr" ? group.name : ""},
      {language_code: "en", name: pLang === "en" ? group.name : ""}
    ],
    ingredients: group.ingredients.map(ing => ({
      quantity: ing.quantity?.toString() || "",
      unit_id: ing.unit?.lexicon_id?.toString() || null,
      ingredient_id: ing.ingredient?.lexicon_id?.toString() || null,
      translations: [
        {language_code: "fr", name: pLang === "fr" ? (ing.ingredient?.term_fr || ing.original_line) : ""},
        {language_code: "en", name: pLang === "en" ? (ing.ingredient?.term_en || ing.original_line) : ""}
      ],
      note: [{language_code: "fr", text: ing.actions.join(", ")}, {language_code: "en", text: ing.actions.join(", ")}]
    }))
  }))

  recipe.value.step_groups = data.step_groups.map(group => ({
    translations: [
      {language_code: "fr", title: pLang === "fr" ? group.name : ""},
      {language_code: "en", title: pLang === "en" ? group.name : ""}
    ],
    steps: group.steps.map(step => ({
      position: step.position,
      duration_minutes: null,
      translations: [
        {language_code: pLang, text: step.raw_text},
        {language_code: pLang === "fr" ? "en" : "fr", text: ""}
      ]
    }))
  }))
}

const previewRecipe = computed(() => {
  return {
    ...recipe.value,
    title: recipe.value.translations.find(t => t.language_code === currentEditLang.value)?.title || "",
    ingredient_groups: recipe.value.ingredient_groups.map(g => ({
      title: g.translations.find(t => t.language_code === currentEditLang.value)?.name,
      ingredients: g.ingredients.map(i => ({
        ...i,
        display_name: i.translations.find(t => t.language_code === currentEditLang.value)?.name,
        unit: {symbol: i.unit_id}
      }))
    })),
    step_groups: recipe.value.step_groups.map(g => ({
      title: g.translations.find(t => t.language_code === currentEditLang.value)?.title,
      steps: g.steps.map(s => ({
        instruction: s.translations.find(t => t.language_code === currentEditLang.value)?.text
      }))
    }))
  } as any
})

const submit = async () => {
  const res = await confirmOcrRecipe({recipe: recipe.value})
  localStorage.removeItem('pending-ocr-data')
  router.push(ROUTES.ADMIN.RECIPE.VIEW(res.id))
}

onMounted(async () => {
  const raw = localStorage.getItem('pending-ocr-data')
  if (raw) {
    storedOcr.value = JSON.parse(raw)
    hydrateFromBackend()
  }

  const langs = await getAllLanguage()
  available_languages.value = langs
  currentEditLang.value = langs.find(l => l.is_default)?.code || "fr"
})
</script>
<template>
  <div v-if="storedOcr" class="min-h-screen bg-background text-foreground antialiased">
    <nav class="sticky top-0 z-50 border-b bg-background/95 backdrop-blur">
      <div class="max-w-[1400px] mx-auto h-14 px-6 flex items-center justify-between">
        <div class="flex items-center gap-4">
          <Button variant="outline" size="sm" @click="router.back()" class="h-8 px-3 rounded-md text-xs font-medium">
            <ChevronLeft class="w-3.5 h-3.5 mr-1"/>
            Back
          </Button>
          <div class="h-4 w-px bg-border"/>
          <div class="flex bg-muted p-1 rounded-md">
            <button v-for="mode in ['split', 'editor', 'preview']" :key="mode"
                    @click="viewMode = mode"
                    :class="[viewMode === mode ? 'bg-background shadow-sm' : 'text-muted-foreground hover:text-foreground']"
                    class="px-3 py-1 text-[10px] font-bold uppercase tracking-wider rounded-sm transition-all"
            >
              {{ mode }}
            </button>
          </div>
        </div>

        <div class="flex items-center gap-3">
          <div class="flex bg-muted p-1 rounded-md mr-2">
            <button v-for="l in available_languages" :key="l.code"
                    @click="currentEditLang = l.code"
                    :class="[currentEditLang === l.code ? 'bg-primary text-primary-foreground' : 'text-muted-foreground']"
                    class="w-8 h-6 text-[10px] font-bold uppercase rounded-sm transition-all"
            >
              {{ l.code }}
            </button>
          </div>
          <Button size="sm" @click="submit" class="h-8 font-bold text-xs uppercase tracking-widest px-6">
            Confirm & Save
          </Button>
        </div>
      </div>
    </nav>

    <main class="max-w-[1400px] mx-auto p-8 lg:p-12">
      <div v-if="viewMode === 'preview'" class="max-w-3xl mx-auto border rounded-2xl p-8 bg-card shadow-lg">
        <RecipeDisplay :recipe="previewRecipe" :multiplier="1"/>
      </div>

      <div v-else class="flex flex-col lg:flex-row gap-12 items-start">
        <div :class="[viewMode === 'split' ? 'lg:w-[65%]' : 'w-full']" class="space-y-12">
          <div class="space-y-2">
            <label class="text-[10px] font-bold uppercase tracking-[0.2em] text-muted-foreground">Recipe Title</label>
            <input
                v-model="recipe.translations.find(t => t.language_code === currentEditLang)!.title"
                class="w-full text-5xl font-extrabold tracking-tight bg-transparent border-none outline-none focus:ring-0 p-0"
                placeholder="The Title"
            />
          </div>

          <IngredientsEditor
              v-model="recipe.ingredient_groups"
              :current-lang="currentEditLang"
              :original-ocr-groups="storedOcr.ingredient_groups"
          />

          <StepsEditor
              v-model="recipe.step_groups"
              :current-lang="currentEditLang"
          />
        </div>

        <aside v-if="viewMode === 'split'" class="lg:w-[35%] sticky top-24 w-full">
          <div class="rounded-xl border bg-card shadow-sm flex flex-col max-h-[calc(100vh-8rem)]">
            <div class="p-4 border-b bg-muted/30 flex items-center justify-between">
              <div class="flex items-center gap-2">
                <ScrollText class="w-4 h-4 text-muted-foreground"/>
                <span class="font-bold text-xs uppercase tracking-widest">Full OCR Trace</span>
              </div>
              <Badge variant="outline" class="text-[9px] uppercase font-mono tracking-tighter">Raw Data</Badge>
            </div>

            <div class="overflow-y-auto p-6 space-y-4">
              <div
                  class="p-4 rounded-lg bg-muted/20 border border-dashed text-[11px] font-mono leading-relaxed text-muted-foreground whitespace-pre-wrap">
                {{ storedOcr.raw_text }}
              </div>

              <div class="space-y-2 pt-4">
                <h4 class="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">Unparsed Segments</h4>
                <div v-for="seg in storedOcr.unparsed_segments" :key="seg"
                     class="p-2 bg-destructive/5 text-destructive border-destructive/20 border rounded-md text-[10px]">
                  {{ seg }}
                </div>
              </div>
            </div>
          </div>
        </aside>
      </div>
    </main>
  </div>
</template>