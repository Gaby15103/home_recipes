<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Switch } from "@/components/ui/switch"
import { Label } from "@/components/ui/label"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Separator } from '@/components/ui/separator'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Dialog, DialogContent } from "@/components/ui/dialog"
import { Sparkles, Scan, Loader2, Image as ImageIcon, Upload, Timer, Users } from "lucide-vue-next"
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
import { getUnits } from "@/api/unit.ts"

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
const units = ref<any[]>([])

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

async function onImportFiles(e: Event) {
  const fileList = (e.target as HTMLInputElement).files
  if (!fileList || fileList.length === 0) return

  const files = Array.from(fileList)
  ocrFiles.value = files

  if (useGemini.value) {
    await handleGeminiProcessing(files)
  } else {
    ocrPreviewUrls.value.forEach(url => URL.revokeObjectURL(url))
    ocrPreviewUrls.value = files.map(file => URL.createObjectURL(file))
    isOcrModalOpen.value = true
  }
}

async function handleGeminiProcessing(files: File[]) {
  loading.value = true
  error.value = null
  try {
    const ocrData = await suggestRecipeFromFiles(files)
    if (ocrData) {
      recipe.value = {
        ...recipe.value,
        ...ocrData,
        translations: ocrData.translations || [],
        ingredient_groups: ocrData.ingredient_groups || [],
        step_groups: ocrData.step_groups || [],
        tags: ocrData.tags || []
      }
      currentLang.value = ocrData.primary_language || currentLang.value
    }
  } catch (err) {
    error.value = "Gemini failed to analyze the images."
  } finally {
    loading.value = false
  }
}

