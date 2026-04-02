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
  <div v-if="storedOcr" class="min-h-screen bg-background text-foreground antialiased font-sans pb-20 md:pb-0">
    <nav class="sticky top-0 z-50 border-b bg-background/95 backdrop-blur shadow-sm">
      <div class="max-w-[1400px] mx-auto h-14 md:h-16 px-4 md:px-6 flex items-center justify-between gap-2">
        <div class="flex items-center gap-2 md:gap-4 overflow-x-auto no-scrollbar py-1">
          <Button variant="ghost" size="icon" @click="router.back()" class="shrink-0 h-9 w-9 md:hidden">
            <ChevronLeft class="w-5 h-5"/>
          </Button>
          <Button variant="outline" size="sm" @click="router.back()"
                  class="hidden md:flex h-8 px-3 text-xs font-bold uppercase tracking-wider shrink-0">
            <ChevronLeft class="w-3.5 h-3.5 mr-1"/> Back
          </Button>

          <div class="flex bg-muted p-1 rounded-lg shrink-0">
            <button v-for="mode in ['split', 'editor', 'preview']" :key="mode" @click="viewMode = mode"
                    :class="[viewMode === mode ? 'bg-background shadow-sm text-foreground' : 'text-muted-foreground']"
                    class="px-3 md:px-4 py-1 text-[9px] md:text-[10px] font-black uppercase rounded-md transition-all">
              <span class="hidden md:inline">{{ mode }}</span>
              <span class="md:hidden">{{ mode[0] }}</span> </button>
          </div>
        </div>

        <div class="flex items-center gap-2 shrink-0">
          <div class="flex bg-muted p-1 rounded-lg">
            <button v-for="l in available_languages" :key="l.code" @click="currentEditLang = l.code"
                    :class="[currentEditLang === l.code ? 'bg-primary text-primary-foreground' : 'text-muted-foreground']"
                    class="px-2 md:w-9 h-6 text-[9px] md:text-[10px] font-black uppercase rounded-md">
              {{ l.code }}
            </button>
          </div>
          <Button size="sm" @click="submit"
                  class="h-9 md:h-8 font-black text-[10px] md:text-xs uppercase px-4 md:px-8 tracking-widest bg-primary shadow-lg shadow-primary/20">
            Confirm
          </Button>
        </div>
      </div>
    </nav>

    <main class="max-w-[1400px] mx-auto p-4 md:p-8 lg:p-12">
      <div v-if="viewMode === 'preview'" class="max-w-4xl mx-auto border rounded-2xl md:rounded-[3rem] p-4 md:p-10 bg-card shadow-2xl">
        <RecipeDisplay :recipe="previewRecipe" :multiplier="1"/>
      </div>

      <div v-else class="flex flex-col lg:flex-row gap-8 md:gap-16 items-start">
        <div :class="[viewMode === 'split' ? 'lg:w-[60%] xl:w-[65%]' : 'w-full']" class="space-y-10 md:space-y-16 w-full">

          <div class="space-y-6 md:space-y-10">
            <div class="space-y-2">
              <label class="text-[9px] md:text-[10px] font-black uppercase tracking-[0.2em] text-primary/60 px-1 italic">
                Recipe Title
              </label>
              <textarea v-model="recipe.translations.find(t => t.language_code === currentEditLang)!.title"
                        rows="2"
                        class="w-full text-3xl md:text-6xl lg:text-7xl font-black bg-transparent border-none outline-none p-0 focus:ring-0 placeholder:text-muted/20 leading-tight resize-none"
                        placeholder="The Grand Recipe..."></textarea>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-12 gap-6 md:gap-10 p-5 md:p-10 bg-card border shadow-xl rounded-2xl md:rounded-[2.5rem]">

              <div class="md:col-span-5 lg:col-span-4 space-y-3">
                <label class="text-[9px] font-black uppercase tracking-widest flex items-center gap-2 text-muted-foreground/60">
                  <ImageIcon class="w-3.5 h-3.5"/> Cover Image
                </label>
                <div class="relative group aspect-square bg-muted/30 border-2 border-dashed rounded-2xl md:rounded-[2rem] flex flex-col items-center justify-center overflow-hidden transition-all hover:border-primary/50">
                  <img v-if="mainImagePreview" :src="mainImagePreview" class="object-cover w-full h-full"/>
                  <div v-else class="flex flex-col items-center gap-3 text-muted-foreground/40 text-center p-4">
                    <Upload class="w-10 h-10 stroke-[1.5]"/>
                    <span class="text-[9px] font-black uppercase tracking-widest">Upload Photo</span>
                  </div>
                  <input type="file" accept="image/*" @change="onMainImageChange" class="absolute inset-0 opacity-0 cursor-pointer z-10"/>
                </div>
              </div>

              <div class="md:col-span-7 lg:col-span-8 flex flex-col justify-center gap-6 md:gap-8">
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 md:gap-6">
                  <div class="space-y-2">
                    <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground/60 ml-1">Servings</label>
                    <div class="relative group">
                      <Users class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground/40 group-focus-within:text-primary transition-colors"/>
                      <Input type="number" v-model.number="recipe.servings" class="pl-11 h-12 md:h-14 bg-background rounded-xl md:rounded-2xl border-none shadow-sm text-lg font-bold"/>
                    </div>
                  </div>
                  <div class="space-y-2">
                    <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground/60 ml-1">Prep Time (min)</label>
                    <div class="relative group">
                      <Timer class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground/40 group-focus-within:text-primary transition-colors"/>
                      <Input type="number" v-model.number="recipe.prep_time_minutes" class="pl-11 h-12 md:h-14 bg-background rounded-xl md:rounded-2xl border-none shadow-sm text-lg font-bold"/>
                    </div>
                  </div>
                  <div class="sm:col-span-2 space-y-2">
                    <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground/60 ml-1">Cook Time (min)</label>
                    <div class="relative group">
                      <Timer class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground/40 group-focus-within:text-primary transition-colors"/>
                      <Input type="number" v-model.number="recipe.cook_time_minutes" class="pl-11 h-12 md:h-14 bg-background rounded-xl md:rounded-2xl border-none shadow-sm text-lg font-bold"/>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="space-y-12 md:space-y-20">
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
        </div>

        <aside v-if="viewMode === 'split'" class="w-full lg:w-[35%] lg:sticky lg:top-24 space-y-6 shrink-0">
          <div class="rounded-2xl border bg-card/50 backdrop-blur shadow-xl flex flex-col max-h-[400px] lg:max-h-[calc(100vh-10rem)] overflow-hidden">
            <div class="p-4 border-b bg-muted/40 flex items-center justify-between">
              <div class="flex items-center gap-2">
                <ScrollText class="w-4 h-4 text-primary"/>
                <span class="font-black text-[10px] uppercase tracking-[0.2em]">Original OCR Source</span>
              </div>
              <Badge variant="outline" class="text-[9px] font-mono tracking-tighter opacity-50 px-2 py-0">V1</Badge>
            </div>
            <div class="overflow-y-auto p-5 text-[11px] font-mono leading-loose text-muted-foreground/70 whitespace-pre-wrap select-all selection:bg-primary selection:text-primary-foreground italic">
              {{ storedOcr.raw_text }}
            </div>
          </div>
        </aside>
      </div>
    </main>
  </div>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

textarea::placeholder {
  transition: opacity 0.2s;
}
textarea:focus::placeholder {
  opacity: 0.1;
}
</style>