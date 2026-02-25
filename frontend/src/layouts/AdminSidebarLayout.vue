<script setup lang="ts">
import AppContent from '@/layouts/AdminContent.vue';
import AppShell from '@/layouts/AdminShell.vue';
import AdminSidebar from '@/components/navigation/Sidebar/AdminSidebar.vue';
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
  <AppShell id="qqc" variant="sidebar" class="h-screen">
    <AdminSidebar />

    <AppContent variant="sidebar" class="flex flex-col">
      <AppSidebarHeader
          :breadcrumbs="breadcrumbs"
          class="bg-appshell h-16 flex items-center px-6 border-b border-sidebar-border/70 rounded-tl-xl rounded-tr-xl"
      />

      <!-- Scrollable slot content -->
      <div class="flex-1 min-h-0 overflow-y-auto px-6 py-4 scrollbar-hide">
        <slot />
      </div>
    </AppContent>
  </AppShell>
</template>
<style lang="css" scoped>
/* Hide scrollbar but still allow scrolling */
.scrollbar-hide::-webkit-scrollbar {
  display: none;
}
.scrollbar-hide {
  -ms-overflow-style: none; /* IE and Edge */
  scrollbar-width: none; /* Firefox */
}
</style>

