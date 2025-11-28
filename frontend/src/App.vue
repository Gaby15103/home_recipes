<script setup lang="ts">
import { ref, onMounted } from "vue";

// shadcn-vue components
import { Button } from "@/components/ui/button";
import { NavigationMenu, NavigationMenuItem, NavigationMenuList } from "@/components/ui/navigation-menu";
import { Switch } from "@/components/ui/switch";
import ModeToggle from "@/components/ModeToggle.vue";
</script>

<template>
  <div class="min-h-screen bg-background text-foreground">
    <!-- NAVBAR -->
    <header class="border-b bg-card/60 backdrop-blur supports-[backdrop-filter]:bg-card/40 sticky top-0 z-50">
      <div class="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">

        <!-- Logo -->
        <h1 class="text-xl font-bold">My Website</h1>

        <!-- Navigation menu -->
        <NavigationMenu>
          <NavigationMenuList class="flex gap-4">
            <NavigationMenuItem>
              <a class="hover:text-primary transition" href="#">Home</a>
            </NavigationMenuItem>
            <NavigationMenuItem>
              <a class="hover:text-primary transition" href="#">About</a>
            </NavigationMenuItem>
            <NavigationMenuItem>
              <a class="hover:text-primary transition" href="#">Contact</a>
            </NavigationMenuItem>
          </NavigationMenuList>
        </NavigationMenu>

        <!-- Dark mode toggle -->
        <div class="flex items-center gap-3">
          <ModeToggle></ModeToggle>
        </div>
      </div>
    </header>

    <!-- CONTENT -->
    <main class="max-w-5xl mx-auto px-6 py-10">
      <h2 class="scroll-m-20 text-3xl font-semibold tracking-tight mb-4">
        Welcome 👋
      </h2>

      <p class="leading-7 opacity-80 mb-8">
        This is your Vue + Vite + Tailwind + shadcn-vue starter layout.
      </p>

      <!-- Example shadcn-vue card -->
      <div class="border bg-card p-6 rounded-xl shadow-sm">
        <p class="opacity-80 mb-4">
          This card uses shadcn-vue + your Tailwind theme.
        </p>
        <Button>Click me</Button>
      </div>
    </main>
  </div>
</template>

<style scoped>
/* Optional entry animations */
@keyframes fade-in-up {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.fade-in-up {
  animation: fade-in-up .4s ease;
}
</style>
