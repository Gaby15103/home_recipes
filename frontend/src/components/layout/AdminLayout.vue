<script setup lang="ts">
import type {BreadcrumbItem} from '@/types';
import AdminSidebarLayout from "@/components/admin/AdminSidebarLayout.vue";
import {computed} from "vue";
import {useRoute} from "vue-router";

const route = useRoute();

const breadcrumbs = computed(() => {
  const metaBreadcrumbs = route.meta.breadcrumb as BreadcrumbItem[] || []

  return metaBreadcrumbs.map(item => {
    let title = item.title

    // Check if the title is a param placeholder like :id
    if (title.startsWith(':')) {
      const paramName = title.substring(1) // gets "id"
      title = route.params[paramName] || title
    }

    return { ...item, title }
  })
})

</script>

<template>
  <AdminSidebarLayout :breadcrumbs="breadcrumbs">
    <slot/>
  </AdminSidebarLayout>
</template>
