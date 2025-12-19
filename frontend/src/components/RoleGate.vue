<script setup lang="ts">
import { useUserStore } from "@/stores/user";
import { computed } from "vue";

const props = defineProps<{ role: string }>();
const userStore = useUserStore();

const hasAccess = computed(() => {
  const user = userStore.user;
  if (!user) return false;
  return user.roles.some(r => r.name === props.role);
});
</script>

<template>
  <template v-if="hasAccess">
    <slot />
  </template>
</template>
