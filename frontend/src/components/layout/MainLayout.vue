<script setup>
import MainNavbar from "../MainNavbar/MainNavbar.vue";

</script>

<template>
  <div class="min-h-screen bg-background text-foreground">
    <MainNavbar/>
    <main class="container mx-auto p-6">
      <slot/>
    </main>
  </div>
</template>