<script setup lang="ts">
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import type {
  StepCreate,
  StepGroupCreate,
  StepImage
} from "@/models/RecipeCreate"
import {useI18n} from "vue-i18n";
const { t } = useI18n()
const props = defineProps<{
  modelValue: StepGroupCreate[]
  images: StepImage[]
}>()

const emit = defineEmits<{
  (e: "update:modelValue", v: StepGroupCreate[]): void
  (e: "update:images", v: StepImage[]): void
}>()

/* ---------------- GROUPS ---------------- */

function addGroup() {
  emit("update:modelValue", [
    ...props.modelValue,
    {
      title: "",
      position: props.modelValue.length,
      steps: [],
    },
  ])
}

function removeGroup(group: StepGroupCreate) {
  const newGroups = props.modelValue.filter(g => g !== group)

  // cleanup images for this group
  const remainingImages = props.images.filter(
      img => img.group_position !== group.position
  )

  remainingImages.forEach(img => URL.revokeObjectURL(img.image_preview))

  emit("update:images", remainingImages)
  emit("update:modelValue", newGroups.map((g, i) => ({ ...g, position: i })))
}

/* ---------------- STEPS ---------------- */

function addStep(group: StepGroupCreate) {
  group.steps.push({
    position: group.steps.length,
    instruction: "",
    duration_minutes: null,
  })
}

function removeStep(group: StepGroupCreate, step: StepCreate) {
  // cleanup image
  const img = props.images.find(
      i =>
          i.group_position === group.position &&
          i.step_position === step.position
  )

  if (img) {
    URL.revokeObjectURL(img.image_preview)
    emit(
        "update:images",
        props.images.filter(i => i !== img)
    )
  }

  group.steps.splice(step.position, 1)
  group.steps.forEach((s, i) => (s.position = i))
}

/* ---------------- IMAGES ---------------- */

function onImageChange(
    e: Event,
    group: StepGroupCreate,
    step: StepCreate
) {
  const input = e.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return

  const existing = props.images.find(
      i =>
          i.group_position === group.position &&
          i.step_position === step.position
  )

  if (existing) {
    URL.revokeObjectURL(existing.image_preview)
  }

  const preview = URL.createObjectURL(file)

  emit("update:images", [
    ...props.images.filter(
        i =>
            i.group_position !== group.position ||
            i.step_position !== step.position
    ),
    {
      group_position: group.position,
      step_position: step.position,
      image_file: file,
      image_preview: preview,
    },
  ])
}

function removeImage(group: StepGroupCreate, step: StepCreate) {
  const img = props.images.find(
      i =>
          i.group_position === group.position &&
          i.step_position === step.position
  )

  if (!img) return

  URL.revokeObjectURL(img.image_preview)

  emit(
      "update:images",
      props.images.filter(i => i !== img)
  )
}

function getImage(group: StepGroupCreate, step: StepCreate) {
  return props.images.find(
      i =>
          i.group_position === group.position &&
          i.step_position === step.position
  )
}

const fileRefs = new Map<string, HTMLInputElement>()

function key(g: number, s: number) {
  return `${g}-${s}`
}

function setFileRef(
    el: HTMLInputElement | null,
    g: number,
    s: number
) {
  if (!el) return
  fileRefs.set(key(g, s), el)
}

function openFile(g: number, s: number) {
  fileRefs.get(key(g, s))?.click()
}


</script>

<template>
  <div class="space-y-6 mb-8">
    <div class="flex justify-between items-center">
      <h2 class="text-xl font-semibold">{{ t('Admin.steps.title') }}</h2>
      <Button size="sm" @click="addGroup">{{ t('Admin.steps.addGroup') }}</Button>
    </div>

    <div
        v-for="group in modelValue"
        :key="group.position"
        class="border rounded p-4 space-y-3"
    >
      <div class="grid grid-cols-2 gap-4">
        <Input v-model="group.title" :placeholder="t('Admin.steps.remove')" />
        <Button variant="outline" @click="removeGroup(group)">{{ t('Admin.steps.groupTitle') }}</Button>
      </div>

      <div
          v-for="step in group.steps"
          :key="step.position"
          class="space-y-2 p-3 border rounded"
      >
        <Textarea v-model="step.instruction" placeholder="Instruction" />

        <div class="flex gap-2 items-center">
          <Input
              type="number"
              :placeholder="t('Admin.steps.duration')"
              v-model.number="step.duration_minutes"
          />

          <div>
            <input
                type="file"
                accept="image/*"
                class="hidden"
                :ref="el => setFileRef(el, group.position, step.position)"
                @change="e => onImageChange(e, group, step)"
            />

            <Button
                size="sm"
                variant="secondary"
                @click="openFile(group.position, step.position)"
            >
              {{ t('Admin.steps.choose_image') }}
            </Button>
            <p class="text-sm text-muted-foreground">
              {{
                getImage(group, step)?.image_file?.name ??
                t("Admin.steps.no_image_selected")
              }}
            </p>

          </div>


          <Button
              v-if="getImage(group, step)"
              variant="destructive"
              size="sm"
              @click="removeImage(group, step)"
          >
            {{ t('Admin.steps.remove_image') }}
          </Button>
        </div>

        <img
            v-if="getImage(group, step)"
            :src="getImage(group, step)!.image_preview"
            class="h-32 rounded border object-cover"
         alt=""/>
      </div>

      <Button size="sm" variant="outline" @click="addStep(group)">
        {{ t('Admin.steps.addStep') }}
      </Button>
    </div>
  </div>
</template>
