<script setup lang="ts">
import { SidebarProvider } from '@/components/ui/sidebar';
import {useSidebarStore} from "@/stores/sidebar.ts";
import {ref} from "vue";

interface Props {
  variant?: 'header' | 'sidebar';
}
//TODO something to do with the props
defineProps<Props>();
const sidebarStore = useSidebarStore()

const isOpen = ref(true);
</script>

<template>
  <div v-if="variant === 'header'" class="flex min-h-screen w-full flex-col">
    <slot />
  </div>
  <SidebarProvider v-else :default-open="isOpen">
    <slot />
  </SidebarProvider>
</template>
