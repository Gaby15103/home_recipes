<script setup lang="ts">
import AppContent from '@/layouts/AdminContent.vue';
import AppShell from '@/layouts/AdminShell.vue';
import AppSidebar from '@/components/navigation/Sidebar/AdminSidebar.vue';
import AppSidebarHeader from '@/components/navigation/Sidebar/AppSidebarHeader.vue';
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