async function handleRegionalOcr(payload: { regions: any[], sourceLang: string }) {
  isOcrModalOpen.value = false
  loading.value = true
  error.value = null
  try {
    const ocrData = await createRecipeFromRegions(ocrFiles.value, payload.regions, payload.sourceLang)
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
  units.value = await getUnits()
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
  <div class="max-w-[1800px] mx-auto p-4 md:p-8 flex flex-col lg:flex-row gap-6 md:gap-10 items-start justify-center relative">

    <div v-if="loading" class="fixed inset-0 z-[100] flex items-center justify-center bg-background/80 backdrop-blur-xl p-4">
      <div class="flex flex-col items-center gap-6 p-8 md:p-12 bg-card border shadow-2xl rounded-[2rem] w-full max-w-sm">
        <div class="relative flex items-center justify-center">
          <Loader2 class="h-12 w-12 animate-spin text-primary absolute opacity-20" />
          <Sparkles class="h-6 w-6 text-primary animate-pulse" />
        </div>
        <div class="text-center space-y-2">
          <p class="font-black text-xl md:text-2xl tracking-tighter">{{ t('Admin.recipe.loading') }}</p>
          <p class="text-[9px] text-primary font-black uppercase tracking-[0.2em]">AI is Analyzing...</p>
        </div>
      </div>
    </div>

    <div v-show="!error" :class="['flex-1 w-full space-y-6 md:space-y-10 transition-all duration-700', loading ? 'blur-2xl scale-95 opacity-0' : 'opacity-100']">

      <div class="flex flex-col xl:flex-row justify-between items-start xl:items-end gap-6 border-b pb-8 px-1">
        <div class="space-y-1">
          <p class="text-[10px] md:text-[11px] font-black uppercase tracking-[0.3em] text-primary/60">Recipe Management</p>
          <h1 class="text-3xl md:text-6xl font-black tracking-tighter leading-tight break-words">{{ t('Admin.recipe.createTitle') }}</h1>
        </div>

        <div class="flex flex-col gap-2 w-full md:w-auto">
          <Label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground px-1">{{ t('Admin.recipe.fields.primaryLanguage') }}</Label>
          <Select v-model="recipe.primary_language">
            <SelectTrigger class="h-10 md:h-12 w-full md:w-[200px] font-bold bg-card border-none shadow-sm rounded-xl">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="lang in available_language" :key="lang.code" :value="lang.code" class="font-bold">
                {{ lang.name }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <Card class="xl:hidden border-none shadow-lg bg-primary/[0.03] border-primary/10 rounded-2xl overflow-hidden">
        <CardContent class="p-6 space-y-4">
          <div class="flex items-center gap-2">
            <Sparkles class="w-4 h-4 text-primary" />
            <span class="text-[10px] font-black uppercase tracking-widest">Magic Import</span>
          </div>
          <div class="grid grid-cols-2 gap-3">
            <div class="flex bg-muted/50 p-1 rounded-xl border">
              <button @click="useGemini = false" :class="[!useGemini ? 'bg-background shadow-sm text-foreground' : 'text-muted-foreground opacity-60']" class="flex-1 flex items-center justify-center gap-2 py-2 rounded-lg transition-all">
                <Scan class="w-4 h-4" /> <span class="text-[9px] font-black uppercase">Classic</span>
              </button>
              <button @click="useGemini = true" :class="[useGemini ? 'bg-background shadow-sm text-primary' : 'text-muted-foreground opacity-60']" class="flex-1 flex items-center justify-center gap-2 py-2 rounded-lg transition-all">
                <Sparkles class="w-4 h-4" /> <span class="text-[9px] font-black uppercase">Gemini</span>
              </button>
            </div>
            <div class="relative flex items-center justify-center bg-primary text-primary-foreground rounded-xl font-black text-[9px] uppercase tracking-widest overflow-hidden active:scale-95 transition-transform">
              <Upload class="w-4 h-4 mr-2" /> Import Photos
              <input type="file" multiple accept="image/*" @change="onImportFiles" :disabled="loading" class="absolute inset-0 opacity-0 cursor-pointer" />
            </div>
          </div>
        </CardContent>
      </Card>

      <JsonImporter v-model="recipe" />

      <Card class="border-none shadow-xl bg-card/40 backdrop-blur-sm rounded-2xl md:rounded-[2rem] overflow-hidden">
        <CardHeader class="bg-muted/30 py-6 px-8 border-b border-white/5">
          <CardTitle class="text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground">{{ t('Admin.recipe.basicInfo') }}</CardTitle>
        </CardHeader>
        <CardContent class="p-6 md:p-10 space-y-8">
          <div class="flex flex-nowrap overflow-x-auto no-scrollbar gap-2 p-1 bg-muted/50 rounded-xl w-full md:w-fit border">
            <button v-for="lang in available_language" :key="lang.code"
                    @click="currentLang = lang.code"
                    :class="[currentLang === lang.code ? 'bg-background shadow-md text-primary' : 'text-muted-foreground hover:text-foreground']"
                    class="whitespace-nowrap px-6 py-2 text-[10px] font-black uppercase rounded-lg transition-all duration-300">
              {{ lang.name }}
            </button>
          </div>

          <div v-for="lang in available_language" :key="lang.code">
            <div v-if="currentLang === lang.code" class="grid grid-cols-1 gap-6 animate-in fade-in slide-in-from-bottom-2">
              <div class="space-y-3">
                <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground ml-1">{{ t('Admin.recipe.fields.title') }}</label>
                <Input v-model="getTranslation(lang.code).title" class="h-12 md:h-16 text-lg md:text-2xl font-black rounded-xl bg-background/50" :placeholder="t('Admin.recipe.placeholders.title')" />
              </div>
              <div class="space-y-3">
                <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground ml-1">{{ t('Admin.recipe.fields.description') }}</label>
                <Textarea v-model="getTranslation(lang.code).description" class="min-h-[120px] rounded-xl bg-background/50 border-none p-5" :placeholder="t('Admin.recipe.placeholders.description')" />
              </div>
            </div>
          </div>

          <Separator class="opacity-50" />

          <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div class="space-y-3">
              <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground ml-1">Main Display Image</label>
              <div class="relative group aspect-video rounded-xl md:rounded-[1.5rem] border-2 border-dashed flex items-center justify-center overflow-hidden transition-all bg-muted/30 hover:border-primary/50 cursor-pointer">
                <img v-if="mainImagePreview" :src="mainImagePreview" class="object-cover w-full h-full" />
                <div v-else class="flex flex-col items-center gap-2 text-muted-foreground opacity-40">
                  <ImageIcon class="w-10 h-10" />
                  <span class="text-[9px] font-black uppercase tracking-widest">Upload Photo</span>
                </div>
                <input type="file" accept="image/*" @change="onMainImageChange" class="absolute inset-0 opacity-0 cursor-pointer" />
              </div>
            </div>
            <div class="flex flex-col justify-center gap-6 px-6 py-6 bg-primary/[0.03] border border-primary/5 rounded-xl">
              <div class="flex items-center justify-between">
                <div class="space-y-1">
                  <Label class="text-base font-black tracking-tight">Private Recipe</Label>
                  <p class="text-xs text-muted-foreground">Only visible to administrators and you.</p>
                </div>
                <Switch v-model:checked="recipe.is_private" class="scale-110 md:scale-125" />
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card class="border-none shadow-xl bg-card/40 rounded-2xl">
        <CardContent class="p-6 md:p-10 grid grid-cols-1 md:grid-cols-3 gap-6">
          <div class="space-y-3">
            <Label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground ml-1 flex items-center gap-2">
              <Users class="w-3 h-3" /> Servings
            </Label>
            <Input type="number" v-model.number="recipe.servings" class="h-14 text-center text-2xl font-black rounded-xl bg-background/50 border-none shadow-inner" />
          </div>
          <div class="space-y-3">
            <Label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground ml-1 flex items-center gap-2">
              <Timer class="w-3 h-3" /> Prep (min)
            </Label>
            <Input type="number" v-model.number="recipe.prep_time_minutes" class="h-14 text-center text-2xl font-black rounded-xl bg-background/50 border-none shadow-inner" />
          </div>
          <div class="space-y-3">
            <Label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground ml-1 flex items-center gap-2">
              <Timer class="w-3 h-3" /> Cook (min)
            </Label>
            <Input type="number" v-model.number="recipe.cook_time_minutes" class="h-14 text-center text-2xl font-black rounded-xl bg-background/50 border-none shadow-inner" />
          </div>
        </CardContent>
      </Card>

      <Card class="border-none shadow-xl bg-card/40 rounded-2xl overflow-hidden">
        <CardHeader class="bg-muted/20 py-4 px-8 border-b border-white/5">
          <CardTitle class="text-[10px] font-black uppercase tracking-widest text-muted-foreground">Classification & Tags</CardTitle>
        </CardHeader>
        <CardContent class="p-6">
          <TagsMultiSelect v-model:model-value="recipe.tags" />
        </CardContent>
      </Card>

      <Card class="border-none shadow-2xl bg-card/60 rounded-[2.5rem] overflow-hidden">
        <CardContent class="p-6 md:p-12 space-y-12">
          <div class="space-y-6">
            <h3 class="text-xl font-black tracking-tighter px-2">Ingredients</h3>
            <IngredientsEditor v-model="recipe.ingredient_groups" :available-languages="available_language" :current-lang="currentLang" :units="units" />
          </div>
          <Separator class="opacity-30" />
          <div class="space-y-6">
            <h3 class="text-xl font-black tracking-tighter px-2">Preparation Steps</h3>
            <StepsEditor v-model="recipe.step_groups" :available-languages="available_language" :current-lang="currentLang" />
          </div>
        </CardContent>
      </Card>

      <div class="xl:hidden pb-10 px-1">
        <Button :disabled="submitting || loading" @click="submit" class="w-full h-16 rounded-2xl shadow-xl text-sm font-black uppercase tracking-widest">
          {{ submitting ? 'Saving...' : t('Admin.common.create') }}
        </Button>
      </div>
    </div>

    <aside class="sticky top-10 hidden xl:flex flex-col gap-6 w-80 shrink-0">
      <div class="bg-card/80 backdrop-blur-md border rounded-[2.5rem] p-8 shadow-2xl space-y-8">

        <div class="space-y-6">
          <div class="flex items-center gap-2 px-1">
            <Sparkles class="w-4 h-4 text-primary" />
            <span class="text-[11px] font-black uppercase tracking-widest">Magic Import</span>
          </div>

          <div class="grid grid-cols-2 gap-2 p-1.5 bg-muted/50 rounded-2xl border">
            <button @click="useGemini = false" type="button" :class="[!useGemini ? 'bg-background shadow-md text-foreground scale-105' : 'text-muted-foreground opacity-60']" class="flex flex-col items-center gap-2 py-4 rounded-xl transition-all">
              <Scan class="w-5 h-5" /> <span class="text-[9px] font-black uppercase">Classic</span>
            </button>
            <button @click="useGemini = true" type="button" :class="[useGemini ? 'bg-background shadow-md text-primary scale-105' : 'text-muted-foreground opacity-60']" class="flex flex-col items-center gap-2 py-4 rounded-xl transition-all">
              <Sparkles class="w-5 h-5" /> <span class="text-[9px] font-black uppercase">Gemini</span>
            </button>
          </div>

          <div class="relative group h-32 rounded-2xl border-2 border-dashed flex flex-col items-center justify-center transition-all hover:bg-primary/5 hover:border-primary/40 cursor-pointer overflow-hidden">
            <div class="flex flex-col items-center gap-2 text-muted-foreground opacity-60 group-hover:opacity-100 transition-opacity">
              <Upload class="w-6 h-6" /> <span class="text-[9px] font-black uppercase">Import Photos</span>
            </div>
            <input type="file" multiple accept="image/*" @change="onImportFiles" :disabled="loading" class="absolute inset-0 opacity-0 cursor-pointer" />
          </div>
        </div>

        <Separator class="opacity-50" />

        <div class="space-y-4">
          <Label class="text-[10px] font-black uppercase tracking-widest text-muted-foreground px-2">Editor View</Label>
          <div class="flex flex-col gap-2">
            <button v-for="lang in available_language" :key="lang.code"
                    @click="currentLang = lang.code"
                    class="flex items-center justify-between px-5 py-4 text-xs font-black rounded-xl border border-transparent transition-all"
                    :class="currentLang === lang.code ? 'bg-primary text-primary-foreground shadow-lg' : 'hover:bg-muted text-muted-foreground/60'">
              <span>{{ lang.name }}</span>
              <span class="text-[9px] opacity-40">{{ lang.code }}</span>
            </button>
          </div>
        </div>

        <Button :disabled="submitting || loading" @click="submit" class="w-full h-16 rounded-2xl shadow-xl text-sm font-black uppercase tracking-widest transition-transform active:scale-95">
          {{ submitting ? 'Saving...' : t('Admin.common.create') }}
        </Button>
      </div>

      <div v-if="error" class="bg-destructive/10 border border-destructive/20 rounded-2xl p-4">
        <p class="text-[10px] text-destructive font-bold text-center italic">{{ error }}</p>
      </div>
    </aside>

    <Dialog v-model:open="isOcrModalOpen">
      <DialogContent class="sm:max-w-[98vw] w-[98vw] h-[95vh] p-0 flex flex-col overflow-hidden rounded-[2.5rem] border-none shadow-2xl">
        <RecipeRegionSelector v-if="ocrPreviewUrls.length > 0" :images="ocrPreviewUrls" :default-lang="recipe.primary_language" @zones-completed="handleRegionalOcr" @cancel="isOcrModalOpen = false" />
      </DialogContent>
    </Dialog>
  </div>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
</style>