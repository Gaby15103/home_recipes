<script setup lang="ts">
import { ref } from "vue"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Plus, Trash2, GripVertical, Check, Edit3, X, StickyNote } from "lucide-vue-next"
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
      trans = { language_code: lang, data: "", note: "" };
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
    <div v-for="(group, gIdx) in modelValue" :key="gIdx" class="relative group/card border rounded-2xl p-6 bg-card shadow-sm transition-all hover:shadow-md">

      <Button
          variant="ghost" size="icon" @click="removeGroup(gIdx)"
          class="absolute -top-3 -right-3 h-8 w-8 rounded-full bg-destructive text-white hover:bg-destructive/90 opacity-0 group-hover/card:opacity-100 transition-opacity shadow-lg"
      >
        <X class="w-4 h-4" />
      </Button>

      <div class="flex items-center gap-4 mb-6">
        <Input
            v-model="getTrans(group, currentLang).title"
            class="w-auto font-bold uppercase text-[10px] tracking-[0.2em] h-8 px-4 rounded-full bg-muted border-none focus-visible:ring-1 focus-visible:ring-primary"
            placeholder="GROUP NAME (EX: SAUCE)"
        />
        <div class="h-px flex-1 bg-gradient-to-r from-border to-transparent" />
      </div>

      <div class="space-y-3">
        <div v-for="(ing, iIdx) in group.ingredients" :key="iIdx"
             class="group/row flex flex-col gap-2 p-4 rounded-xl border border-transparent transition-all"
             :class="[editingRow === `${gIdx}-${iIdx}` ? 'bg-muted/50 border-border shadow-inner' : 'hover:bg-muted/30']">

          <div class="flex items-start gap-3">
            <button class="mt-2 cursor-grab active:cursor-grabbing text-muted-foreground/30 hover:text-foreground">
              <GripVertical class="w-4 h-4" />
            </button>

            <div v-if="editingRow !== `${gIdx}-${iIdx}`" class="flex-1 flex items-start gap-4">
              <div class="min-w-[70px] pt-1 font-mono text-sm">
                <span class="font-bold text-foreground">{{ ing.quantity || '—' }}</span>
                <span class="ml-1 text-[10px] text-muted-foreground uppercase font-bold">{{
                    units.find(u => u.id == ing.unit_id)?.symbol || ''
                  }}</span>
              </div>

              <div class="flex-1 flex flex-col gap-0.5">
                <span class="text-sm font-semibold text-foreground/90">{{ getTrans(ing, currentLang).data || 'Unnamed ingredient' }}</span>
                <span v-if="getTrans(ing, currentLang).note" class="text-[11px] text-muted-foreground italic flex items-center gap-1">
                  <StickyNote class="w-3 h-3 opacity-50" /> {{ getTrans(ing, currentLang).note }}
                </span>
              </div>

              <div class="opacity-0 group-hover/row:opacity-100 flex items-center gap-1 transition-opacity">
                <Button variant="ghost" size="icon" @click="editingRow = `${gIdx}-${iIdx}`" class="h-8 w-8 rounded-lg hover:bg-background"><Edit3 class="w-3.5 h-3.5" /></Button>
                <Button variant="ghost" size="icon" @click="group.ingredients.splice(iIdx, 1)" class="h-8 w-8 hover:text-destructive hover:bg-destructive/10"><Trash2 class="w-3.5 h-3.5" /></Button>
              </div>
            </div>

            <div v-else class="flex-1 space-y-4 py-1">
              <div v-if="getOcrReference(gIdx, iIdx)" class="p-3 rounded-lg bg-background/50 border border-dashed text-[11px] font-mono text-muted-foreground">
                <span class="text-[9px] font-black uppercase tracking-widest block mb-1 opacity-50">Original OCR:</span>
                "{{ getOcrReference(gIdx, iIdx)!.original_line }}"
              </div>

              <div class="grid grid-cols-1 md:grid-cols-12 gap-3">
                <div class="md:col-span-3 flex gap-1 bg-background p-1 rounded-lg border shadow-sm">
                  <Input v-model.number="ing.quantity" type="number" class="w-full h-9 border-none font-bold text-center focus-visible:ring-0" />
                  <div class="w-full border-l pl-1">
                    <IngredientUnitSelect v-model="ing.unit_id" class="h-9 border-none shadow-none focus:ring-0" />
                  </div>
                </div>

                <div class="md:col-span-9 flex flex-col gap-3">
                  <Input v-model="getTrans(ing, currentLang).data"
                         class="h-11 px-4 bg-background border rounded-lg font-bold text-sm shadow-sm"
                         placeholder="Ingredient Name (ex: Butter)" />

                  <div class="relative">
                    <StickyNote class="absolute left-3 top-3 w-4 h-4 text-muted-foreground opacity-40" />
                    <Input v-model="getTrans(ing, currentLang).note"
                           class="h-10 pl-9 pr-4 bg-background/80 border rounded-lg text-xs italic shadow-sm"
                           placeholder="Add a note (ex: melted, at room temperature...)" />
                  </div>
                </div>
              </div>

              <div class="flex justify-end">
                <Button @click="editingRow = null" size="sm" class="h-9 px-6 font-bold text-[11px] uppercase tracking-widest shadow-lg">
                  <Check class="w-3.5 h-3.5 mr-2" /> Done
                </Button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <Button variant="outline" class="w-full mt-6 border-dashed text-muted-foreground hover:text-foreground hover:bg-muted/50 h-11 rounded-xl"
              @click="group.ingredients.push({ translations: [], quantity: 0, unit_id: '', position: group.ingredients.length })">
        <Plus class="w-4 h-4 mr-2" /> New Ingredient
      </Button>
    </div>

    <Button variant="secondary" class="w-full h-16 border-2 border-dashed bg-transparent hover:bg-muted/40 rounded-2xl transition-all" @click="addGroup">
      <Plus class="w-5 h-5 mr-2 opacity-50" />
      <span class="font-bold text-[11px] uppercase tracking-[0.2em]">Add Ingredient Group</span>
    </Button>
  </div>
</template>