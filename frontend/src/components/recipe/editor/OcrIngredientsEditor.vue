<script setup lang="ts">
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Plus, Trash2, GripVertical } from "lucide-vue-next"
import IngredientUnitSelect from "@/components/recipe/forms/IngredientUnitSelect.vue"

const props = defineProps<{ modelValue: any[], currentLang: string }>()
const emit = defineEmits(["update:modelValue"])

const getTrans = (obj: any, lang: string) => {
  let trans = obj.translations.find((t: any) => t.language_code === lang)
  if (!trans) {
    trans = { language_code: lang, name: "" }
    obj.translations.push(trans)
  }
  return trans
}

const removeIngredient = (gIdx: number, iIdx: number) => {
  props.modelValue[gIdx].ingredients.splice(iIdx, 1)
}
</script>

<template>
  <div class="space-y-10">
    <div v-for="(group, gIdx) in modelValue" :key="gIdx" class="space-y-4">
      <div class="flex items-center gap-4">
        <Input v-model="getTrans(group, currentLang).name" class="w-auto min-w-[200px] text-center font-black uppercase tracking-widest text-[10px] border-none bg-slate-100 dark:bg-slate-800 rounded-full h-8" placeholder="Group Name" />
        <div class="h-px flex-1 bg-slate-200 dark:bg-slate-800" />
      </div>

      <div class="space-y-2">
        <div v-for="(ing, iIdx) in group.ingredients" :key="iIdx"
             class="flex items-center gap-3 p-2 rounded-xl border border-transparent hover:border-slate-200 dark:hover:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-900/50 transition-all group/row">

          <GripVertical class="w-4 h-4 text-slate-300 opacity-0 group-hover/row:opacity-100" />

          <div class="flex gap-1 w-44">
            <Input v-model="ing.quantity" class="w-16 h-9 text-xs font-black text-center bg-white dark:bg-slate-950 border-slate-200 dark:border-slate-800" placeholder="Qty" />
            <IngredientUnitSelect v-model="ing.unit_id" class="flex-1 h-9 text-[10px] font-bold" />
          </div>

          <Input v-model="getTrans(ing, currentLang).name" class="flex-1 h-9 text-sm font-medium border-none bg-transparent focus:bg-white dark:focus:bg-slate-950 transition-colors" placeholder="Ingredient name..." />

          <Button variant="ghost" size="icon" @click="removeIngredient(gIdx, iIdx)" class="h-8 w-8 text-slate-300 hover:text-destructive opacity-0 group-hover/row:opacity-100">
            <Trash2 class="w-4 h-4" />
          </Button>
        </div>
      </div>

      <Button variant="ghost" class="w-full border-dashed border-2 text-slate-400 hover:text-primary rounded-xl" @click="group.ingredients.push({ translations: [], quantity: '', unit_id: null })">
        <Plus class="w-4 h-4 mr-2" /> Add Ingredient
      </Button>
    </div>
  </div>
</template>