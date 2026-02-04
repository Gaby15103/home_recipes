<script setup lang="ts">
import Breadcrumbs from '@/components/Breadcrumbs.vue';
import { SidebarTrigger } from '@/components/ui/sidebar';
import type { BreadcrumbItemType } from '@/types';
import {Button} from "@/components/ui/button";
import { Bell } from 'lucide-vue-next';
import UserInfo from "@/components/UserInfo.vue";
import { useAuthStore } from "@/stores/auth.ts";
import {ref} from "vue";
import {Input} from "@/components/ui/input";
import LanguageChanger from "@/components/MainNavbar/LanguageChanger.vue";

const authStore = useAuthStore();

withDefaults(
    defineProps<{
      breadcrumbs?: BreadcrumbItemType[];
    }>(),
    {
      breadcrumbs: () => [],
    },
);

const search = ref('')

const notificationsCount = ref(2) // mock for now
</script>

<template>
  <header
      class="sticky top-0 z-20 flex h-16 shrink-0 items-center gap-2 border-b border-sidebar-border/70 px-6 transition-[width,height]
       ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12 md:px-4 justify-between"
  >
    <div class="flex items-center gap-2">
      <SidebarTrigger class="-ml-1" />
      <template v-if="breadcrumbs && breadcrumbs.length > 0">
        <Breadcrumbs :breadcrumbs="breadcrumbs" />
      </template>
    </div>
    <div class="flex items-center gap-3">

      <div class="relative hidden md:block">
        <Search class="absolute left-2 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <!-- TODO make a search bar for the web site with special search feature example by tags a recipe a suer or things like that -->
        <Input
            v-model="search"
            placeholder="Search recipes..."
            class="pl-8 h-9 w-[220px]"
        />
      </div>

      <Button variant="ghost" size="icon" class="relative">
        <Bell class="h-5 w-5" />

        <span
            v-if="notificationsCount > 0"
            class="absolute -top-1 -right-1 flex h-4 min-w-4 items-center justify-center
                 rounded-full bg-red-500 px-1 text-[10px] font-medium text-white"
        >
          {{ notificationsCount }}
        </span>
      </Button>
      <LanguageChanger/>

      <!-- User menu (placeholder for now) -->
      <Button variant="ghost" class="flex items-center gap-2">
        <UserInfo class="h-5 w-5" :user="authStore.user"/>
      </Button>

    </div>
  </header>
</template>
