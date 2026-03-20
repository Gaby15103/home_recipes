<script setup lang="ts">
import { ref } from "vue"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Input } from "@/components/ui/input"
import { Check, Edit3, Plus, Trash2, X } from "lucide-vue-next"
import type {
  StepGroupCreate,
  StepCreate,
  StepTranslationCreate,
  StepGroupTranslationCreate
} from "@/models/RecipeCreate.ts"

const props = defineProps<{ modelValue: StepGroupCreate[], currentLang: string }>()
const emit = defineEmits(['update:modelValue'])
const editingStep = ref<string | null>(null)

function getTrans(obj: StepGroupCreate, lang: string): StepGroupTranslationCreate;
function getTrans(obj: StepCreate, lang: string): StepTranslationCreate;
function getTrans(obj: any, lang: string): any {
  // Handle StepGroup (Single Object)
  if (obj.translations && !Array.isArray(obj.translations)) {
    return obj.translations;
  }
  // Handle Step (Array)
  if (!obj.translations) obj.translations = [];
  let trans = obj.translations.find((t: any) => t.language_code === lang);
  if (!trans) {
    trans = { language_code: lang, instruction: "" };
    obj.translations.push(trans);
  }
  return trans;
}

const addGroup = () => {
  const newValue = [...props.modelValue, {
    position: props.modelValue.length,
    translations: { language_code: props.currentLang, title: "" },
    steps: []
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
  <div class="space-y-10">
    <div v-for="(group, gIdx) in modelValue" :key="gIdx" class="relative group/card border rounded-xl p-6 bg-card">
      <Button variant="ghost" size="icon" @click="removeGroup(gIdx)"
              class="absolute -top-3 -right-3 h-8 w-8 rounded-full bg-destructive text-white opacity-0 group-hover/card:opacity-100 transition-opacity shadow-sm">
        <X class="w-4 h-4" />
      </Button>

      <div class="flex items-center gap-4 mb-8">
        <Input v-model="getTrans(group, currentLang).title"
               class="w-auto font-bold uppercase text-[10px] tracking-widest h-8 px-4 rounded-full bg-muted border-none"
               placeholder="STEP GROUP TITLE"/>
        <div class="h-px flex-1 bg-border"/>
      </div>

      <div class="space-y-6">
        <div v-for="(step, sIdx) in group.steps" :key="sIdx" class="relative group/step flex gap-4">
          <div class="flex flex-col items-center">
            <div class="w-8 h-8 rounded-full border bg-background flex items-center justify-center text-[10px] font-bold">{{ sIdx + 1 }}</div>
            <div v-if="sIdx !== group.steps.length - 1" class="flex-1 w-px bg-border my-2"/>
          </div>
          <div class="flex-1 space-y-3 pb-6">
            <div v-if="editingStep !== `${gIdx}-${sIdx}`" class="flex items-start justify-between gap-4">
              <p class="text-sm text-muted-foreground whitespace-pre-wrap">{{ getTrans(step, currentLang).instruction || 'No instructions...' }}</p>
              <div class="flex gap-1 opacity-0 group-hover/step:opacity-100 transition-opacity">
                <Button variant="ghost" size="icon" @click="editingStep = `${gIdx}-${sIdx}`"><Edit3 class="w-3.5 h-3.5"/></Button>
                <Button variant="ghost" size="icon" @click="group.steps.splice(sIdx, 1)" class="hover:text-destructive"><Trash2 class="w-3.5 h-3.5"/></Button>
              </div>
            </div>
            <div v-else class="space-y-4">
              <Textarea v-model="getTrans(step, currentLang).instruction" class="min-h-[100px]" />
              <div class="flex justify-end"><Button size="sm" @click="editingStep = null"><Check class="w-3.5 h-3.5 mr-2"/>Save</Button></div>
            </div>
          </div>
        </div>
      </div>

      <Button variant="outline" class="w-full mt-4 border-dashed h-12"
              @click="group.steps.push({ translations: [], position: group.steps.length, image_url: null, duration_minutes: null })">
        <Plus class="w-4 h-4 mr-2"/> Add Step
      </Button>
    </div>

    <Button variant="secondary" class="w-full h-14 border-2 border-dashed bg-transparent hover:bg-muted" @click="addGroup">
      <Plus class="w-5 h-5 mr-2" /> Add Step Group
    </Button>
  </div>
</template>