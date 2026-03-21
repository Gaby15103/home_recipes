<script setup lang="ts">
import { ref } from "vue"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Plus, Trash2, GripVertical, Check, Edit3, X } from "lucide-vue-next"
import { Badge } from "@/components/ui/badge"
import IngredientUnitSelect from "@/components/recipe/forms/IngredientUnitSelect.vue"
import type {
  IngredientGroupCreate,
  IngredientCreate,
  IngredientGroupTranslationCreate,
  IngredientTranslationCreate
} from "@/models/RecipeCreate.ts";
import type { OcrIngredientGroup } from "@/models/OcrResult.ts";
import type {Unit} from "@/models/Recipe.ts";

const props = defineProps<{
  modelValue: IngredientGroupCreate[],
  currentLang: string,
  originalOcrGroups?: OcrIngredientGroup[],
  units: Unit[]
}>()

const emit = defineEmits(['update:modelValue'])
const editingRow = ref<string | null>(null)

function getTrans(obj: IngredientGroupCreate, lang: string): IngredientGroupTranslationCreate;
function getTrans(obj: IngredientCreate, lang: string): IngredientTranslationCreate;
function getTrans(obj: any, lang: string): any {
  if (!obj.translations) obj.translations = [];
  let trans = obj.translations.find((t: any) => t.language_code === lang);

  if (!trans) {
    if ('ingredients' in obj) {
      trans = { language_code: lang, title: "" };
    } else {
      trans = { language_code: lang, data: "", note: null };
    }
    obj.translations.push(trans);
  }
  return trans;
}

const getOcrReference = (gIdx: number, iIdx: number) => {
  return props.originalOcrGroups?.[gIdx]?.ingredients?.[iIdx]
}

const addGroup = () => {
  const newValue = [...props.modelValue, {
    translations: [{ language_code: props.currentLang, title: "" }],
    position: props.modelValue.length,
    ingredients: []
  }];
  emit('update:modelValue', newValue);
}

const removeGroup = (idx: number) => {
  const newValue = [...props.modelValue];
  newValue.splice(idx, 1);
  emit('update:modelValue', newValue);
}
</script>

<template>
  <div class="space-y-8">
    <div v-for="(group, gIdx) in modelValue" :key="gIdx" class="relative group/card border rounded-xl p-6 bg-card shadow-sm">

      <Button
          variant="ghost" size="icon" @click="removeGroup(gIdx)"
          class="absolute -top-3 -right-3 h-8 w-8 rounded-full bg-destructive text-destructive-foreground hover:bg-destructive/90 opacity-0 group-hover/card:opacity-100 transition-opacity shadow-sm"
      >
        <X class="w-4 h-4" />
      </Button>

      <div class="flex items-center gap-4 mb-6">
        <Input
            v-model="getTrans(group, currentLang).title"
            class="w-auto font-bold uppercase text-[10px] tracking-widest h-8 px-3 rounded-full bg-secondary border-none"
            placeholder="Group Name"
        />
        <div class="h-px flex-1 bg-border" />
      </div>

      <div class="space-y-2">
        <div v-for="(ing, iIdx) in group.ingredients" :key="iIdx"
             class="group/row flex flex-col gap-2 p-3 rounded-lg border border-transparent transition-colors"
             :class="[editingRow === `${gIdx}-${iIdx}` ? 'bg-muted border-border' : 'hover:bg-muted/50']">

          <div class="flex items-center gap-3">
            <button class="cursor-grab active:cursor-grabbing text-muted-foreground/40 hover:text-foreground">
              <GripVertical class="w-4 h-4" />
            </button>

            <div v-if="editingRow !== `${gIdx}-${iIdx}`" class="flex-1 flex items-center gap-4">
              <div class="min-w-[80px] font-mono text-sm">
                <span class="font-bold text-foreground">{{ ing.quantity || '—' }}</span>
                <span class="ml-1 text-[10px] text-muted-foreground uppercase">{{
                    units.find(u => u.id == ing.unit_id)?.symbol || ''
                  }}</span>
              </div>
              <span class="text-sm font-medium flex-1">{{ getTrans(ing, currentLang).data || 'Unnamed ingredient' }}</span>

              <div class="opacity-0 group-hover/row:opacity-100 flex items-center gap-1 transition-opacity">
                <Button variant="ghost" size="icon" @click="editingRow = `${gIdx}-${iIdx}`" class="h-8 w-8 rounded-md"><Edit3 class="w-3.5 h-3.5" /></Button>
                <Button variant="ghost" size="icon" @click="group.ingredients.splice(iIdx, 1)" class="h-8 w-8 hover:text-destructive"><Trash2 class="w-3.5 h-3.5" /></Button>
              </div>
            </div>

            <div v-else class="flex-1 space-y-4 py-2">
              <div v-if="getOcrReference(gIdx, iIdx)" class="space-y-3 p-4 rounded-md bg-background border shadow-sm">
                <div class="flex items-center justify-between">
                  <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-widest flex items-center gap-2">
                    <span class="w-1.5 h-1.5 rounded-full bg-zinc-400" /> Raw OCR Detection
                  </span>
                </div>
                <p class="text-[11px] font-mono bg-muted p-2 rounded text-zinc-600 dark:text-zinc-400">
                  "{{ getOcrReference(gIdx, iIdx)!.original_line }}"
                </p>
              </div>

              <div class="flex flex-wrap gap-3">
                <div class="flex gap-1 bg-background p-1 rounded-md border shadow-sm">
                  <Input v-model.number="ing.quantity" type="number" class="w-16 h-9 border-none font-bold text-center focus-visible:ring-0" />
                  <div class="w-32"><IngredientUnitSelect v-model="ing.unit_id" class="h-9 border-none" /></div>
                </div>
                <Input v-model="getTrans(ing, currentLang).data" class="flex-1 h-11 px-3 bg-background border rounded-md font-medium" placeholder="Ingredient Name" />
                <Button @click="editingRow = null" class="h-11 px-4 shadow-sm bg-zinc-900 text-zinc-50 hover:bg-zinc-800 dark:bg-zinc-50 dark:text-zinc-900"><Check class="w-4 h-4 mr-2" /> Done</Button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <Button variant="outline" class="w-full mt-4 border-dashed text-muted-foreground hover:text-foreground h-12"
              @click="group.ingredients.push({ translations: [], quantity: 0, unit_id: '', position: group.ingredients.length })">
        <Plus class="w-4 h-4 mr-2" /> Add Ingredient
      </Button>
    </div>

    <Button variant="secondary" class="w-full h-14 border-2 border-dashed bg-transparent hover:bg-muted" @click="addGroup">
      <Plus class="w-5 h-5 mr-2" /> Add Ingredient Group
    </Button>
  </div>
</template>