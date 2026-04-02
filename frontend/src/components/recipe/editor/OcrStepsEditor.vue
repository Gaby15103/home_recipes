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
  <div class="space-y-8 md:space-y-12">
    <div v-for="(group, gIdx) in modelValue" :key="gIdx"
         class="relative group/card border rounded-xl md:rounded-2xl p-4 md:p-8 bg-card shadow-sm transition-all hover:shadow-md">

      <Button variant="ghost" size="icon" @click="removeGroup(gIdx)"
              class="absolute -top-2 -right-2 h-7 w-7 md:h-8 md:w-8 rounded-full bg-destructive text-white opacity-100 md:opacity-0 md:group-hover/card:opacity-100 transition-opacity shadow-md z-10">
        <X class="w-3.5 h-3.5" />
      </Button>

      <div class="flex items-center gap-3 mb-6 md:mb-10">
        <Input v-model="getTrans(group, currentLang).title"
               class="flex-1 md:w-auto font-black uppercase text-[9px] md:text-[10px] tracking-[0.2em] h-8 px-4 md:px-5 rounded-full bg-muted border-none"
               placeholder="STEP GROUP"/>
        <div class="hidden md:block h-px flex-1 bg-gradient-to-r from-border to-transparent"/>
      </div>

      <div class="space-y-6 md:space-y-10">
        <div v-for="(step, sIdx) in group.steps" :key="sIdx"
             class="relative group/step flex gap-3 md:gap-6 p-2 md:p-4 rounded-xl transition-colors hover:bg-muted/20">

          <div class="flex flex-col items-center">
            <div class="w-8 h-8 md:w-10 md:h-10 rounded-full border-2 bg-background flex items-center justify-center text-[10px] md:text-xs font-black shadow-sm group-hover/step:border-primary transition-colors">
              {{ sIdx + 1 }}
            </div>
            <div v-if="sIdx !== group.steps.length - 1" class="flex-1 w-0.5 bg-border/50 my-2 md:my-3"/>
          </div>

          <div class="flex-1 space-y-4 min-w-0">
            <div v-if="editingStep !== `${gIdx}-${sIdx}`" class="flex items-start gap-3 md:gap-6">
              <div class="flex-1 space-y-2 min-w-0">
                <p class="text-xs md:text-[13px] leading-relaxed text-foreground/80 whitespace-pre-wrap line-clamp-4 md:line-clamp-none italic">
                  {{ getTrans(step, currentLang).instruction || 'No instructions...' }}
                </p>
                <div v-if="step.duration_minutes" class="flex items-center gap-1 text-[8px] md:text-[10px] font-black text-primary uppercase">
                  <Clock class="w-3 h-3" /> {{ step.duration_minutes }} min
                </div>
              </div>

              <div v-if="(step as any).preview_url" class="w-16 h-16 md:w-24 md:h-24 rounded-lg overflow-hidden border shadow-sm shrink-0 bg-muted">
                <img :src="(step as any).preview_url" class="w-full h-full object-cover" />
              </div>

              <div class="flex flex-col gap-1 shrink-0">
                <Button variant="ghost" size="icon" class="h-8 w-8" @click="editingStep = `${gIdx}-${sIdx}`"><Edit3 class="w-3.5 h-3.5"/></Button>
                <Button variant="ghost" size="icon" class="h-8 w-8 hover:text-destructive" @click="group.steps.splice(sIdx, 1)"><Trash2 class="w-3.5 h-3.5"/></Button>
              </div>
            </div>

            <div v-else class="flex flex-col p-4 md:p-5 bg-background border rounded-xl shadow-inner gap-4 md:gap-6">
              <div class="grid grid-cols-1 md:grid-cols-12 gap-4 md:gap-6">
                <div class="md:col-span-8 space-y-4 order-2 md:order-1">
                  <div class="space-y-1.5">
                    <label class="text-[8px] md:text-[9px] font-black uppercase tracking-widest text-muted-foreground">Instructions</label>
                    <Textarea v-model="getTrans(step, currentLang).instruction"
                              class="min-h-[100px] md:min-h-[120px] text-sm bg-muted/30 border-none focus-visible:ring-1"
                              placeholder="Describe this step..."/>
                  </div>
                  <div class="flex items-end gap-4">
                    <div class="flex-1 space-y-1.5">
                      <label class="text-[8px] md:text-[9px] font-black uppercase tracking-widest text-muted-foreground">Duration (min)</label>
                      <Input type="number" v-model.number="step.duration_minutes" class="h-9 bg-muted/30 border-none" />
                    </div>
                    <Button size="sm" class="h-9 px-4 md:px-6 font-bold text-[10px] uppercase tracking-widest" @click="editingStep = null">
                      <Check class="w-3.5 h-3.5 mr-2"/>Done
                    </Button>
                  </div>
                </div>

                <div class="md:col-span-4 space-y-2 order-1 md:order-2">
                  <label class="text-[8px] md:text-[9px] font-black uppercase tracking-widest text-muted-foreground text-center block">Step Media</label>
                  <div class="relative aspect-video md:aspect-square bg-muted/30 border-2 border-dashed rounded-xl flex flex-col items-center justify-center overflow-hidden group/img transition-all hover:border-primary/40">
                    <img v-if="(step as any).preview_url" :src="(step as any).preview_url" class="w-full h-full object-cover" />
                    <div v-else class="flex flex-col items-center gap-1 md:gap-2 text-muted-foreground/40">
                      <Upload class="w-5 h-5 md:w-6 md:h-6" />
                      <span class="text-[7px] md:text-[8px] font-bold uppercase">Add Photo</span>
                    </div>
                    <input type="file" accept="image/*" @change="(e) => onStepImageChange(e, step)" class="absolute inset-0 opacity-0 cursor-pointer z-10" />
                  </div>
                </div>
              </div>
            </div>

          </div>
        </div>
      </div>

      <Button variant="outline" class="w-full mt-6 md:mt-8 border-dashed h-11 md:h-12 rounded-xl text-xs"
              @click="group.steps.push({ translations: [], position: group.steps.length, image_url: null, duration_minutes: null })">
        <Plus class="w-3.5 h-3.5 mr-2"/> New Step
      </Button>
    </div>

    <Button variant="secondary" class="w-full h-14 md:h-16 border-2 border-dashed bg-transparent hover:bg-muted/40 rounded-xl md:rounded-2xl transition-all" @click="addGroup">
      <Plus class="w-4 h-4 mr-2 opacity-50" />
      <span class="font-bold text-[9px] md:text-[11px] uppercase tracking-[0.2em]">Add Step Group</span>
    </Button>
  </div>
</template>