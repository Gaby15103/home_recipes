<script setup lang="ts">
import AppContent from '@/components/AdminContent.vue';
import AppShell from '@/components/AdminShell.vue';
import AppSidebar from '@/components/AdminSidebar.vue';
import AppSidebarHeader from '@/components/AppSidebarHeader.vue';
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
    <AppSidebar />
    <AppContent variant="sidebar">
      <AppSidebarHeader :breadcrumbs="breadcrumbs"/>
      <slot/>
    </AppContent>
  </AppShell>
</template>
