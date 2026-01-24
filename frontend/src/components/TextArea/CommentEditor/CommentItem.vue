<script setup lang="ts">
import { ref } from "vue"
import type { RecipeComment, RecipeCommentCreate } from "@/models/Recipe"
import CommentEditor from "./CommentEditor.vue"
import MarkdownRenderer from "./MarkdownRenderer.vue"
import {useAuthStore} from "@/stores/auth.ts";

const authStore = useAuthStore();
const {
  comment,
  recipeId
} = defineProps<{
  comment: RecipeComment
  recipeId: string
}>()

const replying = ref(false)
const replyContent = ref("")

const submitReply = () => {
  if (!replyContent.value.trim() || !authStore.user) return
  // emit reply to parent component
  comment.children.push({
    id: crypto.randomUUID(),
    user_id: authStore.user.id,
    username: authStore.user.username,
    content: replyContent.value,
    created_at: new Date().toISOString(),
    parent_id: comment.id,
    children: [],
  })
  replyContent.value = ""
  replying.value = false
}
</script>

<template>
  <div class="space-y-2">
    <!-- Comment Body -->
    <div class="p-3 border rounded-lg bg-gray-50 dark:bg-gray-900">
      <div class="flex justify-between items-center">
        <p class="font-semibold">{{ comment.username }}</p>
        <p class="text-xs text-gray-500">{{ new Date(comment.created_at).toLocaleString() }}</p>
      </div>

      <MarkdownRenderer :content="comment.content" />

      <button
          @click="replying = !replying"
          class="text-sm text-blue-500 hover:underline mt-2"
      >
        Reply
      </button>

      <div v-if="replying" class="mt-2">
        <CommentEditor v-model="replyContent" placeholder="Write a reply..." @submit="submitReply" />
      </div>
    </div>

    <!-- Children Comments -->
    <div v-if="comment.children.length" class="ml-6 border-l pl-4 space-y-2">
      <CommentItem
          v-for="child in comment.children"
          :key="child.id"
          :comment="child"
          :recipe-id="recipeId"
      />
    </div>
  </div>
</template>
