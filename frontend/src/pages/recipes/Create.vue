<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Switch } from "@/components/ui/switch"
import { Label } from "@/components/ui/label"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Separator } from '@/components/ui/separator'
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Dialog, DialogContent } from "@/components/ui/dialog"
import { Sparkles, Scan, Loader2, Image as ImageIcon } from "lucide-vue-next"
import { useI18n } from "vue-i18n"

import IngredientsEditor from "@/components/recipe/editor/IngredientsEditor.vue"
import StepsEditor from "@/components/recipe/editor/StepsEditor.vue"
import JsonImporter from "@/components/json/JsonImporter.vue"
import TagsMultiSelect from "@/components/recipe/forms/TagsMultiSelect.vue"
import RecipeRegionSelector from "@/components/recipe/forms/RecipeRegionSelector.vue"

import type { RecipeCreate } from "@/models/RecipeCreate.ts"
import type { Language } from "@/models/Language.ts"
import { createRecipe } from "@/api/recipe.ts"
import { getAllLanguage } from "@/api/Language.ts"
import { createRecipeFromRegions, suggestRecipeFromFiles } from "@/api/ocr.ts"
import router from "@/router"
import { ROUTES } from "@/router/routes.ts"

const { t } = useI18n()

// --- State ---
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

const mainImagePreview = ref<string | null>(null)
const currentLang = ref("")
const available_language = ref<Language[]>([])
const loading = ref(false)
const submitting = ref(false)
const error = ref<string | null>(null)

// --- OCR / AI State ---
const useGemini = ref(false)
const isOcrModalOpen = ref(false)
const ocrFiles = ref<File[]>([])
const ocrPreviewUrls = ref<string[]>([])

// --- Actions ---
function onMainImageChange(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0] ?? null
  if (mainImagePreview.value) URL.revokeObjectURL(mainImagePreview.value)
  recipe.value.image_url = file
  mainImagePreview.value = file ? URL.createObjectURL(file) : null
}

async function submit() {
  submitting.value = true
  try {
    const res = await createRecipe(recipe.value)
    await router.push(ROUTES.ADMIN.RECIPE.VIEW(res.id))
  } catch (e: any) {
    console.error(e)
  } finally {
    submitting.value = false
  }
}

function getTranslation(code: string) {
  let trans = recipe.value.translations.find(t => t.language_code === code)
  if (!trans) {
    trans = { language_code: code, title: "", description: "" }
    recipe.value.translations.push(trans)
  }
  return trans
}

// Gestion de l'import (Choix entre Gemini / Regions)
async function onImportFiles(e: Event) {
  const fileList = (e.target as HTMLInputElement).files
  if (!fileList || fileList.length === 0) return

  const files = Array.from(fileList)
  ocrFiles.value = files

  if (useGemini.value) {
    // Appel à /ocr/process (Gemini)
    await handleGeminiProcessing(files)
  } else {
    // Préparation pour /ocr/process_regions (Interface de sélection)
    ocrPreviewUrls.value.forEach(url => URL.revokeObjectURL(url))
    ocrPreviewUrls.value = files.map(file => URL.createObjectURL(file))
    isOcrModalOpen.value = true
  }
}

async function handleGeminiProcessing(files: File[]) {
  loading.value = true
  error.value = null

  try {
    // 1. Call your Gemini endpoint
    const ocrData = await suggestRecipeFromFiles(files)

    if (ocrData) {
      // 2. Map the result to your local state
      // We use Object.assign or a spread to keep the reactivity intact
      recipe.value = {
        ...recipe.value, // Keep existing metadata like image_url if not provided
        ...ocrData,
        // Ensure lists are replaced with the new AI structure
        translations: ocrData.translations || [],
        ingredient_groups: ocrData.ingredient_groups || [],
        step_groups: ocrData.step_groups || [],
        tags: ocrData.tags || []
      }

      // 3. (Optional) Set the current view to the primary language of the recipe
      currentLang.value = ocrData.primary_language || currentLang.value

      // Success feedback (optional)
      console.log("Recipe populated by Gemini ✨")
    }
  } catch (err) {
    error.value = "Gemini failed to analyze the images."
    console.error(err)
  } finally {
    loading.value = false
  }
}

// Phase 1 (Classique) : Appel au contrôleur /ocr/process_regions
async function handleRegionalOcr(payload: { regions: any[], sourceLang: string }) {
  isOcrModalOpen.value = false
  loading.value = true
  error.value = null
  try {
    const ocrData = await createRecipeFromRegions(
        ocrFiles.value,
        payload.regions,
        payload.sourceLang
    )
    if (ocrData) {
      localStorage.setItem('pending-ocr-data', JSON.stringify(ocrData))
      await router.push(ROUTES.ADMIN.RECIPE.OCR_REVIEW)
    }
  } catch (err) {
    error.value = "Regional processing failed."
  } finally {
    loading.value = false
  }
}

onMounted(async () => {
  const langs = await getAllLanguage()
  available_language.value = langs
  const defaultLang = langs.find(l => l.is_default)?.code || langs[0]?.code
  recipe.value.primary_language = defaultLang
  currentLang.value = defaultLang
})

