<script setup lang="ts">
import { ref, watch, defineProps, defineEmits } from "vue"

const props = defineProps<{
  modelValue: string
  height?: string
}>
const emit = defineEmits<{
  (e: "update:modelValue", value: string): void
}>()

const content = ref(props.modelValue || "")
watch(content, (val) => emit("update:modelValue", val))

function insertText(text: string) {
  const textarea = document.getElementById("md-editor-textarea") as HTMLTextAreaElement
  if (!textarea) return
  const start = textarea.selectionStart
  const end = textarea.selectionEnd
  const selected = textarea.value.slice(start, end)
  const wrapped = text.includes("```") ? `${text}\n${selected}\n\`\`\`` : `${text}${selected}`
  textarea.setRangeText(wrapped, start, end, "end")
  content.value = textarea.value
}
</script>

<textarea
    id="md-editor-textarea"
    v-model="content"
    :style="{ height: props.height || '250px' }"
    class="w-full p-2 border rounded focus:outline-none focus:ring focus:ring-indigo-300 dark:bg-gray-800 dark:text-gray-100"
></textarea>
