<script setup lang="ts">
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import type {
  StepCreate,
  StepGroupCreate,
  StepImage
} from "@/models/RecipeCreate"
import { defineProps, defineEmits } from "vue"

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
</script>

<template>
  <div class="space-y-6 mb-8">
    <div class="flex justify-between items-center">
      <h2 class="text-xl font-semibold">Steps</h2>
      <Button size="sm" @click="addGroup">Add group</Button>
    </div>

    <div
        v-for="group in modelValue"
        :key="group.position"
        class="border rounded p-4 space-y-3"
    >
      <div class="grid grid-cols-2 gap-4">
        <Input v-model="group.title" placeholder="Group title" />
        <Button variant="outline" @click="removeGroup(group)">Remove</Button>
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
              placeholder="Minutes"
              v-model.number="step.duration_minutes"
          />

          <Input
              type="file"
              accept="image/*"
              @change="e => onImageChange(e, group, step)"
          />

          <Button
              v-if="getImage(group, step)"
              variant="destructive"
              size="sm"
              @click="removeImage(group, step)"
          >
            Remove image
          </Button>
        </div>

        <img
            v-if="getImage(group, step)"
            :src="getImage(group, step)!.image_preview"
            class="h-32 rounded border object-cover"
        />
      </div>

      <Button size="sm" variant="outline" @click="addStep(group)">
        Add step
      </Button>
    </div>
  </div>
</template>
