<script setup lang="ts">
import {computed, onMounted, ref} from "vue"
import {Check, ChevronLeft, Image as ImageIcon, ScrollText, Sparkles, Timer, Upload, Users} from "lucide-vue-next"
import {Button} from "@/components/ui/button"
import {Badge} from "@/components/ui/badge"
import {Input} from "@/components/ui/input"
import {Separator} from "@/components/ui/separator" // Added for consistency
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
import {useRoute} from "vue-router";
import {useAuthStore} from "@/stores/auth.ts";

const authStore = useAuthStore();
const storedOcr = ref<OcrRecipeResponse | null>(null)
const currentEditLang = ref("fr")
const available_languages = ref<Language[]>([])
const viewMode = ref<'editor' | 'preview' | 'split'>('split')
const units = ref<Unit[]>([]);

const NIL_UUID = "00000000-0000-0000-0000-000000000000";
const mainImagePreview = ref<string | null>(null)
const route = useRoute()

const isStudio = computed(() => route.path.startsWith('/studio'))

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
  if (mainImagePreview.value) URL.revokeObjectURL(mainImagePreview.value)
  recipe.value.image_url = file
  mainImagePreview.value = file ? URL.createObjectURL(file) : null
}

const hydrateFromBackend = () => {
  if (!storedOcr.value || units.value.length === 0) return;
  const data = storedOcr.value;
  const defaultUnitId = units.value.find(u => u.code === "PIECE")?.id || units.value[0]?.id || NIL_UUID;

  recipe.value.primary_language = data.primary_language || "fr";
  recipe.value.servings = Number(data.detected_servings) || 1;
  recipe.value.translations = [
    {language_code: "fr", title: data.title_fr || "", description: ""},
    {language_code: "en", title: data.title_en || "", description: ""}
  ];

  recipe.value.ingredient_groups = data.ingredient_groups.map((group, gIdx) => ({
    position: gIdx,
    translations: [
      {language_code: "fr", title: group.name_fr || "Ingrédients"},
      {language_code: "en", title: group.name_en || "Ingredients"}
    ],
    ingredients: group.ingredients.map(ing => {
      const found = units.value.find(u =>
          u.name_en?.toLowerCase() === ing.unit?.term_en?.toLowerCase() ||
          u.name_fr?.toLowerCase() === ing.unit?.term_fr?.toLowerCase()
      );
      const finalUnitId = (!found || found.id === NIL_UUID) ? defaultUnitId : found.id;
      return {
        quantity: Number(ing.quantity) || 0,
        unit_id: finalUnitId,
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

  recipe.value.step_groups = data.step_groups.map((group, gIdx) => ({
    position: gIdx,
    translations: [
      {language_code: "fr", title: group.name_fr || "Préparation"},
      {language_code: "en", title: group.name_en || "Preparation"}
    ],
    steps: group.steps.map(step => ({
      position: step.position,
      image_url: null,
      duration_minutes: null,
      translations: [
        {language_code: "fr", instruction: step.raw_text_fr || ""},
        {language_code: "en", instruction: step.raw_text_en || ""}
      ]
    }))
  }));
};

const previewRecipe = computed(() => {
  const lang = currentEditLang.value;
  return {
    ...recipe.value,
    title: recipe.value.translations.find(t => t.language_code === lang)?.title || "",
    image_url: mainImagePreview.value || recipe.value.image_url,
    ingredient_groups: recipe.value.ingredient_groups.map(g => ({
      title: g.translations.find(t => t.language_code === lang)?.title || "",
      ingredients: g.ingredients.map(i => {
        const trans = i.translations.find(t => t.language_code === lang);
        return {
          ...i,
          display_name: trans?.data || "",
          note: trans?.note || "",
          unit: units.value.find(u => u.id === i.unit_id) || {symbol: ""}
        };
      })
    })),
    step_groups: recipe.value.step_groups.map(g => ({
      title: g.translations.find(t => t.language_code === lang)?.title || g.translations[0]?.title || "",
      steps: g.steps.map(s => ({
        ...s,
        instruction: s.translations.find(t => t.language_code === lang)?.instruction || ""
      }))
    }))
  } as any;
});

const submit = async () => {
  if (isStudio)
    recipe.value.author_id = authStore.user.id;
  try {
    const res = await confirmOcrRecipe(recipe.value);
    localStorage.removeItem('pending-ocr-data');
    if (isStudio.value) {
      await router.push(ROUTES.STUDIO.MY_RECIPES)
    } else {
      await router.push(ROUTES.ADMIN.RECIPE.VIEW(res.id))
    }
  } catch (e) {
    console.error("Submission failed.", e);
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
  <div v-if="storedOcr" class="min-h-screen">

    <nav class="sticky top-[-2rem] z-50
                -mt-8 -mx-8 mb-12
                w-[calc(100%+64px)]
                border-b border-white/5 bg-[#0a0a0a]/90 backdrop-blur-3xl shadow-2xl">
      <div class="max-w-400 mx-auto h-20 px-8 flex items-center justify-between">

        <div class="flex items-center gap-8">
          <div class="flex items-center gap-3">
            <Button variant="ghost" size="icon" @click="router.back()"
                    class="h-10 w-10 rounded-xl hover:bg-primary/10 hover:text-primary transition-all">
              <ChevronLeft class="w-5 h-5"/>
            </Button>
            <div class="h-8 w-px bg-primary/10 mx-1 hidden md:block"></div>
            <div class="hidden md:block">
              <h2 class="text-lg font-black tracking-tighter leading-none uppercase italic">Studio<span
                  class="text-primary">.</span>ocr</h2>
              <p class="text-[9px] font-bold uppercase tracking-[0.4em] text-muted-foreground/30 mt-1">Verification
                Engine</p>
            </div>
          </div>

          <div class="flex bg-black/20 p-1.5 rounded-2xl border border-white/3 shadow-inner">
            <button v-for="mode in ['split', 'editor', 'preview']" :key="mode" @click="viewMode = mode"
                    :class="[viewMode === mode ? 'bg-background shadow-[0_4px_12px_rgba(0,0,0,0.3)] text-primary scale-100' : 'text-muted-foreground/40 hover:text-foreground scale-95']"
                    class="px-5 py-2 text-[10px] font-black uppercase rounded-xl transition-all duration-300 ease-out flex items-center gap-2">
              <div v-if="viewMode === mode"
                   class="w-1.5 h-1.5 rounded-full bg-primary shadow-[0_0_8px_rgba(var(--primary),0.8)]"></div>
              {{ mode }}
            </button>
          </div>
        </div>

        <div class="flex items-center gap-6">
          <div class="hidden lg:flex items-center gap-1 bg-black/20 p-1 rounded-full border border-white/3">
            <button v-for="l in available_languages" :key="l.code" @click="currentEditLang = l.code"
                    :class="[currentEditLang === l.code ? 'bg-primary text-primary-foreground shadow-lg' : 'text-muted-foreground/30 hover:text-muted-foreground']"
                    class="w-10 h-8 text-[10px] font-black uppercase rounded-full transition-all">
              {{ l.code }}
            </button>
          </div>

          <div class="h-6 w-px bg-primary/10 mx-2 hidden sm:block"></div>

          <Button @click="submit"
                  class="group relative h-12 overflow-hidden rounded-2xl bg-primary px-10 text-[11px] font-black uppercase tracking-[0.2em] text-primary-foreground transition-all hover:shadow-[0_0_40px_-10px_rgba(var(--primary),0.6)] active:scale-95">
            <div
                class="absolute inset-0 bg-white/10 translate-y-12 group-hover:translate-y-0 transition-transform duration-500"></div>
            <span class="relative flex items-center gap-2">
              <Check class="w-4 h-4"/>
              Confirm Import
            </span>
          </Button>
        </div>
      </div>
    </nav>

    <main class="max-w-350 mx-auto p-6 md:p-12">

      <div v-if="viewMode === 'preview'"
           class="max-w-4xl mx-auto border-none rounded-[3rem] p-4 md:p-12 bg-card shadow-2xl animate-in fade-in zoom-in-95 duration-500">
        <RecipeDisplay :recipe="previewRecipe" :multiplier="1"/>
      </div>

      <div v-else class="flex flex-col lg:flex-row gap-12 md:gap-20 items-start">

        <div :class="[viewMode === 'split' ? 'lg:w-[62%]' : 'w-full']"
             class="space-y-12 md:space-y-20 w-full animate-in slide-in-from-bottom-4 duration-700">

          <div class="space-y-4">
            <div class="flex items-center gap-2 px-1">
              <Sparkles class="w-4 h-4 text-primary animate-pulse"/>
              <label class="text-[10px] font-black uppercase tracking-[0.2em] text-primary/60 italic">AI Suggested
                Title</label>
            </div>
            <textarea v-model="recipe.translations.find(t => t.language_code === currentEditLang)!.title"
                      rows="2"
                      class="w-full text-4xl md:text-7xl font-black bg-transparent border-none outline-none p-0 focus:ring-0 placeholder:text-muted/10 leading-[1.1] resize-none tracking-tighter"
                      placeholder="Recipe Name..."></textarea>
          </div>

          <div
              class="grid grid-cols-1 md:grid-cols-12 gap-10 p-8 md:p-12 bg-card/40 backdrop-blur-sm border border-white/5 shadow-2xl rounded-[2.5rem]">

            <div class="md:col-span-5 lg:col-span-4 space-y-4">
              <label
                  class="text-[10px] font-black uppercase tracking-widest flex items-center gap-2 text-muted-foreground/60 px-1">
                <ImageIcon class="w-4 h-4"/>
                Main Visual
              </label>
              <div
                  class="relative group aspect-square bg-muted/20 border-2 border-dashed border-white/10 rounded-[2rem] flex flex-col items-center justify-center overflow-hidden transition-all hover:border-primary/40 hover:bg-primary/5 cursor-pointer">
                <img v-if="mainImagePreview" :src="mainImagePreview"
                     class="object-cover w-full h-full transition-transform duration-700 group-hover:scale-110"/>
                <div v-else class="flex flex-col items-center gap-4 text-muted-foreground/40 text-center p-6">
                  <div class="p-4 bg-muted/50 rounded-2xl group-hover:scale-110 transition-transform">
                    <Upload class="w-8 h-8 stroke-[1.5]"/>
                  </div>
                  <span class="text-[10px] font-black uppercase tracking-widest">Add Display Photo</span>
                </div>
                <input type="file" accept="image/*" @change="onMainImageChange"
                       class="absolute inset-0 opacity-0 cursor-pointer z-10"/>
              </div>
            </div>

            <div class="md:col-span-7 lg:col-span-8 flex flex-col justify-center gap-8">
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                <div class="space-y-3">
                  <label
                      class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/50 ml-2">Servings</label>
                  <div class="relative group">
                    <Users
                        class="absolute left-5 top-1/2 -translate-y-1/2 w-5 h-5 text-muted-foreground/30 group-focus-within:text-primary transition-colors"/>
                    <Input type="number" v-model.number="recipe.servings"
                           class="pl-14 h-16 bg-background/50 rounded-2xl border-none shadow-inner text-xl font-black"/>
                  </div>
                </div>
                <div class="space-y-3">
                  <label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/50 ml-2">Prep
                    Time</label>
                  <div class="relative group">
                    <Timer
                        class="absolute left-5 top-1/2 -translate-y-1/2 w-5 h-5 text-muted-foreground/30 group-focus-within:text-primary transition-colors"/>
                    <Input type="number" v-model.number="recipe.prep_time_minutes"
                           class="pl-14 h-16 bg-background/50 rounded-2xl border-none shadow-inner text-xl font-black"/>
                  </div>
                </div>
                <div class="sm:col-span-2 space-y-3">
                  <label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground/50 ml-2">Cook
                    Time</label>
                  <div class="relative group">
                    <Timer
                        class="absolute left-5 top-1/2 -translate-y-1/2 w-5 h-5 text-muted-foreground/30 group-focus-within:text-primary transition-colors"/>
                    <Input type="number" v-model.number="recipe.cook_time_minutes"
                           class="pl-14 h-16 bg-background/50 rounded-2xl border-none shadow-inner text-xl font-black"/>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="space-y-16">
            <div class="space-y-8">
              <IngredientsEditor
                  v-model="recipe.ingredient_groups"
                  :current-lang="currentEditLang"
                  :original-ocr-groups="storedOcr.ingredient_groups"
                  :units="units"
              />
            </div>

            <Separator class="opacity-10"/>

            <div class="space-y-8">
              <StepsEditor
                  v-model="recipe.step_groups"
                  :current-lang="currentEditLang"
                  :original-ocr-groups="storedOcr.step_groups"
              />
            </div>
          </div>
        </div>

        <aside v-if="viewMode === 'split'"
               class="hidden lg:block w-[32%] sticky top-20 space-y-6 self-start">
          <div
              class="rounded-[2.5rem] border border-white/5 bg-card/30 backdrop-blur-md shadow-2xl flex flex-col h-[calc(100vh-10rem)] overflow-hidden">

            <div class="p-6 border-b border-white/5 bg-muted/20 flex items-center justify-between shrink-0">
              <div class="flex items-center gap-3">
                <ScrollText class="w-5 h-5 text-primary"/>
                <span class="font-black text-[11px] uppercase tracking-[0.2em]">Raw AI Output</span>
              </div>
              <Badge variant="outline" class="text-[9px] font-mono opacity-40 px-3 py-1 rounded-full">SOURCE_TEXT
              </Badge>
            </div>

            <div
                class="flex-1 overflow-y-auto p-8 text-[12px] font-mono leading-loose text-muted-foreground/60 whitespace-pre-wrap select-all selection:bg-primary/20 selection:text-primary italic no-scrollbar">
              {{ storedOcr.raw_text }}
            </div>
          </div>
        </aside>
      </div>
    </main>
  </div>
</template>

<style>
/* GLOBAL BLOCK:
  This bypasses Vue's scope to force-hide the layout's scrollbar
*/
.custom-scrollbar,
[class*="custom-scrollbar"] {
  scrollbar-width: none !important;
  -ms-overflow-style: none !important;
}

.custom-scrollbar::-webkit-scrollbar,
[class*="custom-scrollbar"]::-webkit-scrollbar {
  display: none !important;
  width: 0 !important;
  height: 0 !important;
}
</style>

<style scoped>
/* SCOPED BLOCK:
  Hides scrollbar on the Title textarea and the OCR aside
*/
textarea,
.no-scrollbar {
  scrollbar-width: none !important;
  -ms-overflow-style: none !important;
}

textarea::-webkit-scrollbar,
.no-scrollbar::-webkit-scrollbar {
  display: none !important;
}

textarea::placeholder {
  transition: opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

textarea:focus::placeholder {
  opacity: 0.05;
}
</style>