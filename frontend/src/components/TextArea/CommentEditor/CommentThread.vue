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
</script>

<template>
  <div class="space-y-2">
    <CommentItem
        v-for="comment in comments"
        :key="comment.id"
        :comment="comment"
        :recipe-id="recipeId"
    />
  </div>
</template>
