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
import type {Unit} from "@/models/Recipe.ts";
import {getUnits} from "@/api/unit.ts";

const storedOcr = ref<OcrRecipeResponse | null>(null)
const currentEditLang = ref("fr")
const available_languages = ref<Language[]>([])
const viewMode = ref<'editor' | 'preview' | 'split'>('split')
const units = ref<Unit[]>([]);

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
  if (!storedOcr.value) return;
  const data = storedOcr.value;

  recipe.value.primary_language = data.primary_language || "fr";
  recipe.value.servings = data.detected_servings || 1;

  // 1. Recipe Title/Desc
  recipe.value.translations = [
    { language_code: "fr", title: data.title_fr, description: "" },
    { language_code: "en", title: data.title_en, description: "" }
  ];

  // 2. Ingredient Groups
  recipe.value.ingredient_groups = data.ingredient_groups.map((group, gIdx) => ({
    position: gIdx,
    translations: [
      { language_code: "fr", title: group.name_fr }, // Note: IngredientGroupTranslation uses 'title'
      { language_code: "en", title: group.name_en }
    ],
    ingredients: group.ingredients.map(ing => ({
      quantity: ing.quantity || 0,
      unit_id: units.value.find(u => u.name_en.toLowerCase() == ing.unit?.term_en.toLowerCase())?.id || "",
      position: ing.position,
      translations: [
        {
          language_code: "fr",
          data: ing.display_name_fr,
          note: ing.actions.map(a => a.term_fr).join(", ")
        },
        {
          language_code: "en",
          data: ing.display_name_en,
          note: ing.actions.map(a => a.term_en).join(", ")
        }
      ]
    }))
  }));

  // 3. Step Groups
  recipe.value.step_groups = data.step_groups.map((group, gIdx) => ({
    position: gIdx,
    translations: [ // Note: Your model has StepGroupTranslationCreate as a single object or array?
      // Based on your code 'translations: StepGroupTranslationCreate', it's an object.
      // But typically it's an array for bilingual. Assuming array based on IngredientGroups:
      { language_code: "fr", title: group.name_fr },
      { language_code: "en", title: group.name_en }
    ] as any,
    steps: group.steps.map(step => ({
      position: step.position,
      image_url: null,
      duration_minutes: null,
      translations: [
        { language_code: "fr", instruction: step.raw_text_fr },
        { language_code: "en", instruction: step.raw_text_en }
      ]
    }))
  }));
};

const previewRecipe = computed(() => {
  const lang = currentEditLang.value;

  return {
    ...recipe.value,
    // Recipe Title
    title: recipe.value.translations.find(t => t.language_code === lang)?.title || "",

    ingredient_groups: recipe.value.ingredient_groups.map(g => ({
      // Group Title
      title: g.translations.find(t => t.language_code === lang)?.title || "",
      ingredients: g.ingredients.map(i => {
        const trans = i.translations.find(t => t.language_code === lang);
        return {
          ...i,
          // Ingredient Name is '.data' in IngredientTranslationCreate
          display_name: trans?.data || "",
          note: trans?.note || "",
          unit: { symbol: i.unit_id }
        };
      })
    })),

    step_groups: recipe.value.step_groups.map(g => ({
      // Step Group Title
      title: (g.translations as any).find?.((t: any) => t.language_code === lang)?.title || "",
      steps: g.steps.map(s => ({
        ...s,
        // Step Instruction is '.instruction' in StepTranslationCreate
        instruction: s.translations.find(t => t.language_code === lang)?.instruction || ""
      }))
    }))
  } as any;
});

const submit = async () => {
  const res = await confirmOcrRecipe({recipe: recipe.value})
  localStorage.removeItem('pending-ocr-data')
  router.push(ROUTES.ADMIN.RECIPE.VIEW(res.id))
}

onMounted(async () => {
  // 1. Load the raw OCR data immediately
  const raw = localStorage.getItem('pending-ocr-data')
  if (raw) {
    storedOcr.value = JSON.parse(raw)
  }

  // 2. Fetch all necessary metadata in parallel
  const [langs, fetchedUnits] = await Promise.all([
    getAllLanguage(),
    getUnits()
  ]);

  // 3. Set the reactive refs
  available_languages.value = langs;
  units.value = fetchedUnits;

  // Set default language based on fetched data
  currentEditLang.value = langs.find(l => l.is_default)?.code || "fr";

  // 4. Finally, hydrate the recipe now that units.value is populated
  if (storedOcr.value) {
    hydrateFromBackend();
  }
});
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
              :units="units"
          />

          <StepsEditor
              v-model="recipe.step_groups"
              :current-lang="currentEditLang"
              :original-ocr-groups="storedOcr.step_groups"
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