<script setup lang="ts">
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
} from "@/components/ui/sidebar"
import {Users, Shield, Settings, PanelLeft} from "lucide-vue-next"
import { useSidebarStore } from '@/stores/sidebar'
import {Button} from "@/components/ui/button";
import {cn} from "@/lib/utils.ts";

const sidebarStore = useSidebarStore()
</script>

<template>
  <Sidebar side="left" variant="floating" class="top-16 h-[calc(100vh-4rem)]">
    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupLabel class="flex justify-between items-center">
          <div>Admin Panel</div>
          <Button
                data-sidebar="trigger"
                data-slot="sidebar-trigger"
                variant="ghost"
                size="icon"
                :class="cn('h-7 w-7')"
                @click="sidebarStore.toggle()"
          >
            <PanelLeft />
          </Button>
        </SidebarGroupLabel>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton as-child>
              <RouterLink to="/admin/users">
                <Users />
                <span>Users</span>
              </RouterLink>
            </SidebarMenuButton>
          </SidebarMenuItem>

          <SidebarMenuItem>
            <SidebarMenuButton as-child>
              <RouterLink to="/admin/roles">
                <Shield />
                <span>Roles</span>
              </RouterLink>
            </SidebarMenuButton>
          </SidebarMenuItem>

          <SidebarMenuItem>
            <SidebarMenuButton as-child>
              <RouterLink to="/admin/settings">
                <Settings />
                <span>Settings</span>
              </RouterLink>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroup>
    </SidebarContent>
  </Sidebar>
</template>
