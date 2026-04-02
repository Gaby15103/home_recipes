<script setup lang="ts">
import {ref} from "vue"
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Textarea} from "@/components/ui/textarea"
import {Check, Edit3, Plus, Timer, Trash2, Upload, X} from "lucide-vue-next"
import type {StepCreate, StepGroupCreate} from "@/models/RecipeCreate.ts"

const props = defineProps<{
  modelValue: StepGroupCreate[],
  currentLang: string
}>()

const emit = defineEmits(['update:modelValue'])
const editingRow = ref<string | null>(null)

function getTrans(obj: any, lang: string): any {
  if (!obj.translations) obj.translations = [];
  let trans = obj.translations.find((t: any) => t.language_code === lang);
  if (!trans) {
    trans = 'steps' in obj
        ? { language_code: lang, title: "" }
        : { language_code: lang, instruction: "" };
    obj.translations.push(trans);
  }
  return trans;
}

const addGroup = () => {
  emit('update:modelValue', [...props.modelValue, {
    translations: [{ language_code: props.currentLang, title: "" }],
    position: props.modelValue.length,
    steps: []
  }]);
}

const addStep = (group: StepGroupCreate) => {
  group.steps.push({
    position: group.steps.length,
    translations: [{ language_code: props.currentLang, instruction: "" }],
    duration_minutes: null,
    image_url: null
  });
}

function onImageChange(e: Event, step: StepCreate) {
  const file = (e.target as HTMLInputElement).files?.[0];
  if (file) step.image_url = file;
}

function getImageUrl(url: any) {
  if (!url) return null;
  return url instanceof File ? URL.createObjectURL(url) : url;
}
</script>

<template>
  <div class="space-y-6 md:space-y-8">
    <div v-for="(group, gIdx) in modelValue" :key="gIdx"
         class="relative group/card border rounded-xl md:rounded-2xl p-4 md:p-6 bg-card shadow-sm transition-all">

      <Button variant="ghost" size="icon" @click="modelValue.splice(gIdx, 1)"
              class="absolute -top-2 -right-2 h-7 w-7 md:h-8 md:w-8 rounded-full bg-destructive text-white opacity-100 md:opacity-0 md:group-hover/card:opacity-100 shadow-lg z-10">
        <X class="w-3.5 h-3.5" />
      </Button>

      <div class="flex items-center gap-3 mb-4 md:mb-6">
        <Input v-model="getTrans(group, currentLang).title"
               class="flex-1 md:w-auto font-black uppercase text-[9px] md:text-[10px] tracking-[0.2em] h-8 px-4 rounded-full bg-muted border-none"
               placeholder="PREPARATION STEPS" />
        <div class="hidden md:block h-px flex-1 bg-gradient-to-r from-border to-transparent" />
      </div>

      <div class="space-y-3 md:space-y-4">
        <div v-for="(step, sIdx) in group.steps" :key="sIdx"
             class="group/row flex flex-col p-3 md:p-4 rounded-xl border border-transparent transition-all"
             :class="[editingRow === `${gIdx}-${sIdx}` ? 'bg-muted/50 border-border shadow-inner' : 'hover:bg-muted/30 border-muted/20 md:border-transparent']">

          <div class="flex items-start gap-3 md:gap-4">
            <span class="mt-0.5 font-black text-xl md:text-2xl opacity-10 shrink-0">{{ sIdx + 1 }}</span>

            <div v-if="editingRow !== `${gIdx}-${sIdx}`" class="flex-1 flex gap-3 md:gap-4 min-w-0">
              <div v-if="step.image_url" class="w-14 h-14 md:w-20 md:h-20 rounded-lg overflow-hidden border shrink-0 bg-muted">
                <img :src="getImageUrl(step.image_url)" class="object-cover w-full h-full" />
              </div>
              <div class="flex-1 min-w-0">
                <div v-if="step.duration_minutes" class="flex items-center gap-1 mb-1">
                  <span class="text-[9px] font-bold bg-secondary px-2 py-0.5 rounded-full flex items-center">
                    <Timer class="w-2.5 h-2.5 mr-1"/>{{ step.duration_minutes }}m
                  </span>
                </div>
                <p class="text-xs md:text-sm text-foreground/80 line-clamp-3 italic leading-relaxed">
                  {{ getTrans(step, currentLang).instruction || 'No instruction...' }}
                </p>
              </div>
              <div class="flex flex-col md:flex-row gap-1 shrink-0">
                <Button variant="ghost" size="icon" @click="editingRow = `${gIdx}-${sIdx}`" class="h-8 w-8 hover:bg-background"><Edit3 class="w-3.5 h-3.5" /></Button>
                <Button variant="ghost" size="icon" @click="group.steps.splice(sIdx, 1)" class="h-8 w-8 hover:text-destructive"><Trash2 class="w-3.5 h-3.5" /></Button>
              </div>
            </div>

            <div v-else class="flex-1 space-y-4">
              <div class="flex flex-col md:flex-row gap-4">
                <div class="w-full md:w-32 space-y-2 shrink-0">
                  <div class="aspect-video md:aspect-square bg-background border-2 border-dashed rounded-xl flex items-center justify-center relative overflow-hidden group/img">
                    <img v-if="step.image_url" :src="getImageUrl(step.image_url)" class="object-cover w-full h-full" />
                    <div v-else class="flex flex-col items-center gap-1 opacity-20">
                      <Upload class="w-5 h-5" />
                      <span class="text-[8px] font-black uppercase">Photo</span>
                    </div>
                    <input type="file" @change="e => onImageChange(e, step)" class="absolute inset-0 opacity-0 cursor-pointer" />
                  </div>
                  <div class="relative">
                    <Timer class="absolute left-2.5 top-2.5 w-3 h-3 text-muted-foreground opacity-40" />
                    <Input v-model.number="step.duration_minutes" type="number" placeholder="Min" class="h-8 pl-7 text-[10px] text-center font-bold" />
                  </div>
                </div>
                <div class="flex-1">
                  <Textarea v-model="getTrans(step, currentLang).instruction" rows="4"
                            class="bg-background resize-none text-sm p-4 rounded-xl min-h-[120px]"
                            placeholder="Describe this step..." />
                </div>
              </div>
              <div class="flex justify-end">
                <Button @click="editingRow = null" size="sm" class="h-9 px-6 font-bold text-[10px] uppercase tracking-widest shadow-md">
                  <Check class="w-3.5 h-3.5 mr-2" /> Done
                </Button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <Button variant="outline" class="w-full mt-4 md:mt-6 border-dashed h-10 md:h-11 rounded-xl text-xs" @click="addStep(group)">
        <Plus class="w-3.5 h-3.5 mr-2" /> Add Step
      </Button>
    </div>

    <Button variant="secondary" class="w-full h-14 md:h-16 border-2 border-dashed bg-transparent hover:bg-muted/40 rounded-xl md:rounded-2xl transition-all" @click="addGroup">
      <Plus class="w-4 h-4 mr-2 opacity-50" /> <span class="font-bold text-[9px] md:text-[11px] uppercase tracking-widest">Add Step Group</span>
    </Button>
  </div>
</template>