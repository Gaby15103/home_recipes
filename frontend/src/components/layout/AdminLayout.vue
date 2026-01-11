<script setup lang="ts">
import type {BreadcrumbItem} from '@/types';
import AdminSidebarLayout from "@/components/admin/AdminSidebarLayout.vue";

const breadcrumbs: BreadcrumbItem[] = [
  {
    title: 'Dashboard',
    href: '/admin/dashboard',
  },
];
</script>

<template>
  <AdminSidebarLayout :breadcrumbs="breadcrumbs">
    <slot/>
  </AdminSidebarLayout>
</template>
