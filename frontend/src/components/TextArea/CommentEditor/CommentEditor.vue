<script setup lang="ts">
import { ref, defineProps, defineEmits, watch, nextTick, onMounted } from "vue";
import MarkdownToolbarModal from "./MarkdownToolbarModal.vue";
import type { RecipeCommentCreate } from "@/models/RecipeCreate";
import { addComment } from "@/api/recipe";
import { useAuthStore } from "@/stores/auth.ts";
import type { RecipeComment } from "@/models/Recipe.ts";
import TurndownService from "turndown";
import { marked } from "marked";

const turndown = new TurndownService();
const authStore = useAuthStore();

const { recipeId, parent, modelValue } = defineProps<{
  recipeId: string;
  parent?: RecipeComment | null;
  modelValue?: string;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
  (e: "posted", comment: any): void;
  (e: "cancel"): void;
}>();

// Editor state
const mode = ref<"markdown" | "rich">("markdown");
const showMarkdownHelper = ref(false);
const content = ref(modelValue || "");

// Ref to contenteditable div
const editableDiv = ref<HTMLElement | null>(null);

// Keep content synced with parent
watch(() => modelValue, (v) => {
  content.value = v || "";
});

// Convert Markdown <-> Rich Text
function markdownToRich(md: string) {
  return marked.parse(md);
}

function richToMarkdown(html: string) {
  return turndown.turndown(html);
}

watch(mode, async (newMode, oldMode) => {
  if (oldMode === "markdown" && newMode === "rich") {
    // Markdown -> Rich
    await nextTick();
    if (editableDiv.value) {
      editableDiv.value.innerHTML = markdownToRich(content.value);
      placeCaretAtEnd(editableDiv.value);
    }
  } else if (oldMode === "rich" && newMode === "markdown") {
    // Rich -> Markdown
    // Update content value now
    content.value = richToMarkdown(editableDiv.value?.innerHTML || "");
    // No need for editableDiv in Markdown mode
  }
});


// Ensure caret is at the end after switching modes
function placeCaretAtEnd(el: HTMLElement) {
  el.focus();
  const range = document.createRange();
  range.selectNodeContents(el);
  range.collapse(false);
  const sel = window.getSelection();
  sel?.removeAllRanges();
  sel?.addRange(range);
}

// Update content on input
function onInput() {
  if (!editableDiv.value) return;
  content.value = editableDiv.value.innerHTML;
  emit("update:modelValue", content.value);
}

// Handle Enter for <br>
function handleKeyDown(event: KeyboardEvent) {
  if (event.key === "Enter") {
    event.preventDefault();

    const sel = window.getSelection();
    if (!sel?.rangeCount || !editableDiv.value) return;

    const range = sel.getRangeAt(0);
    const br = document.createElement("br");

    range.deleteContents();
    range.insertNode(br);
    range.setStartAfter(br);
    range.collapse(true);

    sel.removeAllRanges();
    sel.addRange(range);

    editableDiv.value.scrollTop = editableDiv.value.scrollHeight;
    onInput();
  }
}

// Toolbar formatting
function format(command: string, value?: string) {
  document.execCommand(command, false, value || null);
  onInput();
}

// Cancel
function cancel() {
  content.value = "";
  if (editableDiv.value) editableDiv.value.innerHTML = "";
  emit("update:modelValue", "");
  emit("cancel");
}

// Submit comment
async function submit() {
  if (!content.value.trim() || !authStore.user) return;

  const markdownContent =
      mode.value === "rich" ? richToMarkdown(editableDiv.value?.innerHTML || "") : content.value;

  const comment: RecipeCommentCreate = {
    recipe_id: recipeId,
    parent_id: parent?.id || null,
    user_id: authStore.user.id,
    content: markdownContent,
  };

  try {
    const created = await addComment(recipeId, comment);
    emit("posted", created);

    // Clear editor
    content.value = "";
    if (editableDiv.value) editableDiv.value.innerHTML = "";
    emit("update:modelValue", content.value);
  } catch (err) {
    console.error("Failed to post comment:", err);
    alert("Failed to post comment");
  }
}


// Ensure editableDiv exists after mount
onMounted(() => {
  if (mode.value === "rich" && editableDiv.value) {
    editableDiv.value.innerHTML = markdownToRich(content.value);
  }
});
</script>

<template>
  <div class="border rounded-lg p-3 bg-gray-50 dark:bg-gray-900 space-y-2">
    <!-- Header -->
    <div class="flex justify-between items-center mb-2">
      <div class="flex items-center gap-2 relative">
        <span class="font-semibold">Markdown Editor</span>
        <button @click="showMarkdownHelper = true" class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200">ℹ️</button>
      </div>
      <button class="text-sm px-2 py-1 border rounded hover:bg-gray-200 dark:hover:bg-gray-700"
              @click="mode = mode === 'markdown' ? 'rich' : 'markdown'">
        Switch to {{ mode === "markdown" ? "Rich Text" : "Markdown" }}
      </button>
    </div>

    <!-- Markdown Helper Modal -->
    <MarkdownToolbarModal v-model:open="showMarkdownHelper" />

    <!-- Editor Body -->
    <div v-if="mode === 'markdown'">
      <textarea
          v-model="content"
          :placeholder="parent ? 'Reply to ' + parent.username : 'Write your comment'"
          class="w-full min-h-[120px] border rounded p-2 resize-none focus:outline-none focus:ring-1 focus:ring-primary dark:bg-gray-800 dark:text-gray-200"
      ></textarea>
    </div>

    <div v-else class="space-y-2">
      <div class="flex gap-2 mb-2 flex-wrap">
        <button @click.prevent="format('bold')" class="px-2 py-1 border rounded hover:bg-gray-200 dark:hover:bg-gray-700">B</button>
        <button @click.prevent="format('italic')" class="px-2 py-1 border rounded hover:bg-gray-200 dark:hover:bg-gray-700">I</button>
        <button @click.prevent="format('underline')" class="px-2 py-1 border rounded hover:bg-gray-200 dark:hover:bg-gray-700">U</button>
        <button @click.prevent="format('insertOrderedList')" class="px-2 py-1 border rounded hover:bg-gray-200 dark:hover:bg-gray-700">OL</button>
        <button @click.prevent="format('insertUnorderedList')" class="px-2 py-1 border rounded hover:bg-gray-200 dark:hover:bg-gray-700">UL</button>
        <button @click.prevent="format('createLink', prompt('Enter URL:'))" class="px-2 py-1 border rounded hover:bg-gray-200 dark:hover:bg-gray-700">Link</button>
        <button @click.prevent="format('formatBlock','pre')" class="px-2 py-1 border rounded hover:bg-gray-200 dark:hover:bg-gray-700">Code</button>
      </div>

      <div
          contenteditable
          ref="editableDiv"
          class="w-full min-h-[120px] border rounded p-2 focus:outline-none focus:ring-1 focus:ring-primary dark:bg-gray-800 dark:text-gray-200"
          @input="onInput"
          @keydown="handleKeyDown"
      ></div>
    </div>

    <!-- Footer buttons -->
    <div class="flex justify-end gap-2 mt-2">
      <button class="px-3 py-1 border rounded hover:bg-gray-200 dark:hover:bg-gray-700" @click="cancel">Cancel</button>
      <button class="px-3 py-1 bg-primary text-white rounded hover:bg-primary/90" @click="submit">Comment</button>
    </div>
  </div>
</template>
