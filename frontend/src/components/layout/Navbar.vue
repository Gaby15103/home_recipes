<script setup lang="ts">
import ModeToggle from "@/components/ModeToggle.vue";
import { Button } from "@/components/ui/button";
import { computed } from "vue";
import { useUserStore } from "@/stores/user";
import {DropdownMenu} from "@/components/ui/dropdown-menu";
import defaultAvatar from '@/assets/default.png';

const userStore = useUserStore();

// Computed properties for the user
const user = computed(() => userStore.user || null);

// Logout function
function logout() {
  userStore.logout(); // assuming you have a logout method in the store
}
</script>

<template>
  <nav class="flex items-center h-16 border-b px-6 justify-between">
    <RouterLink to="/" class="font-semibold text-lg">
      Home Recipes
    </RouterLink>

    <div class="flex items-center gap-4">
      <ModeToggle />


      <div class="flex items-center gap-4" v-if="user">
        <DropdownMenu>
          <DropdownMenuTrigger as-child>
            <Button class="flex items-center gap-2">
              <img :src="defaultAvatar" alt="avatar" class="w-8 h-8 rounded-full" />
              <span class="hidden sm:inline">{{ user.username }}</span>
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem>
              Profile
            </DropdownMenuItem>
            <DropdownMenuItem @click="logout">
              Logout
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
        </div>

      <div v-else>
        <RouterLink to="/login">
          <Button variant="outline">Login</Button>
        </RouterLink>
        <RouterLink to="/register">
          <Button class="ml-2">Register</Button>
        </RouterLink>
      </div>
    </div>
  </nav>
</template>
