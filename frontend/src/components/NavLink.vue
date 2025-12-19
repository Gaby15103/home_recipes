<script setup lang="ts">
import { RouterLink } from 'vue-router'

defineProps<{
  to: string
  exact?: boolean
}>()
</script>

<template>
  <RouterLink
      :to="to"
      v-slot="{ isActive, isExactActive }"
  >
    <span
        :class="[
        'text-sm font-medium transition-colors cursor-pointer',
        (exact ? isExactActive : isActive)
          ? 'text-foreground'
          : 'text-muted-foreground hover:text-foreground'
      ]"
    >
      <slot />
    </span>
  </RouterLink>
</template>
