<script setup lang="ts">
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Input } from "@/components/ui/input"
import { Plus, Trash2 } from "lucide-vue-next"

const props = defineProps<{ modelValue: any[], currentLang: string }>()

const getTrans = (obj: any, lang: string) => {
  let trans = obj.translations.find((t: any) => t.language_code === lang)
  if (!trans) {
    trans = { language_code: lang, text: "", title: "" }
    obj.translations.push(trans)
  }
  return trans
}

const removeStep = (gIdx: number, sIdx: number) => {
  props.modelValue[gIdx].steps.splice(sIdx, 1)
}
</script>

<template>
  <div class="space-y-10">
    <div v-for="(group, gIdx) in modelValue" :key="gIdx" class="space-y-6">
      <div class="flex items-center gap-4">
        <Input v-model="getTrans(group, currentLang).title" class="w-auto min-w-[200px] text-center font-black uppercase tracking-widest text-[10px] border-none bg-slate-100 dark:bg-slate-800 rounded-full h-8" placeholder="Step Group (e.g. Preparation)" />
        <div class="h-px flex-1 bg-slate-200 dark:bg-slate-800" />
      </div>

      <div class="space-y-6">
        <div v-for="(step, sIdx) in group.steps" :key="sIdx" class="relative group/step flex gap-4">
          <div class="flex flex-col items-center">
            <div class="w-8 h-8 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center text-[10px] font-black text-slate-400 group-hover/step:bg-primary group-hover/step:text-white transition-colors">
              {{ sIdx + 1 }}
            </div>
            <div class="flex-1 w-px bg-slate-100 dark:bg-slate-800 my-2" />
          </div>

          <div class="flex-1 space-y-2 pb-6">
            <div class="flex items-start gap-4">
              <Textarea
                  v-model="getTrans(step, currentLang).text"
                  class="flex-1 min-h-[80px] text-sm leading-relaxed border-none bg-slate-50 dark:bg-slate-900/50 focus:bg-white dark:focus:bg-slate-950 resize-none transition-all p-4 rounded-2xl"
                  placeholder="Write step instruction..."
              />
              <Button variant="ghost" size="icon" @click="removeStep(gIdx, sIdx)" class="text-slate-300 hover:text-destructive opacity-0 group-hover/step:opacity-100 transition-opacity">
                <Trash2 class="w-4 h-4" />
              </Button>
            </div>
          </div>
        </div>
      </div>

      <Button variant="ghost" class="w-full border-dashed border-2 text-slate-400 hover:text-primary rounded-xl" @click="group.steps.push({ translations: [], position: group.steps.length })">
        <Plus class="w-4 h-4 mr-2" /> Add Step
      </Button>
    </div>
  </div>
</template>