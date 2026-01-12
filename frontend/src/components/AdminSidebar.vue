<script setup lang="ts">
import NavFooter from '@/components/NavFooter.vue';
import NavMain from '@/components/NavMain.vue';
import NavUser from '@/components/NavUser.vue';
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar';
import { type NavItem } from '@/types';
import {Book, Folder, LayoutGrid} from 'lucide-vue-next';
import AppLogo from './AppLogo.vue';
import NavLink from "@/components/NavLink.vue";

const mainNavItems: NavItem[] = [
  {
    title: 'Dashboard',
    href: "/admin/dashboard",
    icon: LayoutGrid,
  },
  {
    title: 'Recipes',
    href: "",
    icon: Book,
    subNavItems: [
      {
        title: 'Dashboard',
        href: "/admin/recipe",
      },
      {
        title: 'List',
        href: "/admin/recipe/list",
      },
      {
        title: 'Create',
        href: "/admin/recipe/create",
      },
    ]
  }
];

const footerNavItems: NavItem[] = [
  {
    title: 'Github Repo',
    href: 'https://github.com/Gaby15103/home_recipes',
    icon: Folder,
  }
];
</script>

<template>
  <Sidebar collapsible="icon" variant="inset">
    <SidebarHeader>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton size="lg" as-child>
            <NavLink to="/admin/dashboard">
              <AppLogo/>
            </NavLink>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarHeader>

    <SidebarContent>
      <NavMain :items="mainNavItems" />

    </SidebarContent>

    <SidebarFooter>
      <NavFooter :items="footerNavItems" />
      <NavUser />
    </SidebarFooter>
  </Sidebar>
  <slot />
</template>
