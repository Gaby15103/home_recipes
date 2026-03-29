<script setup lang="ts">
import {computed, onMounted, ref} from "vue"
import {ChevronLeft, Image as ImageIcon, Timer, Users, Upload, ScrollText} from "lucide-vue-next"
import {Button} from "@/components/ui/button"
import {Badge} from "@/components/ui/badge"
import {Input} from "@/components/ui/input"

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

const NIL_UUID = "00000000-0000-0000-0000-000000000000";

// Gestion de l'image principale
const mainImagePreview = ref<string | null>(null)

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

function onMainImageChange(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0] ?? null
  if (mainImagePreview.value) {
    URL.revokeObjectURL(mainImagePreview.value)
  }
  recipe.value.image_url = file
  mainImagePreview.value = file ? URL.createObjectURL(file) : null
}

const hydrateFromBackend = () => {
  if (!storedOcr.value || units.value.length === 0) return;
  const data = storedOcr.value;

  // FIX: Ensure this is always a STRING id, not the whole object
  const defaultUnitId = units.value.find(u => u.code === "PIECE")?.id || units.value[0]?.id || NIL_UUID;

  // 1. Root Recipe Data
  recipe.value.primary_language = data.primary_language || "fr";
  recipe.value.servings = Number(data.detected_servings) || 1;
  recipe.value.translations = [
    { language_code: "fr", title: data.title_fr || "", description: "" },
    { language_code: "en", title: data.title_en || "", description: "" }
  ];

  // 2. Ingredient Groups
  recipe.value.ingredient_groups = data.ingredient_groups.map((group, gIdx) => ({
    position: gIdx,
    translations: [
      { language_code: "fr", title: group.name_fr || "Ingrédients" },
      { language_code: "en", title: group.name_en || "Ingredients" }
    ],
    ingredients: group.ingredients.map(ing => {
      const found = units.value.find(u =>
          u.name_en?.toLowerCase() === ing.unit?.term_en?.toLowerCase() ||
          u.name_fr?.toLowerCase() === ing.unit?.term_fr?.toLowerCase()
      );

      // Logic: If not found OR it's the NIL_UUID, use the default string ID
      const finalUnitId = (!found || found.id === NIL_UUID)
          ? defaultUnitId
          : found.id;

      return {
        quantity: Number(ing.quantity) || 0,
        unit_id: finalUnitId, // This is now guaranteed to be a string
        position: ing.position,
        translations: [
          {
            language_code: "fr",
            data: ing.display_name_fr || "",
            note: ing.actions?.length ? ing.actions.map(a => a.term_fr).join(", ") : ""
          },
          {
            language_code: "en",
            data: ing.display_name_en || "",
            note: ing.actions?.length ? ing.actions.map(a => a.term_en).join(", ") : ""
          }
        ]
      };
    })
  }));

  // 3. Step Groups
  recipe.value.step_groups = data.step_groups.map((group, gIdx) => ({
    position: gIdx,
    translations: [
      { language_code: "fr", title: group.name_fr || "Préparation" },
      { language_code: "en", title: group.name_en || "Preparation" }
    ],
    steps: group.steps.map(step => ({
      position: step.position,
      image_url: null,
      duration_minutes: null,
      translations: [
        { language_code: "fr", instruction: step.raw_text_fr || "" },
        { language_code: "en", instruction: step.raw_text_en || "" }
      ]
    }))
  }));
};

const previewRecipe = computed(() => {
  const lang = currentEditLang.value;

  return {
    ...recipe.value,
    // Root Title
    title: recipe.value.translations.find(t => t.language_code === lang)?.title || "",
    image_url: mainImagePreview.value || recipe.value.image_url,

    // Ingredient Groups
    ingredient_groups: recipe.value.ingredient_groups.map(g => ({
      title: g.translations.find(t => t.language_code === lang)?.title || "",
      ingredients: g.ingredients.map(i => {
        const trans = i.translations.find(t => t.language_code === lang);
        return {
          ...i,
          display_name: trans?.data || "",
          note: trans?.note || "",
          // Find the actual unit object to get the symbol (e.g., 'ml', 'g')
          unit: units.value.find(u => u.id === i.unit_id) || { symbol: "" }
        };
      })
    })),

    // Step Groups (FIXED: Now treats translations as an array)
    step_groups: recipe.value.step_groups.map(g => ({
      // Use .find() to get the title for the current UI language
      title: g.translations.find(t => t.language_code === lang)?.title || g.translations[0]?.title || "",
      steps: g.steps.map(s => ({
        ...s,
        // Find the instruction for the current UI language
        instruction: s.translations.find(t => t.language_code === lang)?.instruction || ""
      }))
    }))
  } as any;
});

