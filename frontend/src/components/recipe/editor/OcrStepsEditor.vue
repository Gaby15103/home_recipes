<script setup lang="ts">
import {ref} from "vue"
import {Button} from "@/components/ui/button"
import {Textarea} from "@/components/ui/textarea"
import {Input} from "@/components/ui/input"
import {Check, Clock, Edit3, Plus, Trash2, Upload, X} from "lucide-vue-next"
import type {
  StepCreate,
  StepGroupCreate,
  StepGroupTranslationCreate,
  StepTranslationCreate
} from "@/models/RecipeCreate.ts"

const props = defineProps<{ modelValue: StepGroupCreate[], currentLang: string }>()
const emit = defineEmits(['update:modelValue'])
const editingStep = ref<string | null>(null)

function getTrans(obj: StepGroupCreate, lang: string): StepGroupTranslationCreate;
function getTrans(obj: StepCreate, lang: string): StepTranslationCreate;
function getTrans(obj: any, lang: string): any {
  if (!obj.translations) obj.translations = [];

  // Gestion spécifique pour StepGroup (qui peut être un objet ou un array selon ton implémentation)
  if (!Array.isArray(obj.translations)) return obj.translations;

  let trans = obj.translations.find((t: any) => t.language_code === lang);
  if (!trans) {
    trans = { language_code: lang, instruction: "" };
    obj.translations.push(trans);
  }
  return trans;
}

function onStepImageChange(e: Event, step: StepCreate) {
  const file = (e.target as HTMLInputElement).files?.[0] ?? null;
  step.image_url = file; // Stocke le File pour le multipart

  // Création d'une URL locale pour la prévisualisation immédiate
  if (file) {
    (step as any).preview_url = URL.createObjectURL(file);
  }
}

const addGroup = () => {
  const newValue = [...props.modelValue, {
    position: props.modelValue.length,
    translations: [{ language_code: props.currentLang, title: "" }],
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
  <div class="space-y-12">
    <div v-for="(group, gIdx) in modelValue" :key="gIdx" class="relative group/card border rounded-2xl p-8 bg-card shadow-sm transition-all hover:shadow-md">

      <Button variant="ghost" size="icon" @click="removeGroup(gIdx)"
              class="absolute -top-3 -right-3 h-8 w-8 rounded-full bg-destructive text-white opacity-0 group-hover/card:opacity-100 transition-opacity shadow-md">
        <X class="w-4 h-4" />
      </Button>

      <div class="flex items-center gap-4 mb-10">
        <Input v-model="getTrans(group, currentLang).title"
               class="w-auto font-bold uppercase text-[10px] tracking-[0.2em] h-8 px-5 rounded-full bg-muted border-none focus-visible:ring-1 focus-visible:ring-primary"
               placeholder="STEP GROUP (E.G. PREPARATION)"/>
        <div class="h-px flex-1 bg-gradient-to-r from-border to-transparent"/>
      </div>

      <div class="space-y-10">
        <div v-for="(step, sIdx) in group.steps" :key="sIdx"
             class="relative group/step flex gap-6 p-4 rounded-xl transition-colors hover:bg-muted/20">

          <div class="flex flex-col items-center">
            <div class="w-10 h-10 rounded-full border-2 bg-background flex items-center justify-center text-xs font-black shadow-sm group-hover/step:border-primary transition-colors">
              {{ sIdx + 1 }}
            </div>
            <div v-if="sIdx !== group.steps.length - 1" class="flex-1 w-0.5 bg-border/50 my-3"/>
          </div>

          <div class="flex-1 space-y-4">

            <div v-if="editingStep !== `${gIdx}-${sIdx}`" class="flex items-start gap-6">
              <div class="flex-1 space-y-2">
                <p class="text-[13px] leading-relaxed text-foreground/80 whitespace-pre-wrap">
                  {{ getTrans(step, currentLang).instruction || 'No instructions defined yet...' }}
                </p>
                <div v-if="step.duration_minutes" class="flex items-center gap-1.5 text-[10px] font-bold text-primary uppercase">
                  <Clock class="w-3 h-3" /> {{ step.duration_minutes }} min
                </div>
              </div>

              <div v-if="(step as any).preview_url" class="w-24 h-24 rounded-lg overflow-hidden border shadow-sm shrink-0">
                <img :src="(step as any).preview_url" class="w-full h-full object-cover" />
              </div>

              <div class="flex flex-col gap-1 opacity-0 group-hover/step:opacity-100 transition-opacity">
                <Button variant="ghost" size="icon" class="h-8 w-8" @click="editingStep = `${gIdx}-${sIdx}`"><Edit3 class="w-3.5 h-3.5"/></Button>
                <Button variant="ghost" size="icon" class="h-8 w-8 hover:text-destructive" @click="group.steps.splice(sIdx, 1)"><Trash2 class="w-3.5 h-3.5"/></Button>
              </div>
            </div>

            <div v-else class="grid grid-cols-1 md:grid-cols-12 gap-6 p-5 bg-background border rounded-xl shadow-inner">
              <div class="md:col-span-8 space-y-4">
                <div class="space-y-2">
                  <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground">Instructions</label>
                  <Textarea v-model="getTrans(step, currentLang).instruction"
                            class="min-h-[120px] text-sm bg-muted/30 border-none focus-visible:ring-1"
                            placeholder="Describe this step..."/>
                </div>
                <div class="flex items-center gap-4">
                  <div class="flex-1 space-y-1">
                    <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground">Duration (min)</label>
                    <Input type="number" v-model.number="step.duration_minutes" class="h-9 bg-muted/30 border-none" />
                  </div>
                  <Button size="sm" class="mt-auto h-9 px-6 font-bold text-[11px] uppercase tracking-widest" @click="editingStep = null">
                    <Check class="w-3.5 h-3.5 mr-2"/>Done
                  </Button>
                </div>
              </div>

              <div class="md:col-span-4 space-y-2">
                <label class="text-[9px] font-black uppercase tracking-widest text-muted-foreground text-center block">Step Media</label>
                <div class="relative aspect-video md:aspect-square bg-muted/30 border-2 border-dashed rounded-xl flex flex-col items-center justify-center overflow-hidden group/img transition-all hover:border-primary/40">
                  <img v-if="(step as any).preview_url" :src="(step as any).preview_url" class="w-full h-full object-cover" />
                  <div v-else class="flex flex-col items-center gap-2 text-muted-foreground/40">
                    <Upload class="w-6 h-6" />
                    <span class="text-[8px] font-bold uppercase">Add Photo</span>
                  </div>
                  <input type="file" accept="image/*" @change="(e) => onStepImageChange(e, step)" class="absolute inset-0 opacity-0 cursor-pointer z-10" />
                </div>
              </div>
            </div>

          </div>
        </div>
      </div>

      <Button variant="outline" class="w-full mt-8 border-dashed h-12 rounded-xl text-muted-foreground hover:text-foreground hover:bg-muted/50"
              @click="group.steps.push({ translations: [], position: group.steps.length, image_url: null, duration_minutes: null })">
        <Plus class="w-4 h-4 mr-2"/> New Step
      </Button>
    </div>

    <Button variant="secondary" class="w-full h-16 border-2 border-dashed bg-transparent hover:bg-muted/40 rounded-2xl transition-all" @click="addGroup">
      <Plus class="w-5 h-5 mr-2 opacity-50" />
      <span class="font-bold text-[11px] uppercase tracking-[0.2em]">Add Step Group</span>
    </Button>
  </div>
</template>