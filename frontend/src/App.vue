<script setup lang="ts">
import { RouterView, useRoute } from "vue-router"
import PublicLayout from "@/layouts/PublicLayout.vue"
import AdminLayout from "@/layouts/AdminLayout.vue"

const route = useRoute()
</script>

<template>
  <!-- Use Vue's dynamic component -->
  <component :is="route.meta.layout === 'AdminLayout' ? AdminLayout : PublicLayout">
    <RouterView />
  </component>
</template>
