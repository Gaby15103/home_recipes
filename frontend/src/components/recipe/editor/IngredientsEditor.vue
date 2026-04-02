<script setup lang="ts">
import {ref} from "vue"
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Check, Edit3, GripVertical, Plus, StickyNote, Trash2, X} from "lucide-vue-next"
import IngredientUnitSelect from "@/components/recipe/forms/IngredientUnitSelect.vue"
import type {IngredientGroupCreate} from "@/models/RecipeCreate.ts"
import type {Unit} from "@/models/Recipe.ts"

const props = defineProps<{
  modelValue: IngredientGroupCreate[],
  currentLang: string,
  units: Unit[]
}>()

const emit = defineEmits(['update:modelValue'])
const editingRow = ref<string | null>(null)

function getTrans(obj: any, lang: string): any {
  // 1. Force initialize translations if it's missing or null
  if (!obj || !Array.isArray(obj.translations)) {
    obj.translations = [];
  }

  let trans = obj.translations.find((t: any) => t.language_code === lang);

  if (!trans) {
    // 2. Create the correct shape based on whether it's a Group or an Ingredient
    const isGroup = 'ingredients' in obj;

    if (isGroup) {
      trans = { language_code: lang, title: "" };
    } else {
      trans = { language_code: lang, data: "", note: "" };
    }

    obj.translations.push(trans);
  }

  return trans;
}

const addGroup = () => {
  emit('update:modelValue', [...props.modelValue, {
    translations: [{ language_code: props.currentLang, title: "" }],
    position: props.modelValue.length,
    ingredients: []
  }]);
}

const addIngredient = (group: IngredientGroupCreate) => {
  group.ingredients.push({
    translations: [{ language_code: props.currentLang, data: "", note: "" }],
    quantity: 0,
    unit_id: props.units[0]?.id || '',
    position: group.ingredients.length
  });
}
</script>

