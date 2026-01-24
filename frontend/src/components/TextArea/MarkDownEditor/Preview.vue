<script setup lang="ts">
import { defineProps } from "vue"
import {marked} from "marked";
import prism from "prismjs"
import "prismjs/themes/prism-tomorrow.css"


const props = defineProps<{
  content: string
}>()

// Highlight code blocks
marked.setOptions({
  highlight: function (code, lang) {
    if (lang && prism.languages[lang]) return prism.highlight(code, prism.languages[lang], lang)
    return code
  },
})
</script>

<div
    class="prose dark:prose-invert p-2 border rounded bg-gray-50 dark:bg-gray-900 overflow-auto max-h-[500px]"
    v-html="marked.parse(props.content)"
></div>
