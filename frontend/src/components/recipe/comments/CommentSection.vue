<script setup lang="ts">

import {Button} from "@/components/ui/button";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card";
import CommentThread from "@/components/recipe/comments/CommentThread.vue";
</script>

<template>
  <Card class="shadow-md">
    <CardHeader>
      <CardTitle>
        {{ t("recipe.comments.title") }}
      </CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <div class="space-y-2">
        <textarea
            v-model="newComment.content"
            :placeholder="t('recipe.comments.placeholder')"
            class="w-full border rounded-lg p-2"
        />
        <Button @click="postComment">
          {{ t("recipe.comments.post") }}
        </Button>
      </div>
      <div class="space-y-4">
        <CommentThread
            :comments="comments"
            :recipe-id="recipe!.id"
        />
      </div>
    </CardContent>
  </Card>
</template>

<style scoped>

</style>