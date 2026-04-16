<script setup lang="ts">
import { RouterView, useRoute } from "vue-router"
import PublicLayout from "@/layouts/PublicLayout.vue"
import AdminLayout from "@/layouts/AdminLayout.vue"
import StudioLayout from "@/layouts/StudioLayout.vue"
import { computed } from "vue"

const route = useRoute()

const layout = computed(() => {
  switch (route.meta.layout) {
    case 'AdminLayout':
      return AdminLayout
    case 'StudioLayout':
      return StudioLayout
    default:
      return PublicLayout
  }
})
</script>

<template>
  <component :is="layout">
    <RouterView :key="route.fullPath"/>
  </component>
</template>