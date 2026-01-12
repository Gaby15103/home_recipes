<script setup lang="ts">
import type {BreadcrumbItem} from '@/types';
import AdminSidebarLayout from "@/components/admin/AdminSidebarLayout.vue";
import {computed} from "vue";
import {useRoute} from "vue-router";

const route = useRoute();

const breadcrumbs = computed(() => {
  return (route.meta.breadcrumb as BreadcrumbItem[]) ?? []
})

</script>

<template>
  <AdminSidebarLayout :breadcrumbs="breadcrumbs">
    <slot/>
  </AdminSidebarLayout>
</template>