const submit = async () => {
  try {
    // Just send the object as-is.
    // It's already reactive and up-to-date from your inputs.
    const res = await confirmOcrRecipe(recipe.value);

    localStorage.removeItem('pending-ocr-data');
    router.push(ROUTES.ADMIN.RECIPE.VIEW(res.id));
  } catch (e) {
    console.error("Submission failed. Check if unit_id is a valid UUID or NIL_UUID.", e);
  }
};

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
          <Button variant="outline" size="sm" @click="router.back()"
                  class="h-8 px-3 text-xs font-bold uppercase tracking-wider">
            <ChevronLeft class="w-3.5 h-3.5 mr-1"/>
            Back
          </Button>
          <div class="flex bg-muted p-1 rounded-md">
            <button v-for="mode in ['split', 'editor', 'preview']" :key="mode" @click="viewMode = mode"
                    :class="[viewMode === mode ? 'bg-background shadow-sm text-foreground' : 'text-muted-foreground hover:text-foreground']"
                    class="px-4 py-1 text-[10px] font-bold uppercase rounded-sm transition-all">
              {{ mode }}
            </button>
          </div>
        </div>
        <div class="flex items-center gap-3">
          <div class="flex bg-muted p-1 rounded-md mr-2">
            <button v-for="l in available_languages" :key="l.code" @click="currentEditLang = l.code"
                    :class="[currentEditLang === l.code ? 'bg-primary text-primary-foreground' : 'text-muted-foreground']"
                    class="w-9 h-6 text-[10px] font-bold uppercase rounded-sm">
              {{ l.code }}
            </button>
          </div>
          <Button size="sm" @click="submit"
                  class="h-8 font-bold text-xs uppercase px-8 tracking-widest bg-primary hover:bg-primary/90">
            Confirm & Save
          </Button>
        </div>
      </div>
    </nav>

    <main class="max-w-[1400px] mx-auto p-8 lg:p-12">
      <div v-if="viewMode === 'preview'" class="max-w-4xl mx-auto border rounded-3xl p-10 bg-card shadow-2xl">
        <RecipeDisplay :recipe="previewRecipe" :multiplier="1"/>
      </div>

      <div v-else class="flex flex-col lg:flex-row gap-16 items-start">
        <div :class="[viewMode === 'split' ? 'lg:w-[65%]' : 'w-full']" class="space-y-12">

          <div class="space-y-8">
            <div class="space-y-2">
              <label class="text-[10px] font-bold uppercase tracking-[0.2em] text-muted-foreground px-1">Recipe
                Title</label>
              <input v-model="recipe.translations.find(t => t.language_code === currentEditLang)!.title"
                     class="w-full text-6xl font-black bg-transparent border-none outline-none p-0 focus:ring-0 placeholder:text-muted/30"
                     placeholder="The Grand Recipe..."/>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-12 gap-6 p-8 bg-muted/20 rounded-3xl border border-border/50">

              <div class="md:col-span-4 space-y-3">
                <label
                    class="text-[10px] font-bold uppercase tracking-widest flex items-center gap-2 text-muted-foreground">
                  <ImageIcon class="w-3.5 h-3.5"/>
                  Main Image
                </label>
                <div
                    class="relative group aspect-video md:aspect-square bg-background border-2 border-dashed rounded-2xl flex flex-col items-center justify-center overflow-hidden transition-all hover:border-primary/50">
                  <img v-if="mainImagePreview" :src="mainImagePreview" class="object-cover w-full h-full"/>
                  <div v-else class="flex flex-col items-center gap-2 text-muted-foreground">
                    <Upload class="w-8 h-8 opacity-20"/>
                    <span class="text-[10px] font-medium uppercase tracking-tighter">Upload Photo</span>
                  </div>
                  <input type="file" accept="image/*" @change="onMainImageChange"
                         class="absolute inset-0 opacity-0 cursor-pointer z-10"/>
                </div>
              </div>

              <div class="md:col-span-8 grid grid-cols-2 gap-6">
                <div class="col-span-2 space-y-2">
                  <label class="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">Servings</label>
                  <div class="relative">
                    <Users class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground"/>
                    <Input type="number" v-model.number="recipe.servings"
                           class="pl-10 h-12 bg-background rounded-xl border-none shadow-sm"/>
                  </div>
                </div>
                <div class="space-y-2">
                  <label class="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">Prep
                    (min)</label>
                  <div class="relative">
                    <Timer class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground"/>
                    <Input type="number" v-model.number="recipe.prep_time_minutes"
                           class="pl-10 h-12 bg-background rounded-xl border-none shadow-sm"/>
                  </div>
                </div>
                <div class="space-y-2">
                  <label class="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">Cook
                    (min)</label>
                  <div class="relative">
                    <Timer class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground"/>
                    <Input type="number" v-model.number="recipe.cook_time_minutes"
                           class="pl-10 h-12 bg-background rounded-xl border-none shadow-sm"/>
                  </div>
                </div>
              </div>
            </div>
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

        <aside v-if="viewMode === 'split'" class="lg:w-[35%] sticky top-24 space-y-6">
          <div class="rounded-2xl border bg-card shadow-xl flex flex-col max-h-[calc(100vh-10rem)] overflow-hidden">
            <div class="p-5 border-b bg-muted/30 flex items-center justify-between">
              <div class="flex items-center gap-2">
                <ScrollText class="w-4 h-4 text-primary"/>
                <span class="font-bold text-[11px] uppercase tracking-[0.2em]">OCR Trace</span>
              </div>
              <Badge variant="secondary" class="text-[9px] font-mono tracking-tighter">RAW_V1</Badge>
            </div>
            <div
                class="overflow-y-auto p-6 text-[12px] font-mono leading-relaxed text-muted-foreground/80 whitespace-pre-wrap select-all selection:bg-primary selection:text-primary-foreground">
              {{ storedOcr.raw_text }}
            </div>
          </div>
        </aside>
      </div>
    </main>
  </div>
</template>

<style scoped>
input::placeholder {
  transition: opacity 0.2s;
}

input:focus::placeholder {
  opacity: 0.5;
}
</style>