onUnmounted(() => {
  ocrPreviewUrls.value.forEach(url => URL.revokeObjectURL(url))
})
</script>

<template>
  <div class="max-w-[1600px] mx-auto p-6 flex flex-col lg:flex-row gap-6 items-start justify-center relative">

    <div v-if="loading" class="fixed inset-0 z-[100] flex items-center justify-center bg-background/60 backdrop-blur-md">
      <div class="flex flex-col items-center gap-6 p-10 bg-card border shadow-2xl rounded-3xl">
        <Loader2 class="h-12 w-12 animate-spin text-primary" />
        <div class="text-center space-y-1">
          <p class="font-black text-xl tracking-tight">{{ t('Admin.recipe.loading') }}</p>
          <p class="text-xs text-muted-foreground uppercase tracking-widest opacity-70">Processing through AI</p>
        </div>
      </div>
    </div>

    <div v-show="!error" :class="['max-w-4xl min-w-6xl mx-auto p-6 space-y-8 transition-all duration-500', loading ? 'blur-xl scale-95 opacity-0' : '']">

      <div class="flex justify-between items-end border-b pb-6">
        <div class="space-y-1">
          <p class="text-[10px] font-black uppercase tracking-[0.3em] text-primary">Management</p>
          <h1 class="text-5xl font-black tracking-tighter">{{ t('Admin.recipe.createTitle') }}</h1>
        </div>

        <div class="flex flex-col items-end gap-2">
          <Label class="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">{{ t('Admin.recipe.fields.primaryLanguage') }}</Label>
          <Select v-model="recipe.primary_language">
            <SelectTrigger class="w-[180px] font-bold">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="lang in available_language" :key="lang.code" :value="lang.code">
                {{ lang.name }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <JsonImporter v-model="recipe" />

      <div class="grid grid-cols-1 gap-8">
        <Card class="overflow-hidden border-none shadow-lg bg-card/50">
          <CardHeader class="bg-muted/30">
            <CardTitle class="text-sm font-black uppercase tracking-widest">{{ t('Admin.recipe.basicInfo') }}</CardTitle>
          </CardHeader>
          <CardContent class="p-8 space-y-8">
            <div class="flex gap-2 p-1 bg-muted rounded-xl w-fit">
              <button v-for="lang in available_language" :key="lang.code"
                      @click="currentLang = lang.code"
                      :class="[currentLang === lang.code ? 'bg-background shadow-sm text-foreground' : 'text-muted-foreground hover:text-foreground']"
                      class="px-6 py-2 text-xs font-bold uppercase rounded-lg transition-all">
                {{ lang.name }}
              </button>
            </div>

            <div v-for="lang in available_language" :key="lang.code">
              <div v-if="currentLang === lang.code" class="grid grid-cols-1 gap-6 animate-in fade-in slide-in-from-bottom-2">
                <div class="space-y-3">
                  <Label class="text-xs font-bold uppercase">{{ t('Admin.recipe.fields.title') }}</Label>
                  <Input v-model="getTranslation(lang.code).title" class="h-12 text-lg font-bold" />
                </div>
                <div class="space-y-3">
                  <Label class="text-xs font-bold uppercase">{{ t('Admin.recipe.fields.description') }}</Label>
                  <Textarea v-model="getTranslation(lang.code).description" class="min-h-[100px] bg-muted/20 border-none" />
                </div>
              </div>
            </div>

            <Separator />

            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
              <div class="space-y-4">
                <Label class="text-xs font-bold uppercase">Main Display Image</Label>
                <div class="relative group aspect-video rounded-2xl border-2 border-dashed flex items-center justify-center overflow-hidden transition-all hover:border-primary/50 bg-muted/20">
                  <img v-if="mainImagePreview" :src="mainImagePreview" class="object-cover w-full h-full" />
                  <div v-else class="flex flex-col items-center gap-2 text-muted-foreground opacity-40">
                    <ImageIcon class="w-8 h-8" />
                    <span class="text-[10px] font-black uppercase">Click to upload</span>
                  </div>
                  <input type="file" accept="image/*" @change="onMainImageChange" class="absolute inset-0 opacity-0 cursor-pointer" />
                </div>
              </div>
              <div class="flex flex-col justify-center gap-4 px-6 py-4 bg-muted/20 rounded-2xl">
                <div class="flex items-center justify-between">
                  <div class="space-y-0.5">
                    <Label class="text-sm font-bold">Private Recipe</Label>
                    <p class="text-[10px] text-muted-foreground">Only visible to administrators</p>
                  </div>
                  <Switch v-model:checked="recipe.is_private" />
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card class="border-none shadow-lg">
          <CardContent class="p-8 grid grid-cols-1 md:grid-cols-3 gap-8">
            <div class="space-y-3">
              <Label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground">Servings</Label>
              <Input type="number" v-model.number="recipe.servings" class="h-12 text-center text-xl font-black" />
            </div>
            <div class="space-y-3">
              <Label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground">Prep Time (min)</Label>
              <Input type="number" v-model.number="recipe.prep_time_minutes" class="h-12 text-center text-xl font-black" />
            </div>
            <div class="space-y-3">
              <Label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground">Cook Time (min)</Label>
              <Input type="number" v-model.number="recipe.cook_time_minutes" class="h-12 text-center text-xl font-black" />
            </div>
          </CardContent>
        </Card>

        <Card class="border-none shadow-lg">
          <CardHeader><CardTitle class="text-sm font-black uppercase tracking-widest">Classification</CardTitle></CardHeader>
          <CardContent><TagsMultiSelect v-model:model-value="recipe.tags" /></CardContent>
        </Card>

        <Card class="border-none shadow-lg">
          <CardHeader><CardTitle class="text-sm font-black uppercase tracking-widest">Recipe Content</CardTitle></CardHeader>
          <CardContent class="space-y-12">
            <IngredientsEditor v-model="recipe.ingredient_groups" :available-languages="available_language" :current-lang="currentLang" />
            <Separator />
            <StepsEditor v-model="recipe.step_groups" :available-languages="available_language" :current-lang="currentLang" />
          </CardContent>
        </Card>
      </div>
    </div>

    <aside class="sticky top-6 hidden xl:flex flex-col gap-6 w-72 transition-all duration-300">
      <div class="bg-card border rounded-3xl p-6 shadow-xl space-y-6">

        <div class="space-y-4">
          <div class="flex items-center gap-2 px-1">
            <Sparkles class="w-4 h-4 text-primary" />
            <span class="text-[10px] font-black uppercase tracking-widest">Import Engine</span>
          </div>

          <div class="grid grid-cols-2 gap-2 p-1 bg-muted rounded-xl">
            <button @click="useGemini = false" type="button"
                    :class="[!useGemini ? 'bg-background shadow-sm text-foreground' : 'text-muted-foreground']"
                    class="flex flex-col items-center gap-2 py-3 rounded-lg transition-all">
              <Scan class="w-4 h-4" />
              <span class="text-[9px] font-bold uppercase">Manual OCR</span>
            </button>
            <button @click="useGemini = true" type="button"
                    :class="[useGemini ? 'bg-background shadow-sm text-primary' : 'text-muted-foreground']"
                    class="flex flex-col items-center gap-2 py-3 rounded-lg transition-all">
              <Sparkles class="w-4 h-4" />
              <span class="text-[9px] font-bold uppercase">Gemini AI</span>
            </button>
          </div>

          <div class="relative group h-24 rounded-2xl border-2 border-dashed flex flex-col items-center justify-center transition-all hover:bg-muted/50 cursor-pointer overflow-hidden">
            <div class="flex flex-col items-center gap-1 text-muted-foreground opacity-60 group-hover:opacity-100">
              <ImageIcon class="w-5 h-5" />
              <span class="text-[9px] font-black uppercase">Import Photos</span>
            </div>
            <input type="file" multiple accept="image/*" @change="onImportFiles" :disabled="loading" class="absolute inset-0 opacity-0 cursor-pointer" />
          </div>
          <p class="text-[10px] text-center italic text-muted-foreground">
            {{ useGemini ? 'Automated AI detection' : 'Select regions manually' }}
          </p>
        </div>

        <Separator />

        <div class="space-y-4">
          <div class="flex flex-col gap-1.5">
            <Label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground px-1">Switch View</Label>
            <button v-for="lang in available_language" :key="lang.code"
                    @click="currentLang = lang.code"
                    class="flex items-center justify-between px-4 py-3 text-xs font-bold rounded-xl transition-all border border-transparent"
                    :class="currentLang === lang.code ? 'bg-primary text-primary-foreground shadow-lg' : 'hover:bg-muted text-muted-foreground'">
              <span>{{ lang.name }}</span>
              <span class="opacity-40">{{ lang.code }}</span>
            </button>
          </div>
        </div>

        <Separator />

        <Button :disabled="submitting || loading" @click="submit" class="w-full h-14 rounded-2xl shadow-xl shadow-primary/20 text-sm font-black uppercase tracking-widest">
          {{ t('Admin.common.create') }}
        </Button>
      </div>

      <div v-if="error" class="bg-destructive/10 border border-destructive/20 rounded-2xl p-4 animate-in zoom-in-95">
        <p class="text-[11px] text-destructive font-bold text-center italic">{{ error }}</p>
      </div>
    </aside>

    <Dialog v-model:open="isOcrModalOpen">
      <DialogContent class="sm:max-w-[95vw] w-[95vw] h-[90vh] p-0 flex flex-col overflow-hidden rounded-3xl border-none">
        <RecipeRegionSelector
            v-if="ocrPreviewUrls.length > 0"
            :images="ocrPreviewUrls"
            :default-lang="recipe.primary_language"
            @zones-completed="handleRegionalOcr"
            @cancel="isOcrModalOpen = false"
        />
      </DialogContent>
    </Dialog>
  </div>
</template>

<style scoped>
.grayscale-50 { filter: grayscale(50%); }
</style>