<script setup lang="ts">
import {RouterLink} from "vue-router"
import ModeToggle from "@/components/ModeToggle.vue"
import NavLink from "@/components/NavLink.vue"
import {Button} from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"

import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuList,
} from "@/components/ui/navigation-menu"
import {useAuthStore} from "@/stores/auth.ts";
import router from "@/router";
import {storeToRefs} from "pinia";
import { ROUTES } from "@/router/routes";

const authStore = useAuthStore();
const { user, loading } = storeToRefs(authStore);

async function logout() {
  try {
    await authStore.logout()
    await router.push(ROUTES.HOME)
  } catch (e: any) {
    console.log(e.message)
  }
}
</script>

<template>
  <header
      class="sticky top-0 z-50 border-b
           bg-background/80 backdrop-blur
           supports-backdrop-filter:bg-background/60"
  >
    <div class="flex h-16 items-center justify-between px-6">
      <!-- Left -->
      <div class="flex items-center gap-8">
        <RouterLink
            :to="ROUTES.HOME"
            class="text-lg font-semibold tracking-tight"
        >
          Home Recipes
        </RouterLink>

        <NavigationMenu class="hidden md:flex">
          <NavigationMenuList class="gap-2">
            <NavigationMenuItem>
              <NavLink
                  :to="ROUTES.RECIPES"
                  class="text-sm font-medium text-muted-foreground hover:text-foreground transition"
              >
                Recipes
              </NavLink>
            </NavigationMenuItem>
            <NavigationMenuItem>
              <NavLink
                  to="/recipes"
                  class="text-sm font-medium text-muted-foreground hover:text-foreground transition"
              >
              </NavLink>
            </NavigationMenuItem>
          </NavigationMenuList>
        </NavigationMenu>
      </div>

      <!-- Right -->
      <div class="flex items-center gap-3">
        <ModeToggle/>

        <!-- Logged in -->
        <DropdownMenu v-if="user">
          <DropdownMenuTrigger as-child>
            <Button
                variant="ghost"
                class="flex items-center gap-2 px-2"
            >
              <img
                  :src="$apiUrl + user.avatar_url"
                  alt="avatar"
                  class="h-8 w-8 rounded-full"
              />
              <span class="hidden sm:inline text-sm font-medium">
                {{ user.username }}
              </span>
            </Button>
          </DropdownMenuTrigger>

          <DropdownMenuContent align="end" class="w-44">
            <DropdownMenuItem as-child>
              <RouterLink :to="ROUTES.HOME">
                Profile
              </RouterLink>
            </DropdownMenuItem>

            <DropdownMenuItem>
            <RouterLink :to="ROUTES.ADMIN.DASHBOARD">
              Admin Dashboard
            </RouterLink>
            </DropdownMenuItem>

            <DropdownMenuItem
                class="text-destructive focus:text-destructive"
                @click="logout"
            >
              Logout
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>

        <!-- Logged out -->
        <div v-else class="flex items-center gap-2">
          <RouterLink to="/login">
            <Button variant="outline">Login</Button>
          </RouterLink>
          <RouterLink to="/register">
            <Button>Register</Button>
          </RouterLink>
        </div>
      </div>
    </div>
  </header>
</template>
