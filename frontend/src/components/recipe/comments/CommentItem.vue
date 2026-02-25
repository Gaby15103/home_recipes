<script setup lang="ts">
import { ref } from "vue"
import type { RecipeComment } from "@/models/Recipe.ts"
import CommentEditor from "./CommentEditor.vue"
import MarkdownRenderer from "./MarkdownRenderer.vue"
import { useI18n } from "vue-i18n"
import {useAuthStore} from "@/stores/auth.ts";
const { t } = useI18n()
const authStore = useAuthStore();
const {
  comment,
  recipeId
} = defineProps<{
  comment: RecipeComment
  recipeId: string
}>()

const emit = defineEmits<{
  (e: "reply-posted", parentId: string, reply: RecipeComment): void
}>()


const replying = ref(false)
const replyContent = ref("")

function onReplyCreated(reply: RecipeComment) {
  emit("reply-posted", comment.id, reply)
  replying.value = false
}
function onChildReplyPosted(parentId: string, reply: RecipeComment) {
  emit("reply-posted", parentId, reply)
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
          v-if="authStore.user"
          @click="replying = !replying"
          class="text-sm text-blue-500 hover:underline mt-2"
      >
        {{ t("comments.Item.Reply") }}
      </button>

      <div v-if="replying" class="mt-2">
        <CommentEditor
            :recipe-id="recipeId"
            :parent="comment"
            v-model="replyContent"
            @posted="onReplyCreated"
            @cancel="replying = false"
        />
      </div>
    </div>

    <!-- Children Comments -->
    <div v-if="comment.children.length" class="ml-6 border-l pl-4 space-y-2">
      <CommentItem
          v-for="child in comment.children"
          :key="child.id"
          :comment="child"
          :recipe-id="recipeId"
          @reply-posted="onChildReplyPosted"
      />
    </div>
  </div>
</template>