<template>
  <div class="space-y-6 md:space-y-8">
    <div v-for="(group, gIdx) in modelValue" :key="gIdx"
         class="relative group/card border rounded-xl md:rounded-2xl p-4 md:p-6 bg-card shadow-sm transition-all hover:shadow-md">

      <Button variant="ghost" size="icon" @click="modelValue.splice(gIdx, 1)"
              class="absolute -top-2 -right-2 h-7 w-7 md:h-8 md:w-8 rounded-full bg-destructive text-white hover:bg-destructive/90 opacity-100 md:opacity-0 md:group-hover/card:opacity-100 transition-opacity shadow-lg z-10">
        <X class="w-3.5 h-3.5" />
      </Button>

      <div class="flex items-center gap-3 mb-4 md:mb-6">
        <Input v-model="getTrans(group, currentLang).title"
               class="flex-1 md:w-auto font-black uppercase text-[9px] md:text-[10px] tracking-[0.2em] h-8 px-4 rounded-full bg-muted border-none focus-visible:ring-1 focus-visible:ring-primary"
               placeholder="GROUP NAME" />
        <div class="hidden md:block h-px flex-1 bg-gradient-to-r from-border to-transparent" />
      </div>

      <div class="space-y-3 md:space-y-4">
        <div v-for="(ing, iIdx) in group.ingredients" :key="iIdx"
             class="group/row flex flex-col p-3 md:p-4 rounded-xl border border-transparent transition-all"
             :class="[editingRow === `${gIdx}-${iIdx}` ? 'bg-muted/50 border-border shadow-inner' : 'hover:bg-muted/30 border-muted/20 md:border-transparent']">

          <div class="flex items-start gap-2 md:gap-4">
            <button class="mt-2 text-muted-foreground/30 hover:text-foreground shrink-0 hidden md:block">
              <GripVertical class="w-4 h-4" />
            </button>

            <div v-if="editingRow !== `${gIdx}-${iIdx}`" class="flex-1 flex items-start justify-between gap-2">
              <div class="flex flex-col md:flex-row md:items-center gap-1 md:gap-4 flex-1 min-w-0">
                <div class="min-w-[60px] md:min-w-[80px] font-mono text-xs md:text-sm shrink-0">
                  <span class="font-bold text-foreground">{{ ing.quantity || '0' }}</span>
                  <span class="ml-1 text-[9px] md:text-[10px] text-muted-foreground uppercase font-bold">
                    {{ units.find(u => u.id == ing.unit_id)?.symbol || '' }}
                  </span>
                </div>
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-semibold truncate">{{ getTrans(ing, currentLang).data || 'Ingredient' }}</p>
                  <p v-if="getTrans(ing, currentLang).note" class="text-[10px] md:text-[11px] text-muted-foreground italic flex items-center gap-1 mt-0.5">
                    <StickyNote class="w-3 h-3 opacity-50 shrink-0" /> {{ getTrans(ing, currentLang).note }}
                  </p>
                </div>
              </div>
              <div class="flex items-center gap-1 shrink-0">
                <Button variant="ghost" size="icon" @click="editingRow = `${gIdx}-${iIdx}`" class="h-8 w-8 hover:bg-background"><Edit3 class="w-3.5 h-3.5" /></Button>
                <Button variant="ghost" size="icon" @click="group.ingredients.splice(iIdx, 1)" class="h-8 w-8 hover:text-destructive"><Trash2 class="w-3.5 h-3.5" /></Button>
              </div>
            </div>

            <div v-else class="flex-1 space-y-4">
              <div class="grid grid-cols-2 md:flex md:flex-wrap items-end gap-3">
                <div class="space-y-1.5">
                  <label class="text-[8px] font-black uppercase tracking-tighter text-muted-foreground ml-1">Qty</label>
                  <Input v-model.number="ing.quantity" type="number" class="h-10 bg-background font-bold text-center" />
                </div>
                <div class="space-y-1.5">
                  <label class="text-[8px] font-black uppercase tracking-tighter text-muted-foreground ml-1">Unit</label>
                  <IngredientUnitSelect v-model="ing.unit_id" class="h-10 bg-background w-full" />
                </div>
                <div class="col-span-2 md:flex-1 space-y-1.5">
                  <label class="text-[8px] font-black uppercase tracking-tighter text-muted-foreground ml-1">Ingredient Name</label>
                  <Input v-model="getTrans(ing, currentLang).data" class="h-10 bg-background font-bold text-sm" placeholder="Butter, Sugar..." />
                </div>
              </div>

              <div class="relative space-y-1.5">
                <label class="text-[8px] font-black uppercase tracking-tighter text-muted-foreground ml-1">Note / Action</label>
                <div class="relative">
                  <StickyNote class="absolute left-3 top-3 w-4 h-4 text-muted-foreground opacity-40" />
                  <Input v-model="getTrans(ing, currentLang).note" class="h-10 pl-9 bg-background/80 text-xs italic" placeholder="e.g. melted" />
                </div>
              </div>

              <div class="flex justify-end pt-1">
                <Button @click="editingRow = null" size="sm" class="h-8 md:h-9 px-4 md:px-6 font-bold text-[10px] uppercase tracking-widest shadow-md">
                  <Check class="w-3.5 h-3.5 mr-2" /> Done
                </Button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <Button variant="outline" class="w-full mt-4 md:mt-6 border-dashed h-10 md:h-12 rounded-xl text-xs text-muted-foreground" @click="addIngredient(group)">
        <Plus class="w-3.5 h-3.5 mr-2" /> New Ingredient
      </Button>
    </div>

    <Button variant="secondary" class="w-full h-14 md:h-20 border-2 border-dashed bg-transparent hover:bg-muted/40 rounded-xl md:rounded-2xl transition-all" @click="addGroup">
      <Plus class="w-4 h-4 mr-2 opacity-50" /> <span class="font-bold text-[9px] md:text-[11px] uppercase tracking-[0.2em]">Add Ingredient Group</span>
    </Button>
  </div>
</template>