<script setup lang="ts">
import { ref, defineProps, defineEmits, watch } from "vue";

const props = defineProps<{
  open: boolean
}>();

const emit = defineEmits<{
  (e: "update:open", value: boolean): void
}>();

function close() {
  emit("update:open", false);
}
</script>

<template>
  <div
      v-if="props.open"
      class="fixed inset-0 flex items-center justify-center z-50 bg-black/30"
  >
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg max-w-md w-full p-4 relative">
      <button
          class="absolute top-2 right-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          @click="close"
      >
        ✖
      </button>
      <h3 class="text-lg font-bold mb-2 text-center">Markdown Help</h3>
      <p class="text-sm mb-2">
        Markdown allows you to format text quickly without a toolbar.
        <a href="https://www.markdownguide.org/basic-syntax/" target="_blank" class="underline text-blue-600">Full guide</a>
      </p>
      <table class="table-auto w-full text-sm border-collapse border border-gray-300 dark:border-gray-600">
        <thead>
        <tr>
          <th class="border px-2 py-1">Type this</th>
          <th class="border px-2 py-1">Result</th>
        </tr>
        </thead>
        <tbody>
        <tr><td>*italic*</td><td><em>italic</em></td></tr>
        <tr><td>**bold**</td><td><strong>bold</strong></td></tr>
        <tr><td>[link](url)</td><td><a href="#">link</a></td></tr>
        <tr><td>- list item</td><td><ul><li>list item</li></ul></td></tr>
        <tr><td>`code`</td><td><code>code</code></td></tr>
        <tr><td>```code```</td><td><pre>code</pre></td></tr>
        <tr><td>> quote</td><td><blockquote>quote</blockquote></td></tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
