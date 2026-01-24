<script setup lang="ts">
import {ref, watch, defineProps, defineEmits} from "vue"
import Toolbar from "./Toolbar.vue"
import EditorArea from "./EditorArea.vue"
import Preview from "./Preview.vue"

const props = defineProps<{
  modelValue: string
  height?: string
}>
const emit = defineEmits<{
  (e: "update:modelValue", value: string): void
}>()

const content = ref(props.modelValue || "")
watch(content, (val) => emit("update:modelValue", val))

function handleAction(action: string) {
  const textarea = document.getElementById("md-editor-textarea") as HTMLTextAreaElement
  if (!textarea) return
  const start = textarea.selectionStart
  const end = textarea.selectionEnd
  const selected = textarea.value.slice(start, end)
  const insert = action.includes("```") ? `${action}\n${selected}\n\`\`\`` : `${action}${selected}`
  textarea.setRangeText(insert, start, end, "end")
  content.value = textarea.value
}
</script>
<template>
  <div class="border rounded p-2 dark:border-gray-700">
    <Toolbar @action="handleAction"/>
    <div class="flex gap-2">
      <EditorArea v-model="content" :height="props.height"/>
      <Preview :content="content"/>
    </div>
  </div>
</template>
