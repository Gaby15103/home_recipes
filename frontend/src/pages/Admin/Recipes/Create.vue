<script setup lang="ts">
import {onMounted, ref} from "vue"
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Textarea} from "@/components/ui/textarea"
import {Switch} from "@/components/ui/switch"
import {Label} from "@/components/ui/label"
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card"
import {Separator} from '@/components/ui/separator'
import IngredientsEditor from "@/pages/Admin/Recipes/IngredientsEditor.vue";
import StepsEditor from "@/pages/Admin/Recipes/StepsEditor.vue";
import type {RecipeCreate} from "@/models/RecipeCreate.ts";
import {createRecipe} from "@/api/recipe.ts"
import router from "@/router";
import JsonImporter from "@/components/json/JsonImporter.vue";
import TagsMultiSelect from "@/components/Recipe/TagsMultiSelect.vue";
import {useI18n} from "vue-i18n";
import type {Language} from "@/models/Language.ts";
import {getAllLanguage} from "@/api/Language.ts";
import {Select, SelectTrigger, SelectValue, SelectItem, SelectGroup, SelectContent} from "@/components/ui/select";
const { t } = useI18n()

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

const mainImageFile = ref<File | null>(null)
const mainImagePreview = ref<string | null>(null)
const currentLang = ref("")
const available_language = ref<Language[]>()

function onMainImageChange(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0] ?? null

  if (mainImagePreview.value) {
    URL.revokeObjectURL(mainImagePreview.value)
  }
  recipe.value.image_url = file
  mainImageFile.value = file
  mainImagePreview.value = file ? URL.createObjectURL(file) : null
}


const submitting = ref(false)

async function submit() {
  console.log("Recipe payload:", recipe.value)
  try {
    const res = await createRecipe(recipe.value)
    await router.push("/recipe/" + res.id)
  } catch (e: any) {
    console.error(e)
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
onMounted(async () => {
  const langs = await getAllLanguage();
  available_language.value = langs;

  const defaultLang = langs.find(l => l.is_default)?.code || langs[0]?.code;
  recipe.value.primary_language = defaultLang;
  currentLang.value = defaultLang; // Set initial tab
})
</script>

<template>
  <div class="max-w-[1600px] mx-auto p-6 flex flex-col lg:flex-row gap-6 items-start justify-center">

    <div class="max-w-4xl min-w-6xl mx-auto p-6 space-y-6">
      <div class="flex justify-between items-center">
        <h1 class="text-3xl font-bold">{{ t('Admin.recipe.createTitle') }}</h1>

        <div class="flex items-center gap-2">
          <Label class="text-sm">{{ t('Admin.recipe.fields.primaryLanguage') }}</Label>
          <Select v-model="recipe.primary_language">
            <SelectTrigger class="w-[180px]">
              <SelectValue :placeholder="t('Admin.recipe.placeholders.selectLanguage')" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem
                    v-for="lang in available_language"
                    :key="lang.code"
                    :value="lang.code"
                >
                  {{ lang.name }}
                </SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>
      </div>

      <JsonImporter v-model="recipe" />

      <Card>
        <CardHeader>
          <CardTitle>{{ t('Admin.recipe.basicInfo') }}</CardTitle>
        </CardHeader>

        <CardContent class="space-y-6">
          <div v-if="available_language && available_language.length > 0">
            <div class="flex border-b mb-4">
              <button
                  v-for="lang in available_language"
                  :key="lang.code"
                  @click="currentLang = lang.code"
                  type="button"
                  class="px-4 py-2 text-sm font-medium transition-colors border-b-2"
                  :class="currentLang === lang.code ? 'border-primary text-primary' : 'border-transparent text-muted-foreground'"
              >
                {{ lang.name }}
              </button>
            </div>

            <div v-for="lang in available_language" :key="lang.code">
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
              <Input type="file" accept="image/*" @change="onMainImageChange" :v-model="recipe.image_url"/>
              <div class="ml-auto flex items-center gap-2">
                <Label>{{ t('Admin.recipe.fields.private') }}</Label>
                <Switch v-model:checked="recipe.is_private"/>
              </div>
            </div>
            <img v-if="mainImagePreview" :src="mainImagePreview" class="h-40 rounded border object-cover mt-2" alt=""/>
          </div>
        </CardContent>
      </Card>

      <!-- Numbers -->
      <Card>
        <CardHeader>
          <CardTitle>
            {{ t('Admin.recipe.details') }}
          </CardTitle>
        </CardHeader>

        <CardContent class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div class="space-y-2">
            <Label>
              {{ t('Admin.recipe.fields.servings') }}
            </Label>
            <Input type="number" min="1" v-model.number="recipe.servings"/>
          </div>

          <div class="space-y-2">
            <Label>
              {{ t('Admin.recipe.fields.prepTime') }}
            </Label>
            <Input type="number" min="0" v-model.number="recipe.prep_time_minutes"/>
          </div>

          <div class="space-y-2">
            <Label>
              {{ t('Admin.recipe.fields.cookTime') }}
            </Label>
            <Input type="number" min="0" v-model.number="recipe.cook_time_minutes"/>
          </div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          {{ t('Admin.recipe.tags') }}
        </CardHeader>
        <CardContent class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <TagsMultiSelect v-model:model-value="recipe.tags"/>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>
            {{ t('Admin.recipe.recipeSection') }}
          </CardTitle>
        </CardHeader>

        <CardContent class="grid grid-cols-1 ">
          <div class="space-y-2">
            <IngredientsEditor
                v-model="recipe.ingredient_groups"
                :available-languages="available_language!"
                :current-lang="currentLang"
            />
          </div>
          <separator orientation="horizontal" class="mb-8"/>
          <div class="space-y-2">
            <StepsEditor
                v-model="recipe.step_groups"
                :available-languages="available_language!"
                :current-lang="currentLang"
            />
          </div>
        </CardContent>
      </Card>
    </div>

    <aside class="sticky top-6 hidden xl:flex flex-col gap-4 w-60">
      <div class="bg-card border rounded-xl p-4 shadow-md space-y-4">

        <div class="space-y-2">
          <Label class="text-[10px] font-bold text-muted-foreground uppercase px-1">
            {{ t('Admin.recipe.fields.primaryLanguage') }}
          </Label>
          <Select v-model="recipe.primary_language">
            <SelectTrigger class="w-full bg-background">
              <SelectValue :placeholder="t('Admin.recipe.placeholders.selectLanguage')" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem v-for="lang in available_language" :key="lang.code" :value="lang.code">
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
                v-for="lang in available_language"
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
            {{ t('Admin.common.create') }}
          </Button>
        </div>
      </div>
    </aside>

  </div>

</template>