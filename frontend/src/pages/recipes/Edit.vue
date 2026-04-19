<script setup lang="ts">
import {computed, onMounted, ref, watch} from "vue"
import {useRoute, useRouter} from "vue-router"
import {useI18n} from "vue-i18n"

import {ImageIcon} from 'lucide-vue-next'
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Textarea} from "@/components/ui/textarea"
import {Switch} from "@/components/ui/switch"
import {Label} from "@/components/ui/label"
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card"
import {Separator} from "@/components/ui/separator"
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select"

import IngredientsEditor from "@/components/recipe/editor/IngredientsEditor.vue"
import StepsEditor from "@/components/recipe/editor/StepsEditor.vue"
import TagsMultiSelect from "@/components/recipe/forms/TagsMultiSelect.vue"
import JsonImporter from "@/components/json/JsonImporter.vue"

import type {StepImage} from "@/models/RecipeCreate.ts"
import type {RecipeEditor, RecipeTranslation} from "@/models/Recipe.ts"
import type {Language} from "@/models/Language.ts"

import {getRecipeByIdEditor, updateRecipe} from "@/api/recipe.ts"
import {getAllLanguage} from "@/api/Language.ts"
import {ROUTES} from "@/router/routes.ts"
import type {InputTag, Tag as RecipeTag} from "@/models/Tag.ts";
import {getUnits} from "@/api/unit.ts";

const { t } = useI18n()
const apiUrl = import.meta.env.VITE_STATIC_URL
const route = useRoute()
const router = useRouter()

const isStudio = computed(() => route.path.startsWith('/studio'))
const recipe = ref<RecipeEditor | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)
const units = ref<any[]>([])

// Language state
const currentLang = ref("")
const available_languages = ref<Language[]>([])

/* ========= LOAD ========= */

onMounted(async () => {
  loading.value = true
  try {
    const [langs, data,fetchedUnits] = await Promise.all([
      getAllLanguage(),
      getRecipeByIdEditor(route.params.id as string, true),
      getUnits()
    ])
    const transformedTags: InputTag[] = (data.tags as RecipeTag[]).map((tag): InputTag => {
      return {
        type: 'Existing',
        id: tag.id
      };
    });

    recipe.value = {
      ...data,
      tags: transformedTags
    }
    available_languages.value = langs
    units.value = fetchedUnits

    // Set initial tab to primary language or first available
    currentLang.value = langs.find(l => l.is_default)?.code || langs[0]?.code
  } catch (err: any) {
    error.value = err.message ?? "Failed to fetch recipe"
  } finally {
    loading.value = false
  }
})

/* ========= HELPER ========= */

function getTranslation(code: string): RecipeTranslation {
  if (!recipe.value) return { language_code: code, title: "", description: "" }

  let trans = recipe.value.translations.find(t => t.language_code === code)
  if (!trans) {
    trans = { language_code: code, title: "", description: "" }
    recipe.value.translations.push(trans)
  }
  return trans
}

/* ========= IMAGES ========= */

const stepImages = ref<StepImage[]>([])
const mainImageFile = ref<File | null>(null)
const mainImagePreview = ref<string | null>(null)

watch(recipe, r => {
  if (!r) return
  if (r.image_url && !mainImagePreview.value) {
    mainImagePreview.value = apiUrl + r.image_url
  }
}, { immediate: true })

function onMainImageChange(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0] ?? null
  if (mainImagePreview.value?.startsWith("blob:")) {
    URL.revokeObjectURL(mainImagePreview.value)
  }
  mainImageFile.value = file
  mainImagePreview.value = file ? URL.createObjectURL(file) : null
}

/* ========= SUBMIT ========= */

const submitting = ref(false)

