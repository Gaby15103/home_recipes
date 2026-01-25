<script setup lang="ts">
import type { RecipeComment } from "@/models/Recipe"
import CommentItem from "./CommentItem.vue"
const {
  comments,
  recipeId
} = defineProps<{
  comments: RecipeComment[]
  recipeId: string
}>()
function onReplyPosted(parentId: string, reply: RecipeComment) {
  const insert = (nodes: RecipeComment[]): boolean => {
    for (const node of nodes) {
      if (node.id === parentId) {
        node.children.push(reply)
        return true
      }
      if (insert(node.children)) return true
    }
    return false
  }

  insert(comments)
}

</script>

<template>
  <div class="space-y-2">
    <CommentItem
        v-for="comment in comments"
        :key="comment.id"
        :comment="comment"
        :recipe-id="recipeId"
        @reply-posted="onReplyPosted"
    />
  </div>
</template>
