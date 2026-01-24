<script setup lang="ts">
import { ref, defineProps, defineEmits, watch } from "vue";
import MarkdownToolbarModal from "./MarkdownToolbarModal.vue";
import type { RecipeCommentCreate } from "@/models/RecipeCreate";
import { addComment } from "@/api/recipe";
import { useAuthStore } from "@/stores/auth.ts";

const authStore = useAuthStore();

const {
  recipeId,
  parentId,
  modelValue
} = defineProps<{
  recipeId: string
  parentId?: string | null
  modelValue?: string
}>()

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void
  (e: "posted", comment: any): void
}>();

// Editor mode
const mode = ref<"markdown" | "rich">("markdown");

// Markdown helper modal
const showMarkdownHelper = ref(false);

// Local content copy
const content = ref(modelValue || "");

// Keep the content synced with parent
watch(() => modelValue, (v) => {
  content.value = v || "";
});

// Update content in parent
function updateContent() {
  emit("update:modelValue", content.value);
}

// Cancel button
function cancel() {
  content.value = "";
  updateContent();
}

// Submit comment
async function submit() {
  if (!content.value.trim() || !authStore.user) return;

  const comment: RecipeCommentCreate = {
    recipe_id: recipeId,
    parent_id: parentId || null,
    user_id: authStore.user.id,
    content: content.value,
  };

  try {
    const created = await addComment(recipeId, comment);
    emit("posted", created);
    content.value = "";
    updateContent();
  } catch (err) {
    console.error("Failed to post comment:", err);
    alert("Failed to post comment");
  }
}

// Rich Text Toolbar actions
function format(command: string, value?: string) {
  document.execCommand(command, false, value || null);
  updateContent();
}
</script>

<template>
  <div class="border rounded-lg p-3 bg-gray-50 dark:bg-gray-900 space-y-2">
    <!-- Header -->
    <div class="flex justify-between items-center mb-2">
      <div class="flex items-center gap-2 relative">
        <span class="font-semibold">Markdown Editor</span>
        <button
            @click="showMarkdownHelper = true"
            class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
        >ℹ️</button>
      </div>

      <button
          class="text-sm px-2 py-1 border rounded hover:bg-gray-200 dark:hover:bg-gray-700"
          @click="mode = mode === 'markdown' ? 'rich' : 'markdown'"
      >
        Switch to {{ mode === "markdown" ? "Rich Text" : "Markdown" }}
      </button>
    </div>

    <!-- Markdown Helper Modal -->
    <MarkdownToolbarModal v-model:open="showMarkdownHelper" />

    <!-- Editor Body -->
    <div v-if="mode === 'markdown'">
      <textarea
          v-model="content"
          placeholder="Write your comment..."
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
          class="w-full min-h-[120px] border rounded p-2 focus:outline-none focus:ring-1 focus:ring-primary dark:bg-gray-800 dark:text-gray-200"
          @input="updateContent"
          v-html="content"
      ></div>
    </div>

    <!-- Footer buttons -->
    <div class="flex justify-end gap-2 mt-2">
      <button
          class="px-3 py-1 border rounded hover:bg-gray-200 dark:hover:bg-gray-700"
          @click="cancel"
      >Cancel</button>
      <button
          class="px-3 py-1 bg-primary text-white rounded hover:bg-primary/90"
          @click="submit"
      >Comment</button>
    </div>
  </div>
</template>