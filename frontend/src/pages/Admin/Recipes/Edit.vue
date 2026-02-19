<script setup lang="ts">
import {onMounted, ref, watch} from "vue"
import {useRoute, useRouter} from "vue-router"
import {useI18n} from "vue-i18n"

import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Textarea} from "@/components/ui/textarea"
import {Switch} from "@/components/ui/switch"
import {Label} from "@/components/ui/label"
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card"
import {Separator} from "@/components/ui/separator"
import {Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select"

import IngredientsEditor from "@/pages/Admin/Recipes/IngredientsEditor.vue"
import StepsEditor from "@/pages/Admin/Recipes/StepsEditor.vue"
import TagsMultiSelect from "@/components/Recipe/TagsMultiSelect.vue"
import JsonImporter from "@/components/json/JsonImporter.vue"

import type {StepImage} from "@/models/RecipeCreate"
import type {RecipeEditor, RecipeTranslation} from "@/models/Recipe.ts"
import type {Language} from "@/models/Language.ts"

import {getRecipeByIdEditor, updateRecipe} from "@/api/recipe"
import {getAllLanguage} from "@/api/Language.ts"
import {ROUTES} from "@/router/routes.ts"
import type { Tag as RecipeTag, InputTag } from "@/models/Tag.ts";

const { t } = useI18n()
const apiUrl = import.meta.env.VITE_STATIC_URL
const route = useRoute()
const router = useRouter()

const recipe = ref<RecipeEditor | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

// Language state
const currentLang = ref("")
const available_languages = ref<Language[]>([])

/* ========= LOAD ========= */

onMounted(async () => {
  loading.value = true
  try {
    const [langs, data] = await Promise.all([
      getAllLanguage(),
      getRecipeByIdEditor(route.params.id as string, true)
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
    await router.push(ROUTES.ADMIN.RECIPE.VIEW(updated.id))
  } catch (e) {
    console.error(e)
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <div class="max-w-[1600px] mx-auto p-6 flex flex-col lg:flex-row gap-6 items-start justify-center">

    <div v-if="recipe" class="max-w-4xl min-w-[60%] mx-auto p-6 space-y-6">
      <div class="flex justify-between items-center">
        <h1 class="text-3xl font-bold">{{ t('Admin.recipe.editTitle') }}</h1>
      </div>

      <JsonImporter v-model="recipe"/>

      <Card>
        <CardHeader>
          <CardTitle>{{ t('Admin.recipe.basicInfo') }}</CardTitle>
        </CardHeader>

        <CardContent class="space-y-6">
          <div v-if="available_languages.length > 0">
            <div class="flex border-b mb-4">
              <button
                  v-for="lang in available_languages"
                  :key="lang.code"
                  @click="currentLang = lang.code"
                  type="button"
                  class="px-4 py-2 text-sm font-medium transition-colors border-b-2"
                  :class="currentLang === lang.code ? 'border-primary text-primary' : 'border-transparent text-muted-foreground'"
              >
                {{ lang.name }}
              </button>
            </div>

            <div v-for="lang in available_languages" :key="lang.code">
              <div v-if="currentLang === lang.code" class="space-y-4">
                <div class="space-y-2">
                  <Label :for="'title-' + lang.code">
                    {{ t('Admin.recipe.fields.title') }} ({{ lang.code.toUpperCase() }})
                  </Label>
                  <Input
                      :id="'title-' + lang.code"
                      v-model="getTranslation(lang.code).title"
                      :placeholder="t('Admin.recipe.placeholders.title')"
                  />
                </div>

                <div class="space-y-2">
                  <Label :for="'description-' + lang.code">
                    {{ t('Admin.recipe.fields.description') }} ({{ lang.code.toUpperCase() }})
                  </Label>
                  <Textarea
                      :id="'description-' + lang.code"
                      v-model="getTranslation(lang.code).description"
                      :placeholder="t('Admin.recipe.placeholders.description')"
                  />
                </div>
              </div>
            </div>
          </div>

          <Separator />

          <div class="space-y-2">
            <Label>{{ t('Admin.recipe.fields.image') }}</Label>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <Input type="file" accept="image/*" @change="onMainImageChange"/>
              <div class="ml-auto flex items-center gap-2">
                <Label>{{ t('Admin.recipe.fields.private') }}</Label>
                <Switch v-model:checked="recipe.is_private"/>
              </div>
            </div>
            <img v-if="mainImagePreview" :src="mainImagePreview" class="h-40 rounded border object-cover mt-2" />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader><CardTitle>{{ t('Admin.recipe.details') }}</CardTitle></CardHeader>
        <CardContent class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div class="space-y-2">
            <Label>{{ t('Admin.recipe.fields.servings') }}</Label>
            <Input type="number" min="1" v-model.number="recipe.servings"/>
          </div>
          <div class="space-y-2">
            <Label>{{ t('Admin.recipe.fields.prepTime') }}</Label>
            <Input type="number" min="0" v-model.number="recipe.prep_time_minutes"/>
          </div>
          <div class="space-y-2">
            <Label>{{ t('Admin.recipe.fields.cookTime') }}</Label>
            <Input type="number" min="0" v-model.number="recipe.cook_time_minutes"/>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader><CardTitle>{{ t('Admin.recipe.tags') }}</CardTitle></CardHeader>
        <CardContent><TagsMultiSelect v-model:model-value="recipe.tags"/></CardContent>
      </Card>

      <Card>
        <CardHeader><CardTitle>{{ t('Admin.recipe.recipeSection') }}</CardTitle></CardHeader>
        <CardContent class="space-y-8">
          <IngredientsEditor
              v-model="recipe.ingredient_groups"
              :available-languages="available_languages"
              :current-lang="currentLang"
          />
          <Separator />
          <StepsEditor
              v-model="recipe.step_groups"
              :available-languages="available_languages"
              :current-lang="currentLang"
          />
        </CardContent>
      </Card>
    </div>

    <aside v-if="recipe" class="sticky top-6 hidden xl:flex flex-col gap-4 w-60">
      <div class="bg-card border rounded-xl p-4 shadow-md space-y-4">
        <div class="space-y-2">
          <Label class="text-[10px] font-bold text-muted-foreground uppercase px-1">
            {{ t('Admin.recipe.fields.primaryLanguage') }}
          </Label>
          <Select v-model="(recipe as any).primary_language">
            <SelectTrigger class="w-full bg-background">
              <SelectValue :placeholder="t('Admin.recipe.placeholders.selectLanguage')" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem v-for="lang in available_languages" :key="lang.code" :value="lang.code">
                  {{ lang.name }}
                </SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>

        <Separator />

        <div class="space-y-2">
          <Label class="text-[10px] font-bold text-muted-foreground uppercase px-1">
            {{ t('Admin.recipe.fields.switchLanguage') }}
          </Label>
          <div class="flex flex-col gap-1">
            <button
                v-for="lang in available_languages"
                :key="lang.code"
                @click="currentLang = lang.code"
                type="button"
                class="flex items-center justify-between px-3 py-2 text-sm font-medium rounded-md transition-all border"
                :class="currentLang === lang.code
                  ? 'bg-primary text-primary-foreground border-primary shadow-sm'
                  : 'hover:bg-muted text-muted-foreground border-transparent'"
            >
              <span>{{ lang.name }}</span>
              <span class="text-[10px] uppercase opacity-70">{{ lang.code }}</span>
            </button>
          </div>
        </div>

        <Separator />

        <div class="pt-2">
          <Button :disabled="submitting" @click="submit" class="w-full shadow-lg h-11">
            {{ t('Admin.common.save') }}
          </Button>
        </div>
      </div>
    </aside>

    <div v-else-if="loading" class="p-10 text-center">Loading recipe...</div>
    <div v-else-if="error" class="p-10 text-center text-destructive">{{ error }}</div>
  </div>
</template>