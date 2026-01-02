<script setup lang="ts">
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible"
import type { RecipeFilter } from "@/models/Recipe.ts"
import { computed } from "vue"

const props = defineProps<{
  modelValue: RecipeFilter
}>()

const emit = defineEmits<{
  (e: "update:modelValue", value: RecipeFilter): void
}>()

const filters = computed({
  get: () => props.modelValue,
  set: value => emit("update:modelValue", value),
})

function reset() {
  emit("update:modelValue", {
    search: "",
    ingredient: "",
    tags: [],
    minPrep: null,
    maxPrep: null,
    minCook: null,
    maxCook: null,
    minSteps: null,
    maxSteps: null,
    dateFrom: null,
    dateTo: null,
  })
}


</script>

<template>
  <div class="flex gap-4 mb-4">
    <Input
        v-model="filters.search"
        placeholder="Search name or description"
    />

    <Input
        v-model="filters.ingredient"
        placeholder="Ingredient"
    />
    <Button variant="ghost" @click="reset">
      Reset filters
    </Button>
  </div>

  <Collapsible class="border rounded-lg p-4">
    <CollapsibleTrigger class="flex justify-between w-full">
      <span class="font-medium">Advanced filters</span>
      <ChevronDown class="h-4 w-4" />
    </CollapsibleTrigger>

    <CollapsibleContent class="mt-4 space-y-4">
      <div class="grid grid-cols-2 gap-4">
        <Input type="number" v-model.number="filters.minPrep" placeholder="Min prep" />
        <Input type="number" v-model.number="filters.maxPrep" placeholder="Max prep" />
        <Input type="number" v-model.number="filters.minCook" placeholder="Min cook" />
        <Input type="number" v-model.number="filters.maxCook" placeholder="Max cook" />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <Input type="number" v-model.number="filters.minSteps" placeholder="Min steps" />
        <Input type="number" v-model.number="filters.maxSteps" placeholder="Max steps" />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <Input type="date" v-model="filters.dateFrom" />
        <Input type="date" v-model="filters.dateTo" />
      </div>

      <TagsSelect v-model="filters.tags" />
    </CollapsibleContent>
  </Collapsible>
</template>