<script setup lang="ts">
import UserInfo from '@/components/UserInfo.vue';
import {
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
} from '@/components/ui/dropdown-menu';
import type { User } from '@/models/User';
import { LogOut, Settings } from 'lucide-vue-next';
import { useAuthStore } from "@/stores/auth.ts";
import NavLink from "@/components/NavLink.vue";
import {Button} from "@/components/ui/button";

const authStore = useAuthStore();
interface Props {
  user: User;
}

const handleLogout = () => {
  authStore.logout()
};

defineProps<Props>();
</script>

<template>
  <DropdownMenuLabel class="p-0 font-normal">
    <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
      <UserInfo :user="user" :show-email="true" />
    </div>
  </DropdownMenuLabel>
  <DropdownMenuSeparator/>
  <DropdownMenuGroup>
    <DropdownMenuItem :as-child="true">
      <NavLink class="block w-full cursor-pointer" to="/admin/user/edit/profile">
        <Settings  class="mr-2 h-4 w-4" />
        Settings
      </NavLink>
    </DropdownMenuItem>
  </DropdownMenuGroup>
  <DropdownMenuSeparator />
  <DropdownMenuItem :as-child="true">
    <Button
        variant="ghost"
        class="flex items-center gap-2 w-full justify-start px-2 py-1.5 text-sm font-medium text-muted-foreground hover:text-foreground"
        @click="handleLogout"
        as="button"
        data-test="logout-button"
    >
      <LogOut class="h-4 w-4" />
      Log out
    </Button>
  </DropdownMenuItem>
</template>
