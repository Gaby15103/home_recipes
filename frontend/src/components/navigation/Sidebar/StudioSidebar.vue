<script setup lang="ts">
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar
} from '@/components/ui/sidebar';
import { BarChart3, BookOpen, Home, LayoutGrid } from 'lucide-vue-next';
import AppLogo from '../../common/AppLogo.vue';
import NavLink from "@/components/navigation/NavLink.vue";
import { ROUTES } from "@/router/routes.ts";
import NavMain from "@/components/navigation/Sidebar/NavMain.vue";
import type { NavItem } from "@/types";

const { state } = useSidebar();

const studioNav: NavItem[] = [
  {
    title: 'Dashboard',
    icon: LayoutGrid,
    href: ROUTES.STUDIO.BASE
  },
  {
    title: 'My Recipes',
    icon: BookOpen,
    href: ROUTES.STUDIO.MY_RECIPES,
    subNavItems: [
      { title: 'All Content', href: ROUTES.STUDIO.MY_RECIPES },
      { title: 'New Recipe', href: ROUTES.STUDIO.CREATE }
    ]
  },
  {
    title: 'Analytics',
    icon: BarChart3,
    href: ROUTES.STUDIO.ANALYTICS
  }
];
</script>

<template>
  <Sidebar collapsible="icon" class="border-r border-neutral-800 bg-[#0a0a0a]!">

    <SidebarHeader
        class="h-16 border-b border-neutral-800/50 flex flex-col justify-center transition-all duration-300 group-data-[collapsible=icon]:px-0 px-6"
    >
      <NavLink
          :to="ROUTES.STUDIO.BASE"
          class="flex items-center gap-3 overflow-hidden group-data-[collapsible=icon]:justify-center"
      >
        <AppLogo class="shrink-0 h-8 w-8 rounded shadow-[0_0_20px_rgba(var(--primary),0.3)]"/>
        <span
            v-if="state !== 'collapsed'"
            class="font-black tracking-tighter text-white truncate animate-in fade-in slide-in-from-left-2 duration-300"
        >
          STUDIO
        </span>
      </NavLink>
    </SidebarHeader>

    <SidebarContent class="pt-6 transition-all duration-300 group-data-[collapsible=icon]:px-0 px-3">
      <NavMain :items="studioNav" />
    </SidebarContent>

    <SidebarFooter class="p-4 border-t border-neutral-800 group-data-[collapsible=icon]:px-0">
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton
              as-child
              class="text-neutral-500 hover:text-white h-11 group-data-[collapsible=icon]:justify-center"
              tooltip="Exit Studio"
          >
            <NavLink :to="ROUTES.HOME" class="flex items-center gap-3 px-2 group-data-[collapsible=icon]:px-0 group-data-[collapsible=icon]:justify-center">
              <Home class="h-5 w-5 shrink-0"/>
              <span v-if="state !== 'collapsed'" class="text-xs font-bold uppercase tracking-widest">Exit Studio</span>
            </NavLink>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarFooter>
  </Sidebar>
</template>