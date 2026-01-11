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
import {
  Collapsible,
  CollapsibleTrigger,
  CollapsibleContent
} from "@/components/ui/collapsible";
import { Book } from 'lucide-vue-next'


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
            <Collapsible class="group/collapsible">
              <CollapsibleTrigger as-child>
                <SidebarMenuButton>
                  <Book />
                  <span>Recipes</span>
                </SidebarMenuButton>
              </CollapsibleTrigger>
              <CollapsibleContent>
                <SidebarMenuSub>
                  <SidebarMenuItem>
                    <SidebarMenuButton as-child>
                      <RouterLink to="/admin/recipes">
                        <span class="ml-[10%]">List</span>
                      </RouterLink>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                  <SidebarMenuItem>
                    <SidebarMenuButton as-child>
                      <RouterLink to="/admin/recipe/create">
                        <span class="ml-[10%]">Create</span>
                      </RouterLink>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                  <SidebarMenuItem>
                    <SidebarMenuButton as-child>
                      <RouterLink to="/admin/recipe/edit">
                        <span class="ml-[10%]">Edit</span>
                      </RouterLink>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                  <SidebarMenuItem>
                    <SidebarMenuButton as-child>
                      <RouterLink to="/admin/recipe/categories">
                        <span class="ml-[10%]">Categories</span>
                      </RouterLink>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                </SidebarMenuSub>
              </CollapsibleContent>
            </Collapsible>
          </SidebarMenuItem>

          <SidebarMenuItem>
            <SidebarMenuButton as-child>
              <RouterLink to="/">
                <Users />
                <span>Users</span>
              </RouterLink>
            </SidebarMenuButton>
          </SidebarMenuItem>

          <SidebarMenuItem>
            <SidebarMenuButton as-child>
              <RouterLink to="/">
                <Shield />
                <span>Roles</span>
              </RouterLink>
            </SidebarMenuButton>
          </SidebarMenuItem>

          <SidebarMenuItem>
            <SidebarMenuButton as-child>
              <RouterLink to="/">
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
