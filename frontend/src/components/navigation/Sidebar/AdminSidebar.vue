<script setup lang="ts">
import NavFooter from '@/components/navigation/Sidebar/NavFooter.vue';
import NavMain from '@/components/navigation/Sidebar/NavMain.vue';
import NavUser from '@/components/navigation/Sidebar/NavUser.vue';
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
import {Book, Folder, Home, LayoutGrid} from 'lucide-vue-next';
import AppLogo from '../../common/AppLogo.vue';
import NavLink from "@/components/navigation/NavLink.vue";
import { ROUTES } from "@/router/routes.ts";

const mainNavItems: NavItem[] = [
  {
    title: 'Return to the client',
    href: ROUTES.HOME,
    icon: Home,
  },
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
