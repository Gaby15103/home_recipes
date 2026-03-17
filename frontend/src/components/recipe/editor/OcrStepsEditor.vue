<script setup lang="ts">
import {ref} from "vue"
import {Button} from "@/components/ui/button"
import {Textarea} from "@/components/ui/textarea"
import {Input} from "@/components/ui/input"
import {Check, Edit3, Plus, Trash2} from "lucide-vue-next"

const props = defineProps<{ modelValue: any[], currentLang: string, originalOcrGroups?: any[] }>()
const editingStep = ref<string | null>(null)

const getTrans = (obj: any, lang: string) => {
  let trans = obj.translations.find((t: any) => t.language_code === lang)
  if (!trans) {
    trans = { language_code: lang, text: "", title: "" }
    obj.translations.push(trans)
  }
  return trans
}
</script>

<template>
  <div class="space-y-10">
    <div v-for="(group, gIdx) in modelValue" :key="gIdx" class="border rounded-xl p-6 bg-card">
      <div class="flex items-center gap-4 mb-8">
        <Input v-model="getTrans(group, currentLang).title" class="w-auto font-bold uppercase text-[10px] tracking-widest h-8 px-4 rounded-full bg-muted border-none" />
        <div class="h-px flex-1 bg-border" />
      </div>

      <div class="space-y-6">
        <div v-for="(step, sIdx) in group.steps" :key="sIdx" class="relative group/step flex gap-4">
          <div class="flex flex-col items-center">
            <div class="w-8 h-8 rounded-full border bg-background flex items-center justify-center text-[10px] font-bold">
              {{ sIdx + 1 }}
            </div>
            <div class="flex-1 w-px bg-border my-2" />
          </div>

          <div class="flex-1 space-y-3 pb-6">
            <div v-if="editingStep !== `${gIdx}-${sIdx}`" class="flex items-start justify-between gap-4">
              <p class="text-sm leading-relaxed text-muted-foreground">{{ getTrans(step, currentLang).text || 'No instructions...' }}</p>
              <div class="flex gap-1 opacity-0 group-hover/step:opacity-100 transition-opacity">
                <Button variant="ghost" size="icon" @click="editingStep = `${gIdx}-${sIdx}`" class="h-8 w-8"><Edit3 class="w-3.5 h-3.5" /></Button>
                <Button variant="ghost" size="icon" @click="group.steps.splice(sIdx, 1)" class="h-8 w-8 hover:text-destructive"><Trash2 class="w-3.5 h-3.5" /></Button>
              </div>
            </div>

            <div v-else class="space-y-4 animate-in fade-in slide-in-from-top-1 duration-200">
              <Textarea v-model="getTrans(step, currentLang).text" class="min-h-[100px] text-sm bg-muted/30 focus-visible:ring-1" />
              <div class="flex justify-end">
                <Button size="sm" @click="editingStep = null"><Check class="w-3.5 h-3.5 mr-2" /> Save Step</Button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <Button variant="outline" class="w-full border-dashed h-12" @click="group.steps.push({ translations: [], position: group.steps.length })">
        <Plus class="w-4 h-4 mr-2" /> Add Step
      </Button>
    </div>
  </div>
</template>