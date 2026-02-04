<script setup lang="ts">
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible"
import type { RecipeFilter } from "@/models/Recipe.ts"
import { computed } from "vue"
import {useI18n} from "vue-i18n";
import {Button} from "@/components/ui/button";
import {Input} from "@/components/ui/input";
import TagsMultiSelect from "@/components/Recipe/TagsMultiSelect.vue";
const { t } = useI18n()
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
        :placeholder="t('Admin.filters.search')"
    />

    <Input
        v-model="filters.ingredient"
        :placeholder="t('Admin.filters.ingredient')"
    />
    <Button variant="ghost" @click="reset">
      {{ t('Admin.filters.reset') }}
    </Button>
  </div>

  <Collapsible class="border rounded-lg p-4">
    <CollapsibleTrigger class="flex justify-between w-full">
      <span class="font-medium">
        {{ t('Admin.filters.advanced') }}
      </span>
      <ChevronDown class="h-4 w-4" />
    </CollapsibleTrigger>

    <CollapsibleContent class="mt-4 space-y-4">
      <div class="grid grid-cols-2 gap-4">
        <Input
            type="number"
            v-model.number="filters.minPrep"
            :placeholder="t('Admin.filters.prep_min')"
        />
        <Input
            type="number"
            v-model.number="filters.maxPrep"
            :placeholder="t('Admin.filters.prep_max')"
        />
        <Input
            type="number"
            v-model.number="filters.minCook"
            :placeholder="t('Admin.filters.cook_min')"
        />
        <Input
            type="number"
            v-model.number="filters.maxCook"
            :placeholder="t('Admin.filters.cook_max')"
        />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <Input
            type="number"
            v-model.number="filters.minSteps"
            :placeholder="t('Admin.filters.steps_min')"
        />
        <Input
            type="number"
            v-model.number="filters.maxSteps"
            :placeholder="t('Admin.filters.steps_max')"
        />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <Input type="date" v-model="filters.dateFrom" />
        <Input type="date" v-model="filters.dateTo" />
      </div>

      <TagsMultiSelect v-model="filters.tags" />
    </CollapsibleContent>
  </Collapsible>
</template>