async function submit() {
  if (!recipe.value) return
  submitting.value = true
  try {
    const updated = await updateRecipe(
        recipe.value.id,
        recipe.value,
        stepImages.value,
        mainImageFile.value
    )

    if (isStudio.value) {
      await router.push(ROUTES.STUDIO.MY_RECIPES)
    } else {
      await router.push(ROUTES.ADMIN.RECIPE.VIEW(updated.id))
    }
  } catch (e) {
    console.error(e)
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <div class="max-w-450 mx-auto p-4 md:p-8 flex flex-col lg:flex-row gap-6 md:gap-10 items-start justify-center relative">

    <div v-if="loading" class="fixed inset-0 z-100 flex items-center justify-center bg-background/80 backdrop-blur-xl p-4">
      <div class="flex flex-col items-center gap-6 p-8 md:p-12 bg-card border shadow-2xl rounded-4xl w-full max-w-sm text-center">
        <Loader2 class="h-12 w-12 animate-spin text-primary/40" />
        <p class="font-black text-xl tracking-tighter">Fetching Recipe...</p>
      </div>
    </div>

    <div v-if="recipe" v-show="!error"
         :class="['flex-1 w-full space-y-6 md:space-y-12 transition-all duration-700', loading ? 'blur-2xl scale-95 opacity-0' : 'opacity-100']">

      <div class="flex flex-col xl:flex-row justify-between items-start xl:items-end gap-4 md:gap-6 border-b pb-6 md:pb-8 px-1">
        <div class="space-y-1 flex-1">
          <p class="text-[9px] md:text-[11px] font-black uppercase tracking-[0.3em] text-primary/60">Editor Mode</p>
          <h1 class="text-3xl md:text-6xl font-black tracking-tighter leading-tight wrap-break-word">
            {{ t('Admin.recipe.editTitle') }}
          </h1>
        </div>

        <div class="flex flex-col gap-2 w-full md:w-auto shrink-0">
          <Label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground px-1">Primary Language</Label>
          <Select v-model="recipe.primary_language">
            <SelectTrigger class="h-10 md:h-12 w-full md:w-50 font-bold bg-card border-none shadow-sm rounded-xl">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="lang in available_languages" :key="lang.code" :value="lang.code" class="font-bold">
                {{ lang.name }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <JsonImporter v-model="recipe" />

      <div class="grid grid-cols-1 gap-6 md:gap-10">
        <Card class="border-none shadow-xl bg-card/40 backdrop-blur-sm rounded-3xl md:rounded-4xl overflow-hidden">
          <CardHeader class="bg-muted/30 py-4 md:py-6 px-6 md:px-10 border-b border-white/5">
            <CardTitle class="text-[10px] font-black uppercase tracking-[0.2em] text-muted-foreground">
              {{ t('Admin.recipe.basicInfo') }}
            </CardTitle>
          </CardHeader>

          <CardContent class="p-5 md:p-12 space-y-8 md:space-y-10">
            <div class="flex flex-nowrap md:flex-wrap gap-2 p-1 bg-muted/50 rounded-xl md:rounded-2xl w-full md:w-fit border overflow-x-auto no-scrollbar">
              <button v-for="lang in available_languages" :key="lang.code"
                      @click="currentLang = lang.code"
                      :class="[currentLang === lang.code ? 'bg-background shadow-sm text-primary scale-[1.02] md:scale-105' : 'text-muted-foreground hover:text-foreground']"
                      class="whitespace-nowrap px-4 md:px-8 py-2 md:py-2.5 text-[10px] md:text-[11px] font-black uppercase rounded-lg md:rounded-xl transition-all duration-300">
                {{ lang.name }}
              </button>
            </div>

            <div v-for="lang in available_languages" :key="lang.code">
              <div v-if="currentLang === lang.code" class="grid grid-cols-1 gap-6 md:gap-8 animate-in fade-in slide-in-from-bottom-2 duration-400">
                <div class="space-y-3">
                  <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground ml-1">{{ t('Admin.recipe.fields.title') }}</label>
                  <Input v-model="getTranslation(lang.code).title"
                         class="h-12 md:h-20 text-lg md:text-3xl font-black rounded-xl md:rounded-2xl bg-background/50 px-4 md:px-8"
                         :placeholder="t('Admin.recipe.placeholders.title')" />
                </div>
                <div class="space-y-3">
                  <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground ml-1">{{ t('Admin.recipe.fields.description') }}</label>
                  <Textarea v-model="getTranslation(lang.code).description"
                            class="min-h-30 md:min-h-45 text-sm md:text-lg rounded-xl md:rounded-2xl bg-background/50 border-none p-5 md:p-8"
                            :placeholder="t('Admin.recipe.placeholders.description')" />
                </div>
              </div>
            </div>

            <Separator class="opacity-50" />

            <div class="grid grid-cols-1 xl:grid-cols-2 gap-8 md:gap-12">
              <div class="space-y-3">
                <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground ml-1">Recipe Cover</label>
                <div class="relative group aspect-video md:aspect-21/9 xl:aspect-video rounded-xl md:rounded-4xl border-2 border-dashed flex items-center justify-center overflow-hidden transition-all bg-muted/30 hover:bg-muted/50 cursor-pointer">
                  <img v-if="mainImagePreview" :src="mainImagePreview" class="object-cover w-full h-full" />
                  <div v-else class="flex flex-col items-center gap-2 text-muted-foreground opacity-40">
                    <ImageIcon class="w-8 h-8 md:w-12 md:h-12" />
                    <span class="text-[9px] md:text-[11px] font-black uppercase tracking-widest">Change Photo</span>
                  </div>
                  <input type="file" accept="image/*" @change="onMainImageChange" class="absolute inset-0 opacity-0 cursor-pointer" />
                </div>
              </div>
              <div class="flex flex-col justify-center gap-4 md:gap-6 px-6 md:px-10 py-6 md:py-10 bg-primary/3 border border-primary/5 rounded-xl md:rounded-4xl">
                <div class="flex items-center justify-between gap-4">
                  <div class="space-y-1">
                    <Label class="text-base md:text-xl font-black tracking-tight">Private Recipe</Label>
                    <p class="text-[10px] md:text-sm text-muted-foreground max-w-60">Visible only to admins and author.</p>
                  </div>
                  <Switch v-model="recipe.is_private" class="scale-110 md:scale-150 shrink-0" />
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 md:gap-8">
          <Card v-for="metric in ['servings', 'prep_time_minutes', 'cook_time_minutes']" :key="metric" class="border-none shadow-lg bg-card/40 rounded-xl md:rounded-4xl">
            <CardContent class="p-6 md:p-10 space-y-2 md:space-y-4">
              <Label class="text-[9px] font-black uppercase tracking-[0.2em] text-muted-foreground block text-center italic">
                {{ metric.replace(/_/g, ' ') }}
              </Label>
              <Input type="number" v-model.number="recipe[metric]" class="h-12 md:h-20 text-center text-2xl md:text-4xl font-black rounded-lg md:rounded-2xl bg-background/50 border-none shadow-inner" />
            </CardContent>
          </Card>
        </div>

        <Card class="border-none shadow-xl bg-card/40 rounded-xl md:rounded-4xl">
          <CardHeader class="py-4 px-10 border-b border-white/5">
            <CardTitle class="text-[10px] font-black uppercase tracking-widest text-muted-foreground">Categorization</CardTitle>
          </CardHeader>
          <CardContent class="p-8">
            <TagsMultiSelect v-model:model-value="recipe.tags"/>
          </CardContent>
        </Card>

        <Card class="border-none shadow-2xl bg-card/60 rounded-[1.5rem] md:rounded-[3rem] overflow-hidden">
          <CardHeader class="bg-muted/30 py-5 md:py-8 px-6 md:px-12">
            <CardTitle class="text-[10px] md:text-xs font-black uppercase tracking-[0.3em] text-primary">Recipe Documentation</CardTitle>
          </CardHeader>
          <CardContent class="p-5 md:p-16 space-y-12 md:space-y-20">
            <IngredientsEditor
                v-model="recipe.ingredient_groups"
                :available-languages="available_languages"
                :current-lang="currentLang"
                :units="units"
            />
            <Separator class="opacity-30" />
            <StepsEditor
                v-model="recipe.step_groups"
                :available-languages="available_languages"
                :current-lang="currentLang"
            />
          </CardContent>
        </Card>
      </div>
    </div>

    <aside v-if="recipe" class="w-full lg:w-72 xl:w-80 shrink-0 lg:sticky lg:top-10">
      <div class="bg-card/80 backdrop-blur-md border rounded-[1.5rem] md:rounded-[2.5rem] p-5 md:p-8 shadow-2xl space-y-6 md:space-y-8 border-white/10">

        <div class="space-y-4">
          <Label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground px-2">Editor Context</Label>
          <div class="flex flex-col gap-2">
            <button v-for="lang in available_languages" :key="lang.code"
                    @click="currentLang = lang.code"
                    class="flex items-center justify-between px-4 py-3 md:py-4 text-[11px] font-black rounded-xl md:rounded-2xl transition-all border border-transparent shadow-sm"
                    :class="currentLang === lang.code
                      ? 'bg-primary text-primary-foreground'
                      : 'bg-muted/40 text-muted-foreground hover:bg-muted'">
              <span class="truncate max-w-[100px] md:max-w-[140px]">{{ lang.name }}</span>
              <span class="text-[8px] md:text-[9px] opacity-40 uppercase">{{ lang.code }}</span>
            </button>
          </div>
        </div>

        <Separator class="opacity-50" />

        <div class="space-y-3">
          <Button :disabled="submitting" @click="submit" class="w-full h-12 md:h-16 rounded-xl md:rounded-[1.5rem] shadow-xl text-[11px] md:text-sm font-black uppercase tracking-[0.2em]">
            <Loader2 v-if="submitting" class="w-4 h-4 mr-2 animate-spin" />
            {{ submitting ? 'Updating...' : t('Admin.common.save') }}
          </Button>

          <Button variant="ghost" @click="router.back()" class="w-full text-[10px] font-black uppercase tracking-widest opacity-50 hover:opacity-100">
            Cancel Changes
          </Button>
        </div>
      </div>

      <div v-if="error" class="mt-4 bg-destructive/10 border border-destructive/20 rounded-xl p-4">
        <p class="text-[11px] text-destructive font-bold text-center">{{ error }}</p>
      </div>
    </aside>

  </div>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
</style>