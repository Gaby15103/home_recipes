<script setup lang="ts">
import { RouterView, useRoute } from "vue-router"
import MainLayout from "@/components/layout/MainLayout.vue"
import AdminLayout from "@/components/layout/AdminLayout.vue"

const route = useRoute()
</script>

<template>
  <!-- Use Vue's dynamic component -->
  <component :is="route.meta.layout === 'AdminLayout' ? AdminLayout : MainLayout">
    <RouterView />
  </component>
</template>
