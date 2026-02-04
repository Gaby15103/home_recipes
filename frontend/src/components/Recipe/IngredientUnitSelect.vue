<script setup lang="ts">
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { IngredientUnit } from "@/models/Recipe"
import {useI18n} from "vue-i18n";
const { t } = useI18n()
const props = defineProps<{
  modelValue: IngredientUnit
}>()

const emit = defineEmits<{
  (e: "update:modelValue", value: IngredientUnit): void
}>()

const units = Object.values(IngredientUnit)
</script>

<template>
  <Select
      :model-value="modelValue"
      @update:model-value="emit('update:modelValue', $event)"
  >
    <SelectTrigger class="w-full">
      <SelectValue :placeholder="t('admin.unite.placeholder')" />
    </SelectTrigger>

    <SelectContent>
      <SelectItem
          v-for="unit in units"
          :key="unit"
          :value="unit"
      >
        {{ unit }}
      </SelectItem>
    </SelectContent>
  </Select>
</template>
