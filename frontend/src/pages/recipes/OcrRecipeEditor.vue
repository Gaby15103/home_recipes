<script setup lang="ts">
import { onMounted, ref } from "vue"
import { ChevronLeft, Save, ScrollText } from "lucide-vue-next"
import { Button } from "@/components/ui/button"
import { Separator } from "@/components/ui/separator"

import IngredientsEditor from "@/components/recipe/editor/OcrIngredientsEditor.vue"
import StepsEditor from "@/components/recipe/editor/OcrStepsEditor.vue"

import type { RecipeCreate } from "@/models/RecipeCreate"
import type { OcrRecipeResponse } from "@/models/OcrResult"
import { confirmOcrRecipe } from "@/api/ocr"
import router from "@/router"
import { ROUTES } from "@/router/routes"
import type { Language } from "@/models/Language.ts"
import { getAllLanguage } from "@/api/Language.ts"

const storedOcr = ref<OcrRecipeResponse | null>(null)
const currentEditLang = ref("fr")
const available_languages = ref<Language[]>([])
const viewMode = ref<'preview' | 'split'>('split')

const recipe = ref<RecipeCreate>({
  translations: [
    { language_code: "fr", title: "", description: "" },
    { language_code: "en", title: "", description: "" }
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
      { language_code: "fr", name: pLang === "fr" ? group.name : "" },
      { language_code: "en", name: pLang === "en" ? group.name : "" }
    ],
    ingredients: group.ingredients.map(ing => ({
      quantity: ing.quantity,
      unit_id: ing.unit?.lexicon_id || null,
      ingredient_id: ing.ingredient?.lexicon_id || null,
      translations: [
        { language_code: "fr", name: pLang === "fr" ? ing.display_name : "" },
        { language_code: "en", name: pLang === "en" ? ing.display_name : "" }
      ],
      note: [{ language_code: "fr", text: "" }, { language_code: "en", text: "" }]
    }))
  }))

  recipe.value.step_groups = data.step_groups.map(group => ({
    translations: [
      { language_code: "fr", title: pLang === "fr" ? group.name : "" },
      { language_code: "en", title: pLang === "en" ? group.name : "" }
    ],
    steps: group.steps.map(step => ({
      position: step.position,
      duration_minutes: null,
      translations: [
        { language_code: pLang, text: step.raw_text },
        { language_code: pLang === "fr" ? "en" : "fr", text: "" }
      ]
    }))
  }))
}

const submit = async () => {
  const res = await confirmOcrRecipe({ recipe: recipe.value })
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
  const defaultLang = langs.find(l => l.is_default)?.code || langs[0]?.code
  currentEditLang.value = defaultLang
})
</script>

<template>
  <div v-if="storedOcr" class="min-h-screen bg-slate-50 dark:bg-[#0B0E14] text-slate-900 dark:text-slate-100">
    <nav class="sticky top-0 z-50 border-b border-slate-200 dark:border-slate-800 bg-white/80 dark:bg-slate-950/80 backdrop-blur-xl">
      <div class="max-w-[1600px] mx-auto h-16 px-6 flex items-center justify-between">
        <div class="flex items-center gap-6">
          <Button variant="ghost" size="icon" @click="router.back()" class="rounded-full bg-slate-100 dark:bg-slate-800">
            <ChevronLeft class="w-4 h-4" />
          </Button>
          <div class="flex p-1 bg-slate-100 dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800">
            <button @click="viewMode = 'preview'" :class="[viewMode === 'preview' ? 'bg-white dark:bg-slate-800 shadow-sm' : 'text-slate-500']" class="px-4 py-1.5 text-[11px] font-black uppercase rounded-lg transition-all">Preview</button>
            <button @click="viewMode = 'split'" :class="[viewMode === 'split' ? 'bg-white dark:bg-slate-800 shadow-sm' : 'text-slate-500']" class="px-4 py-1.5 text-[11px] font-black uppercase rounded-lg transition-all">Split View</button>
          </div>
        </div>

        <div class="flex items-center gap-4">
          <div class="flex items-center bg-slate-100 dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 p-1">
            <button
                v-for="l in available_languages" :key="l.code"
                @click="currentEditLang = l.code"
                :class="[currentEditLang === l.code ? 'bg-primary text-white shadow-md' : 'text-slate-500']"
                class="px-4 py-1.5 text-[11px] font-black uppercase rounded-lg transition-all"
            >
              {{ l.code }}
            </button>
          </div>
          <Button @click="submit" class="bg-primary hover:bg-primary/90 text-white font-black px-6 shadow-lg shadow-primary/20 rounded-xl">
            <Save class="w-4 h-4 mr-2" /> Confirm
          </Button>
        </div>
      </div>
    </nav>

    <main :class="['mx-auto py-12 px-8 flex gap-12 transition-all', viewMode === 'split' ? 'max-w-[1600px]' : 'max-w-4xl']">
      <div :class="[viewMode === 'split' ? 'w-[65%]' : 'w-full']" class="space-y-16">
        <header class="space-y-6">
          <input
              v-model="recipe.translations.find(t => t.language_code === currentEditLang)!.title"
              class="w-full text-6xl font-black tracking-tight bg-transparent border-none outline-none focus:ring-0 p-0 text-slate-900 dark:text-white"
              placeholder="Recipe Title..."
          />
          <div class="flex items-center gap-12 py-8 border-y border-slate-200 dark:border-slate-800">
            <div class="space-y-1">
              <span class="text-[10px] font-black uppercase text-slate-400 tracking-widest">Servings</span>
              <input type="number" v-model="recipe.servings" class="block w-20 text-2xl font-black bg-transparent border-none focus:ring-0 p-0 dark:text-white" />
            </div>
          </div>
        </header>

        <section class="space-y-12">
          <IngredientsEditor v-model="recipe.ingredient_groups" :current-lang="currentEditLang" :available-languages="available_languages" />
          <Separator class="bg-slate-200 dark:bg-slate-800" />
          <StepsEditor v-model="recipe.step_groups" :current-lang="currentEditLang" :available-languages="available_languages" />
        </section>
      </div>

      <aside v-if="viewMode === 'split'" class="w-[35%] sticky top-28 h-[calc(100vh-10rem)]">
        <div class="h-full bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-[2rem] shadow-xl flex flex-col overflow-hidden">
          <div class="p-6 border-b bg-slate-50 dark:bg-slate-800 flex items-center gap-2 text-[10px] font-black uppercase tracking-widest text-slate-500">
            <ScrollText class="w-4 h-4 text-primary" /> Scan Reference
          </div>
          <div class="flex-1 overflow-y-auto p-8 space-y-6 font-mono text-[11px] text-slate-500">
            <div v-for="group in storedOcr.ingredient_groups" :key="group.name" class="p-4 rounded-2xl bg-slate-50 dark:bg-slate-950/50 border border-slate-100 dark:border-slate-800">
              <div class="text-primary font-bold uppercase mb-2">{{ group.name }}</div>
              <div v-for="ing in group.ingredients" :key="ing.original_line" class="opacity-70">• {{ ing.original_line }}</div>
            </div>
            <div class="pt-6 opacity-40">
              <p class="whitespace-pre-wrap">{{ storedOcr.raw_text }}</p>
            </div>
          </div>
        </div>
      </aside>
    </main>
  </div>
</template>