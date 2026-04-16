<script setup lang="ts">

import {ROUTES} from "@/router/routes.ts";
import {Button} from "@/components/ui/button";
import Utensils from "@/components/icon/Utensils.vue";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu";
import {Heart, LayoutDashboard, LogOut, LucideUserRoundCog, User} from "lucide-vue-next";
import ModeToggle from "@/components/navigation/Navbar/ModeToggle.vue";
import {RouterLink} from "vue-router";
import LanguageChanger from "@/components/navigation/Navbar/LanguageChanger.vue";
import BellNotification from "@/components/navigation/Navbar/BellNotification.vue";
import {useAuthStore} from "@/stores/auth.ts";
import {storeToRefs} from "pinia";
import {useI18n} from "vue-i18n";
import {logout} from "@/api";

const authStore = useAuthStore()
const {user} = storeToRefs(authStore)
const {t} = useI18n()
</script>

<template>
  <div class="hidden md:flex items-center gap-2">
    <LanguageChanger/>
    <ModeToggle/>
    <div v-if="user">
      <BellNotification/>
    </div>
  </div>

  <div class="hidden md:block">
    <DropdownMenu v-if="user">
      <DropdownMenuTrigger as-child>
        <Button variant="ghost" class="flex items-center gap-2 px-2">
          <img :src="$apiUrl + user.avatar_url" alt="avatar" class="h-8 w-8 rounded-full object-cover"/>
          <span class="hidden sm:inline text-sm font-medium">{{ user.username }}</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent class="w-56" align="end">
        <DropdownMenuLabel class="font-normal">
          <div class="flex flex-col space-y-1">
            <p class="text-sm font-medium leading-none">{{ user.username }}</p>
            <p class="text-xs leading-none text-muted-foreground">{{ user.email }}</p>
          </div>
        </DropdownMenuLabel>
        <DropdownMenuSeparator/>
        <DropdownMenuItem as-child>
          <RouterLink :to="ROUTES.USER.PROFILE(user.id)" class="cursor-pointer w-full flex items-center">
            <User class="mr-2 h-4 w-4"/>
            {{ t('Profile') }}
          </RouterLink>
        </DropdownMenuItem>
        <DropdownMenuItem as-child>
          <RouterLink :to="ROUTES.USER.Favorite(user.id)" class="cursor-pointer w-full flex items-center">
            <Heart class="mr-2 h-4 w-4"/>
            {{ t('Favorite') }}
          </RouterLink>
        </DropdownMenuItem>
        <DropdownMenuItem as-child>
          <RouterLink :to="ROUTES.USER.SETTINGS" class="cursor-pointer w-full flex items-center">
            <LucideUserRoundCog class="mr-2 h-4 w-4"/>
            {{ t('Settings') }}
          </RouterLink>
        </DropdownMenuItem>
        <DropdownMenuItem as-child>
          <RouterLink :to="ROUTES.STUDIO.BASE" class="cursor-pointer w-full flex items-center">
            <Utensils class="mr-2 h-4 w-4"/> {{ t('Studio') }}
          </RouterLink>
        </DropdownMenuItem>
        <DropdownMenuItem v-if="authStore.hasRole('ADMIN') || authStore.hasRole('MODERATOR')" as-child>
          <RouterLink :to="ROUTES.ADMIN.DASHBOARD" class="cursor-pointer w-full flex items-center">
            <LayoutDashboard class="mr-2 h-4 w-4"/>
            {{ t('Admin Dashboard') }}
          </RouterLink>
        </DropdownMenuItem>
        <DropdownMenuSeparator/>
        <DropdownMenuItem @click="logout"
                          class="text-destructive focus:bg-destructive focus:text-destructive-foreground cursor-pointer">
          <LogOut class="mr-2 h-4 w-4"/>
          {{ t('Logout') }}
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>

    <div v-else class="flex items-center gap-2">
      <Button variant="ghost" as-child>
        <RouterLink to="/login">{{ t('Login') }}</RouterLink>
      </Button>
      <Button as-child>
        <RouterLink to="/register">{{ t('Register') }}</RouterLink>
      </Button>
    </div>
  </div>
</template>

<style scoped>

</style>