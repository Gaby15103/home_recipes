<script setup lang="ts">
import MarkdownIt from "markdown-it"
import DOMPurify from "dompurify"
import {computed} from "vue";

const md = new MarkdownIt({
  linkify: true,
  breaks: true
})

const props = defineProps<{ content: string }>()

const rendered = computed(() => {
  return DOMPurify.sanitize(md.render(props.content))
})
</script>

<template>
  <div class="prose dark:prose-invert" v-html="rendered"></div>
</template>
