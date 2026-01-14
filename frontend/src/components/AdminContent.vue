<script setup lang="ts">
import { SidebarInset } from '@/components/ui/sidebar';
import { computed } from 'vue';

interface Props {
  variant?: 'header' | 'sidebar';
  class?: string;
}

const props = defineProps<Props>();
const className = computed(() => props.class);
</script>
<template>
  <SidebarInset
      v-if="props.variant === 'sidebar'"
      :class="[className, 'flex flex-col flex-1 min-h-0 ']"
  >
    <slot />
  </SidebarInset>

  <main
      v-else
      class="mx-auto flex flex-1 min-h-0 w-full max-w-7xl flex-col gap-4 rounded-xl"
      :class="className"
  >
    <slot />
  </main>
</template>


