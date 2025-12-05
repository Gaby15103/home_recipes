<template>
  <div class="min-h-screen bg-background text-foreground">
    <Navbar />

    <main class="container mx-auto p-6">
      <slot />
    </main>
  </div>
</template>

<script setup>
import Navbar from "./Navbar.vue";
</script>
