<script setup lang="ts">
import {ref, onMounted, computed} from "vue";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import type { Unit } from "@/models/Recipe";
import { useI18n } from "vue-i18n";
import {getUnits} from '@/api/unit'

const { t, locale } = useI18n();

const props = defineProps<{
  modelValue: string | undefined // This is the unit ID (UUID)
}>()

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void
}>()

const units = ref<Unit[]>([]);
const isLoading = ref(true);

onMounted(async () => {
  try {
    units.value = await getUnits();
  } catch (error) {
    console.error("Failed to load units", error);
  } finally {
    isLoading.value = false;
  }
});
const internalValue = computed(() => props.modelValue?.toString() || "");

// Helper to pick name based on current language
const localizedName = (unit: Unit) => {
  return locale.value === 'fr' ? unit.name_fr : unit.name_en;
}
</script>

<template>
  <Select
      :model-value="internalValue"
      @update:model-value="(val) => emit('update:modelValue', val)"
      :disabled="isLoading"
  >
    <SelectTrigger class="w-full">
      <SelectValue :placeholder="isLoading ? '...' : t('Admin.unit.placeholder')" />
    </SelectTrigger>

    <SelectContent>
      <SelectItem
          v-for="unit in units"
          :key="unit.id"
          :value="unit.id"
      >
        {{ localizedName(unit) }} ({{ unit.symbol }})
      </SelectItem>
    </SelectContent>
  </Select>
</template>