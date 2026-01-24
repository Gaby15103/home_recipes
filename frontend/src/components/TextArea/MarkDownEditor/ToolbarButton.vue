<script setup lang="ts">
import { defineProps, defineEmits } from "vue"

const props = defineProps<{
  icon?: string
  label: string
}>
const emit = defineEmits<{
  (e: "click"): void
}>()
</script>
<template>
  <button
      type="button"
      class="px-2 py-1 border rounded hover:bg-gray-100 dark:hover:bg-gray-700"
      @click="$emit('click')"
      :title="props.label"
  >
    <slot>{{ props.label }}</slot>
  </button>
</template>

