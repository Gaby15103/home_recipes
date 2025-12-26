<script setup lang="ts">
import ModeToggle from "@/components/ModeToggle.vue";
import {Button} from "@/components/ui/button";
import {computed} from "vue";
import {useUserStore} from "@/stores/user";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import NavLink from "@/components/NavLink.vue";

const API_URL = import.meta.env.VITE_API_URL;

const userStore = useUserStore();

// Computed properties for the user
const user = computed(() => userStore.user || null);

function logout() {
  userStore.logout();
}
</script>

<template>
  <nav class="flex items-center h-16 border-b px-6 justify-between">
    <div class="flex items-center gap-6">
      <RouterLink to="/" class="font-semibold text-lg">
        Home Recipes
      </RouterLink>
      <div class="hidden md:flex items-center gap-4">
        <NavLink to="/recipes" class="text-sm font-medium text-muted-foreground hover:text-foreground">
          Recipes
        </NavLink>

        <NavLink to="/Tags" class="text-sm font-medium text-muted-foreground hover:text-foreground">
          Tags
        </NavLink>
      </div>
    </div>


    <div class="flex items-center gap-4">
      <ModeToggle/>


      <div class="flex items-center gap-4" v-if="user">
        <DropdownMenu>
          <DropdownMenuTrigger as-child>
            <Button class="flex items-center gap-2">
              <img :src="API_URL+user.avatar_url" alt="avatar" class="w-8 h-8 rounded-full"/>
              <span class="hidden sm:inline">{{ user.username }}</span>
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem as-child>
              <RouterLink to="/Profile">
                Profile
              </RouterLink>
            </DropdownMenuItem>
            <DropdownMenuItem v-if="userStore.hasRole('ADMIN')" as-child>
              <RouterLink to="/Admin">
                Admin
              </RouterLink>
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
