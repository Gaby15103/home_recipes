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
import { ROUTES } from "@/router/routes";

const mainNavItems: NavItem[] = [
  {
    title: 'Dashboard',
    href: ROUTES.ADMIN.DASHBOARD,
    icon: LayoutGrid,
  },
  {
    title: 'Recipes',
    href: "",
    icon: Book,
    subNavItems: [
      {
        title: 'Dashboard',
        href: ROUTES.ADMIN.RECIPE.BASE,
      },
      {
        title: 'List',
        href: ROUTES.ADMIN.RECIPE.LIST,
      },
      {
        title: 'Create',
        href: ROUTES.ADMIN.RECIPE.CREATE,
      },
    ]
  }
];

const footerNavItems: NavItem[] = [
  {
    title: 'Github Repo',
    href: ROUTES.GITHUB_REPO,
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
