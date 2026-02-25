<script setup lang="ts">
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Label } from "@/components/ui/label"
import type {
  StepCreate,
  StepGroupCreate,
  StepGroupTranslationCreate,
  StepTranslationCreate
} from "@/models/RecipeCreate.ts"
import { useI18n } from "vue-i18n";
import type { Language } from "@/models/Language.ts";
import { getCurrentInstance } from 'vue'

// 1. Get the instance
const instance = getCurrentInstance()

// 2. Access the property
const apiUrl = instance?.proxy?.$apiUrl
const { t } = useI18n()

const props = defineProps<{
  modelValue: StepGroupCreate[]
  availableLanguages: Language[]
  currentLang: string
}>()

const emit = defineEmits<{
  (e: "update:modelValue", v: StepGroupCreate[]): void
}>()

/* ---------------- TRANSLATION HELPERS ---------------- */

function getGroupTrans(group: StepGroupCreate, langCode: string): StepGroupTranslationCreate {
  if (!Array.isArray(group.translations)) {
    group.translations = [] as any;
  }
  let trans = (group.translations as any).find((t: any) => t.language_code === langCode);
  if (!trans) {
    trans = { language_code: langCode, title: "" };
    (group.translations as any).push(trans);
  }
  return trans;
}

function getStepTrans(step: StepCreate, langCode: string): StepTranslationCreate {
  if (!Array.isArray(step.translations)) {
    step.translations = [] as any;
  }
  let trans = (step.translations as any).find((t: any) => t.language_code === langCode);
  if (!trans) {
    trans = { language_code: langCode, instruction: "" };
    (step.translations as any).push(trans);
  }
  return trans;
}

/* ---------------- GROUPS ---------------- */

function addGroup() {
  if (!props.availableLanguages) return;

  const translations = props.availableLanguages.map(l => ({
    language_code: l.code,
    title: ""
  }));

  emit("update:modelValue", [
    ...props.modelValue,
    {
      translations: translations as any,
      position: props.modelValue.length,
      steps: [],
    },
  ])
}

function removeGroup(index: number) {
  const newGroups = [...props.modelValue];
  const group = newGroups[index];

  // Cleanup local object URLs to prevent memory leaks
  group.steps.forEach(step => {
    if (typeof step.image_url === 'string' && step.image_url.startsWith('blob:')) {
      URL.revokeObjectURL(step.image_url);
    }
  });

  newGroups.splice(index, 1);
  emit("update:modelValue", newGroups.map((g, i) => ({ ...g, position: i })));
}

/* ---------------- STEPS ---------------- */

function addStep(group: StepGroupCreate) {
  if (!props.availableLanguages) return;

  const translations = props.availableLanguages.map(l => ({
    language_code: l.code,
    instruction: ""
  }));

  group.steps.push({
    position: group.steps.length,
    translations: [],
    duration_minutes: null,
    image_url: null
  });
  emit("update:modelValue", [...props.modelValue]);
}

function removeStep(group: StepGroupCreate, sIdx: number) {
  const step = group.steps[sIdx];

  if (typeof step.image_url === 'string' && step.image_url.startsWith('blob:')) {
    URL.revokeObjectURL(step.image_url);
  }

  group.steps.splice(sIdx, 1);
  group.steps.forEach((s, i) => (s.position = i));
  emit("update:modelValue", [...props.modelValue]);
}

/* ---------------- IMAGES (Direct to step.image_url) ---------------- */

function onImageChange(e: Event, step: StepCreate) {
  const input = e.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  // Revoke old preview if it exists
  if (typeof step.image_url === 'string' && step.image_url.startsWith('blob:')) {
    URL.revokeObjectURL(step.image_url);
  }

  // Store the File object (your backend logic handles the temp file creation later)
  // We also create a temporary preview for the <img> tag
  step.image_url = file;
  input.value = "";
  emit("update:modelValue", [...props.modelValue]);
}

function removeImage(step: StepCreate) {
  if (typeof step.image_url === 'string' && step.image_url.startsWith('blob:')) {
    URL.revokeObjectURL(step.image_url);
  }
  step.image_url = null;
  emit("update:modelValue", [...props.modelValue]);
}

function getImageUrl(step: StepCreate): string | null {
  if (!step.image_url) return null;
  if (step.image_url instanceof File) {
    return URL.createObjectURL(step.image_url);
  }
  if (step.image_url.includes("/assets/")){
    return apiUrl+step.image_url
  }
  return step.image_url as string;
}

// Refs management for hidden file inputs
const fileRefs = new Map<string, HTMLInputElement>()
const key = (g: number, s: number) => `${g}-${s}`
const setFileRef = (el: any, g: number, s: number) => { if (el) fileRefs.set(key(g, s), el) }
const openFile = (g: number, s: number) => fileRefs.get(key(g, s))?.click()

</script>

<template>
  <div class="space-y-6 mb-8">
    <div class="flex justify-between items-center">
      <div>
        <h2 class="text-xl font-semibold">{{ t('Admin.steps.title') }}</h2>
        <p class="text-xs text-muted-foreground uppercase font-bold">{{ currentLang }}</p>
      </div>
      <Button type="button" size="sm" @click="addGroup">{{ t('Admin.steps.addGroup') }}</Button>
    </div>

    <div v-for="(group, gIdx) in modelValue" :key="gIdx" class="border rounded-lg p-4 space-y-4 bg-card">
      <div class="flex gap-4 items-end">
        <div class="flex-1">
          <Label class="text-xs">{{ t('Admin.steps.groupTitle') }} ({{ currentLang }})</Label>
          <Input v-model="getGroupTrans(group, currentLang).title" :placeholder="t('Admin.steps.groupTitlePlaceholder')" />
        </div>
        <Button type="button" variant="destructive" size="sm" @click="removeGroup(gIdx)">
          ✕
        </Button>
      </div>

      <div v-for="(step, sIdx) in group.steps" :key="sIdx" class="space-y-3 p-4 border rounded-md bg-muted/30">
        <div class="flex justify-between items-center">
          <Label class="font-bold text-sm">{{ t('Admin.steps.step') }} {{ sIdx + 1 }}</Label>
          <Button type="button" variant="destructive" size="sm" class=" h-8" @click="removeStep(group, sIdx)">
            ✕
          </Button>
        </div>

        <Textarea
            v-model="getStepTrans(step, currentLang).instruction"
            :placeholder="t('Admin.steps.instructionPlaceholder')"
            rows="3"
        />

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-end">
          <div class="space-y-1">
            <Label class="text-xs">{{ t('Admin.steps.duration') }} (min)</Label>
            <Input type="number" v-model.number="step.duration_minutes" placeholder="0" />
          </div>

          <div class="flex flex-col gap-2">
            <input
                type="file"
                accept="image/*"
                class="hidden"
                :ref="el => setFileRef(el, group.position, step.position)"
                @change="e => onImageChange(e, step)"
            />

            <div class="flex gap-2">
              <Button type="button" size="sm" variant="secondary" @click="openFile(group.position, step.position)">
                {{ step.image_url ? t('Admin.steps.change_image') : t('Admin.steps.choose_image') }}
              </Button>
              <Button v-if="step.image_url" type="button" variant="destructive" size="sm" @click="removeImage(step)">
                ✕
              </Button>
            </div>

          </div>
        </div>

        <img v-if="step.image_url" :src="getImageUrl(step)!" class=" w-full rounded border object-cover mt-2" alt=""/>
      </div>

      <Button type="button" size="sm" variant="outline" class="w-full border-dashed" @click="addStep(group)">
        + {{ t('Admin.steps.addStep') }}
      </Button>
    </div>
  </div>
</template>