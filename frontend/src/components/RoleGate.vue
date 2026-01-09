<script setup lang="ts">
import { useAuthStore } from "@/stores/auth";
import { computed } from "vue";

const props = defineProps<{ role: string }>();
const authStore = useAuthStore();

const hasAccess = computed(() => {
  if (!authStore.isAuthenticated) return false;
  return authStore.hasRole(props.role)
});
</script>

<template>
  <template v-if="hasAccess">
    <slot />
  </template>
</template>
