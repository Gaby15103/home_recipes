<script setup lang="ts">
import AppContent from '@/components/AdminContent.vue';
import AppShell from '@/components/AdminShell.vue';
import AdminSidebar from '@/components/AdminSidebar.vue';
import AdminSidebarHeader from '@/components/AdminSidebarHeader.vue';
import type { BreadcrumbItemType } from '@/types';

interface Props {
  breadcrumbs?: BreadcrumbItemType[];
}

withDefaults(defineProps<Props>(), {
  breadcrumbs: () => [],
});
</script>

<template>
  <AppShell variant="sidebar">
    <AdminSidebar />
    <AppContent variant="sidebar" class="overflow-x-hidden">
      <AdminSidebarHeader :breadcrumbs="breadcrumbs" />
      <slot />
    </AppContent>
  </AppShell>
</template>
