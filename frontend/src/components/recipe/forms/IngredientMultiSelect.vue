<script setup lang="ts">
import { ref, watch } from 'vue'
import {
  TagsInputInput,
  TagsInputItem,
  TagsInputItemDelete,
  TagsInputItemText,
  TagsInputRoot
} from 'radix-vue'
import { Icon } from '@iconify/vue'
import { useI18n } from "vue-i18n"

const { t } = useI18n()

// We change the model to string[] for simpler filtering
const props = withDefaults(defineProps<{ modelValue: string[] }>(), {
  modelValue: () => []
})
const emit = defineEmits(['update:modelValue'])

const localTags = ref<string[]>(props.modelValue)

// Sync with parent
watch(() => props.modelValue, (newVal) => {
  localTags.value = newVal
})

watch(localTags, (newVal) => {
  emit('update:modelValue', newVal)
})
</script>

<template>
  <TagsInputRoot
      v-model="localTags"
      class="flex gap-2 items-center rounded-lg border border-input p-2 min-h-11 flex-wrap flex-1 bg-background shadow-sm focus-within:ring-2 focus-within:ring-ring outline-none"
  >
    <TagsInputItem
        v-for="tag in localTags"
        :key="tag"
        :value="tag"
        class="flex items-center justify-center gap-2 text-secondary-foreground bg-secondary hover:bg-secondary/80 transition-colors rounded px-2 py-1"
    >
      <TagsInputItemText class="text-sm font-medium">{{ tag }}</TagsInputItemText>
      <TagsInputItemDelete>
        <Icon icon="lucide:x" class="h-3 w-3" />
      </TagsInputItemDelete>
    </TagsInputItem>

    <TagsInputInput
        :placeholder="t('Ingredients.search_placeholder')"
        class="focus:outline-none flex-1 rounded bg-transparent placeholder:text-muted-foreground px-1 min-w-[150px] text-sm"
    />
  </TagsInputRoot>
</template>