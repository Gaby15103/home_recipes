<script setup lang="ts">
import { computed } from "vue";
import { useAuthStore } from "@/stores/auth.ts";
import { RouterLink, useRouter } from "vue-router";
import ModeToggle from "@/components/ModeToggle.vue";
import { Button } from "@/components/ui/button";

const authStore = useAuthStore();
const router = useRouter();

const user = computed(() => authStore.user || null);

async function logout() {
  try {
    await authStore.logout();
    await router.push("/login");
  } catch (e: any) {
    console.error(e.message);
  }
}
</script>

<template>
  <header class="h-16 border-b bg-background/80 backdrop-blur sticky top-0 z-50 flex items-center justify-between px-6">
    <!-- Left -->
    <div class="flex items-center gap-6">
      <RouterLink to="/admin/dashboard" class="text-lg font-bold">
        Admin Dashboard
      </RouterLink>
    </div>

    <!-- Right -->
    <div class="flex items-center gap-4">
      <ModeToggle />

      <div v-if="user" class="flex items-center gap-2">
        <Button variant="ghost" class="flex items-center gap-2 p-2">
          <img
              :src="$apiUrl + user.avatar_url"
              alt="avatar"
              class="h-8 w-8 rounded-full"
          />
          <span class="hidden sm:inline">{{ user.username }}</span>
        </Button>
        <Button variant="destructive" @click="logout">Logout</Button>
      </div>

      <div v-else class="flex gap-2">
        <RouterLink to="/login">
          <Button variant="outline">Login</Button>
        </RouterLink>
        <RouterLink to="/register">
          <Button>Register</Button>
        </RouterLink>
      </div>
    </div>
  </header>
</template>
