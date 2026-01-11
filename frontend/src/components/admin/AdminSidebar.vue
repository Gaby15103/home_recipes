<script setup lang="ts">
import {
  Sidebar,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarProvider
} from "@/components/ui/sidebar";
import { RouterLink } from "vue-router";
import { ref } from "vue";

const isOpen = ref(true);
</script>

<template>
  <SidebarProvider :open="isOpen">
    <Sidebar
        side="left"
        class=" border-r bg-background flex flex-col top-16 w-64 transition-all"
    >
      <SidebarMenu>
        <SidebarMenuItem>
          <RouterLink to="/admin/dashboard" class="hover:bg-gray-100 rounded px-2 py-1 block">
            Dashboard
          </RouterLink>
        </SidebarMenuItem>

        <SidebarMenuSub title="Recipes">
          <SidebarMenuItem>
            <RouterLink to="/admin/recipes" class="hover:bg-gray-100 rounded px-2 py-1 block">
              List
            </RouterLink>
          </SidebarMenuItem>
          <SidebarMenuItem>
            <RouterLink to="/admin/recipe/create" class="hover:bg-gray-100 rounded px-2 py-1 block">
              Create
            </RouterLink>
          </SidebarMenuItem>
          <SidebarMenuItem>
            <RouterLink to="/admin/recipe/Categories" class="hover:bg-gray-100 rounded px-2 py-1 block">
              Categories
            </RouterLink>
          </SidebarMenuItem>
        </SidebarMenuSub>

        <SidebarMenuSub title="Users">
          <SidebarMenuItem>
            <RouterLink to="/admin/users" class="hover:bg-gray-100 rounded px-2 py-1 block">
              User List
            </RouterLink>
          </SidebarMenuItem>
        </SidebarMenuSub>

        <SidebarMenuSub title="Settings">
          <SidebarMenuItem>
            <RouterLink to="/admin/settings" class="hover:bg-gray-100 rounded px-2 py-1 block">
              Site Settings
            </RouterLink>
          </SidebarMenuItem>
        </SidebarMenuSub>
      </SidebarMenu>
    </Sidebar>
  </SidebarProvider>
</template>
