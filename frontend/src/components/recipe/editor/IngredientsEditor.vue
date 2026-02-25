<script setup lang="ts">
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Label } from "@/components/ui/label"
import type { IngredientGroupCreate, IngredientCreate } from "@/models/RecipeCreate.ts";
import IngredientUnitSelect from "@/components/recipe/forms/IngredientUnitSelect.vue";
import { useI18n } from "vue-i18n";
import type { Language } from "@/models/Language.ts";

const { t } = useI18n()

const props = defineProps<{
  modelValue: IngredientGroupCreate[],
  currentLang: string,
  availableLanguages: Language[]
}>()

const emit = defineEmits(["update:modelValue"])

function getIngTrans(ingredient: IngredientCreate, langCode: string) {
  let trans = ingredient.translations.find(t => t.language_code === langCode);
  if (!trans) {
    trans = { language_code: langCode, data: "" };
    ingredient.translations.push(trans);
  }
  return trans;
}
// We need to store the translations for ALL languages locally
// because your model only supports one at a time for the final payload.
// Alternatively, we ensure the currentLang is always synced.

function addGroup() {
  const newGroup = {
    // FIXED: Must be an array [] so that .find() works later
    translations: [
      { language_code: props.currentLang, title: "" }
    ] as any,
    position: props.modelValue.length,
    ingredients: [],
  };

  emit("update:modelValue", [...props.modelValue, newGroup]);
}

function addIngredient(group: IngredientGroupCreate) {
  group.ingredients.push({
    // Initialize with an empty array so we can store multiple languages
    translations: [],
    quantity: 0,
    unit_id: "",
    note: [],
    position: group.ingredients.length
  });
  emit("update:modelValue", [...props.modelValue]);
}
function getGroupTrans(group: IngredientGroupCreate, langCode: string) {
  let trans = group.translations.find(t => t.language_code === langCode);
  if (!trans) {
    trans = { language_code: langCode, title: "" };
    group.translations.push(trans);
  }
  return trans;
}

// Logic to handle the note_translations (which IS an array in your model)
function getNote(ing: IngredientCreate, lang: string) {
  if (!ing.note) ing.note = [];
  let note = ing.note.find(n => n.language_code === lang);
  if (!note) {
    note = { language_code: lang, note: "" };
    ing.note.push(note);
  }
  return note;
}

// CRITICAL: This ensures that when the user types,
// the object's language_code matches the active tab.
function syncLang(obj: any) {
  if (obj.translations) {
    obj.translations.language_code = props.currentLang;
  }
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex justify-between items-center">
      <div class="flex flex-col">
        <h2 class="text-xl font-semibold">{{ t('Admin.ingredients.title') }}</h2>
        <span class="text-xs text-primary font-bold uppercase">{{ currentLang }}</span>
      </div>
      <Button type="button" size="sm" @click="addGroup">{{ t('Admin.ingredients.addGroup') }}</Button>
    </div>

    <div v-for="(group, gIdx) in modelValue" :key="gIdx" class="border rounded p-4 space-y-4">
      <div class="flex gap-4 items-end">
        <div class="flex-1">
          <Label class="text-xs">{{ t('Admin.ingredients.groupTitle') }}</Label>
          <Input
              v-model="getGroupTrans(group,currentLang).title"
              @input="syncLang(group)"
              :placeholder="`Title in ${currentLang}`"
          />
        </div>
        <Button type="button" variant="destructive" size="sm" @click="modelValue.splice(gIdx, 1); emit('update:modelValue', [...modelValue])">
          ✕
        </Button>
      </div>

      <div v-for="(ing, iIdx) in group.ingredients" :key="iIdx" class="grid grid-cols-12 gap-2 p-2 border rounded bg-muted/20">

        <div class="col-span-2">
          <Label class="text-xs">{{ t('Admin.ingredients.quantity') }}</Label>
          <Input type="number" v-model.number="ing.quantity" />
        </div>

        <div class="col-span-3">
          <Label class="text-xs">{{ t('Admin.ingredients.unit') }}</Label>
          <IngredientUnitSelect v-model="ing.unit_id" />
        </div>

        <div class="col-span-5">
          <Label class="text-xs">{{ t('Admin.ingredients.name') }}</Label>
          <Input
              v-model="getIngTrans(ing, currentLang).data"
              @input="syncLang(ing)"
              :placeholder="`Name in ${currentLang}`"
          />
        </div>
        <div class="col-span-1"/>
        <div class="col-span-1 ms-auto flex items-end">
          <Button type="button" variant="destructive" size="sm" class=" w-full" @click="group.ingredients.splice(iIdx, 1); emit('update:modelValue', [...modelValue])">✕</Button>
        </div>

        <div class="col-span-12">
          <Label class="text-xs">{{ t('Admin.ingredients.note') }} ({{ currentLang }})</Label>
          <Textarea v-model="getNote(ing, currentLang).note" rows="1" />
        </div>
      </div>

      <Button type="button" variant="outline" class="w-full border-dashed" @click="addIngredient(group)">
        + {{ t('Admin.ingredients.addIngredient') }}
      </Button>
    </div>
  </div>
</template>