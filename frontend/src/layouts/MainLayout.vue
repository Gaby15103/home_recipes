<script setup lang="ts">
import AppLayout from '@/layouts/AppSidebarLayout.vue';
import type { BreadcrumbItemType } from '@/types';
import Footer from "@/components/navigation/Footer.vue";

interface Props {
  breadcrumbs?: BreadcrumbItemType[];
}

withDefaults(defineProps<Props>(), {
  breadcrumbs: () => [],
});
</script>

<template>
  <AppLayout :breadcrumbs="breadcrumbs">
    <slot />
  </AppLayout>
</template>
