<script setup lang="ts">
import { RouterLink } from "vue-router"
import { Button } from "@/components/ui/button"
import {useAuthStore} from "@/stores/auth";
import {ROUTES} from "@/router/routes.ts";

const authStore = useAuthStore();
</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center text-center"
       v-if="!authStore.isAuthenticated">
    <h1 class="text-4xl font-bold mb-4">
      🍳 Home Recipes
    </h1>

    <p class="text-muted-foreground mb-6 max-w-md">
      Browse delicious homemade recipes and manage your own cookbook.
    </p>

    <div class="flex gap-4" >
      <RouterLink :to="ROUTES.LOGIN">
        <Button>Login</Button>
      </RouterLink>
      <RouterLink :to="ROUTES.REGISTER">
        <Button variant="outline">Register</Button>
      </RouterLink>
    </div>
  </div>
</template>
