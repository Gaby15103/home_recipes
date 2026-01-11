<script setup lang="ts">
import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar';
import { useActiveUrl } from '@/composables/useActiveUrl';
import { type NavItem } from '@/types';
import NavLink from "@/components/NavLink.vue";

defineProps<{
  items: NavItem[];
}>();

const { urlIsActive } = useActiveUrl();
</script>

<template>
  <SidebarGroup class="px-2 py-0">
    <SidebarGroupLabel>Platform</SidebarGroupLabel>
    <SidebarMenu>
      <SidebarMenuItem v-for="item in items" :key="item.title">
        <SidebarMenuButton
            as-child
            :is-active="urlIsActive(item.href)"
            :tooltip="item.title"
        >
          <NavLink :to="item.href">
            <component :is="item.icon"/>
            <span>{{item.title}}</span>
          </NavLink>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  </SidebarGroup>
</template>
