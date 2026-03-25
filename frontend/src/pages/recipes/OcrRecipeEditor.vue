<script setup lang="ts">
import {computed, onMounted, ref} from "vue"
import {ChevronLeft, Image as ImageIcon, Timer, Users} from "lucide-vue-next"
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
  image_url: "",
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
  recipe.value.translations = [
    { language_code: "fr", title: data.title_fr, description: "" },
    { language_code: "en", title: data.title_en, description: "" }
  ];

  recipe.value.ingredient_groups = data.ingredient_groups.map((group, gIdx) => ({
    position: gIdx,
    translations: [
      { language_code: "fr", title: group.name_fr },
      { language_code: "en", title: group.name_en }
    ],
    ingredients: group.ingredients.map(ing => ({
      quantity: ing.quantity || 0,
      unit_id: units.value.find(u => u.name_en.toLowerCase() == ing.unit?.term_en.toLowerCase())?.id || "",
      position: ing.position,
      translations: [
        { language_code: "fr", data: ing.display_name_fr, note: ing.actions.map(a => a.term_fr).join(", ") },
        { language_code: "en", data: ing.display_name_en, note: ing.actions.map(a => a.term_en).join(", ") }
      ]
    }))
  }));

  recipe.value.step_groups = data.step_groups.map((group, gIdx) => ({
    position: gIdx,
    translations: [
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
    title: recipe.value.translations.find(t => t.language_code === lang)?.title || "",
    ingredient_groups: recipe.value.ingredient_groups.map(g => ({
      title: g.translations.find(t => t.language_code === lang)?.title || "",
      ingredients: g.ingredients.map(i => {
        const trans = i.translations.find(t => t.language_code === lang);
        return { ...i, display_name: trans?.data || "", note: trans?.note || "", unit: { symbol: i.unit_id } };
      })
    })),
    step_groups: recipe.value.step_groups.map(g => ({
      title: (g.translations as any).find?.((t: any) => t.language_code === lang)?.title || "",
      steps: g.steps.map(s => ({
        ...s,
        instruction: s.translations.find(t => t.language_code === lang)?.instruction || ""
      }))
    }))
  } as any;
});

const submit = async () => {
  const lang = recipe.value.primary_language;
  const mainTitle = recipe.value.translations.find(t => t.language_code === lang)?.title || "Recipe";

  // Mapping vers OcrCorrectionWrapper attendu par Rust
  const payload = {
    modified_recipe: {
      title: mainTitle,
      primary_language: recipe.value.primary_language,
      image_url: recipe.value.image_url || "",
      author_id: null,
      author: null,
      servings: recipe.value.servings || 1,
      prep_time_minutes: recipe.value.prep_time_minutes || 0,
      cook_time_minutes: recipe.value.cook_time_minutes || 0,
      is_private: recipe.value.is_private,
      tags: [],
      ingredient_groups: recipe.value.ingredient_groups.map(ig => ({
        translations: ig.translations,
        ingredients: ig.ingredients.map(i => ({
          position: i.position,
          quantity: i.quantity,
          source_ocr_lines: [i.translations.find(t => t.language_code === lang)?.data || ""],
          confirmed_lexicon_id: null,
          main_db_ingredient_id: "00000000-0000-0000-0000-000000000000", // Placeholder UUID
          unit_id: i.unit_id || null
        }))
      })),
      step_groups: recipe.value.step_groups.map(sg => ({
        translations: sg.translations,
        steps: sg.steps.map(s => ({
          position: s.position,
          text: s.translations.find(t => t.language_code === lang)?.instruction || "",
          source_ocr_segments: []
        }))
      }))
    },
    lexicon_feedback: []
  };

  try {
    const res = await confirmOcrRecipe(payload);
    localStorage.removeItem('pending-ocr-data');
    router.push(ROUTES.ADMIN.RECIPE.VIEW(res.id));
  } catch (e) {
    console.error("Submission failed", e);
  }
}

onMounted(async () => {
  const raw = localStorage.getItem('pending-ocr-data')
  if (raw) storedOcr.value = JSON.parse(raw)

  const [langs, fetchedUnits] = await Promise.all([getAllLanguage(), getUnits()]);
  available_languages.value = langs;
  units.value = fetchedUnits;
  currentEditLang.value = langs.find(l => l.is_default)?.code || "fr";

  if (storedOcr.value) hydrateFromBackend();
});
</script>

<template>
  <div v-if="storedOcr" class="min-h-screen bg-background text-foreground antialiased font-sans">
    <nav class="sticky top-0 z-50 border-b bg-background/95 backdrop-blur">
      <div class="max-w-[1400px] mx-auto h-14 px-6 flex items-center justify-between">
        <div class="flex items-center gap-4">
          <Button variant="outline" size="sm" @click="router.back()" class="h-8 px-3 text-xs">
            <ChevronLeft class="w-3.5 h-3.5 mr-1"/> Back
          </Button>
          <div class="flex bg-muted p-1 rounded-md">
            <button v-for="mode in ['split', 'editor', 'preview']" :key="mode" @click="viewMode = mode"
                    :class="[viewMode === mode ? 'bg-background shadow-sm' : 'text-muted-foreground']"
                    class="px-3 py-1 text-[10px] font-bold uppercase rounded-sm transition-all">
              {{ mode }}
            </button>
          </div>
        </div>
        <div class="flex items-center gap-3">
          <div class="flex bg-muted p-1 rounded-md mr-2">
            <button v-for="l in available_languages" :key="l.code" @click="currentEditLang = l.code"
                    :class="[currentEditLang === l.code ? 'bg-primary text-primary-foreground' : 'text-muted-foreground']"
                    class="w-8 h-6 text-[10px] font-bold uppercase rounded-sm">
              {{ l.code }}
            </button>
          </div>
          <Button size="sm" @click="submit" class="h-8 font-bold text-xs uppercase px-6">Confirm & Save</Button>
        </div>
      </div>
    </nav>

    <main class="max-w-[1400px] mx-auto p-8">
      <div v-if="viewMode === 'preview'" class="max-w-3xl mx-auto border rounded-2xl p-8 bg-card shadow-lg">
        <RecipeDisplay :recipe="previewRecipe" :multiplier="1"/>
      </div>

      <div v-else class="flex flex-col lg:flex-row gap-12 items-start">
        <div :class="[viewMode === 'split' ? 'lg:w-[65%]' : 'w-full']" class="space-y-10">

          <div class="space-y-6">
            <div class="space-y-2">
              <label class="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">Recipe Title</label>
              <input v-model="recipe.translations.find(t => t.language_code === currentEditLang)!.title"
                     class="w-full text-5xl font-black bg-transparent border-none outline-none p-0 focus:ring-0"
                     placeholder="Enter title..."/>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-4 gap-4 p-5 bg-muted/30 rounded-xl border">
              <div class="space-y-2 col-span-1 md:col-span-2">
                <label class="text-[9px] font-bold uppercase flex items-center gap-1.5 text-muted-foreground">
                  <ImageIcon class="w-3 h-3"/> Image URL
                </label>
                <input v-model="recipe.image_url" class="w-full h-9 bg-background border rounded-md px-3 text-sm outline-none focus:ring-1 focus:ring-primary"/>
              </div>
              <div class="space-y-2">
                <label class="text-[9px] font-bold uppercase flex items-center gap-1.5 text-muted-foreground">
                  <Users class="w-3 h-3"/> Servings
                </label>
                <input type="number" v-model.number="recipe.servings" class="w-full h-9 bg-background border rounded-md px-3 text-sm outline-none"/>
              </div>
              <div class="space-y-2">
                <label class="text-[9px] font-bold uppercase flex items-center gap-1.5 text-muted-foreground">
                  <Timer class="w-3 h-3"/> Prep/Cook (min)
                </label>
                <div class="flex gap-2">
                  <input type="number" v-model.number="recipe.prep_time_minutes" placeholder="Prep" class="w-full h-9 bg-background border rounded-md px-2 text-sm outline-none"/>
                  <input type="number" v-model.number="recipe.cook_time_minutes" placeholder="Cook" class="w-full h-9 bg-background border rounded-md px-2 text-sm outline-none"/>
                </div>
              </div>
            </div>
          </div>

          <IngredientsEditor v-model="recipe.ingredient_groups" :current-lang="currentEditLang" :original-ocr-groups="storedOcr.ingredient_groups" :units="units" />
          <StepsEditor v-model="recipe.step_groups" :current-lang="currentEditLang" :original-ocr-groups="storedOcr.step_groups" />
        </div>

        <aside v-if="viewMode === 'split'" class="lg:w-[35%] sticky top-24">
          <div class="rounded-xl border bg-card shadow-sm flex flex-col max-h-[calc(100vh-8rem)]">
            <div class="p-4 border-b bg-muted/50 flex items-center justify-between">
              <span class="font-bold text-[10px] uppercase tracking-widest">OCR Source Trace</span>
              <Badge variant="outline" class="text-[9px] font-mono">Raw</Badge>
            </div>
            <div class="overflow-y-auto p-5 text-[11px] font-mono leading-relaxed text-muted-foreground whitespace-pre-wrap">
              {{ storedOcr.raw_text }}
            </div>
          </div>
        </aside>
      </div>
    </main>
  </div>
</template>