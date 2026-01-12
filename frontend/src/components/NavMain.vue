<script setup lang="ts">
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible';
import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem
} from '@/components/ui/sidebar';

import { type NavItem } from '@/types';
import { useActiveUrl } from '@/composables/useActiveUrl';
import NavLink from "@/components/NavLink.vue";

defineProps<{
  items: NavItem[];
}>();

const { urlIsActive } = useActiveUrl();

function isMenuItemActive(item: NavItem) {
  if (item.href && urlIsActive(item.href)) {
    return true;
  }

  if (item.subNavItems) {
    return item.subNavItems.some(sub => urlIsActive(sub.href));
  }

  return false;
}
</script>

<template>
  <SidebarGroupLabel>Platform</SidebarGroupLabel>

  <SidebarGroup class="px-2 py-0">
    <SidebarMenu>
      <Collapsible
          v-for="item in items"
          :key="item.title"
          :default-open="isMenuItemActive(item)"
          class="group/collapsible"
      >
        <SidebarMenuItem class="rounded-sm">
          <SidebarMenuButton
              v-if="!item.subNavItems"
              as-child
              :is-active="urlIsActive(item.href)"
          >
            <NavLink :to="item.href!">
              <component v-if="item.icon" :is="item.icon" />
              <span>{{ item.title }}</span>
            </NavLink>
          </SidebarMenuButton>

          <CollapsibleTrigger v-else as-child>
            <SidebarMenuButton>
              <div class="flex w-full items-center justify-between">
                <div class="flex items-center gap-2">
                  <component v-if="item.icon" :is="item.icon" />
                  <span>{{ item.title }}</span>
                </div>

                <svg
                    class="arrow h-4 w-4 transition-transform duration-300"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                  <polyline points="6 9 12 15 18 9" />
                </svg>
              </div>
            </SidebarMenuButton>
          </CollapsibleTrigger>

          <CollapsibleContent
              v-if="item.subNavItems"
              class="collapsible pl-4 py-2"
          >
            <SidebarMenuSub>
              <div
                  v-for="subItem in item.subNavItems"
                  :key="subItem.title"
                  class="pl-2"
              >
                <SidebarMenuSubItem>
                  <SidebarMenuSubButton
                      as-child
                      class="h-9"
                      :is-active="urlIsActive(subItem.href)"
                  >
                    <NavLink :to="subItem.href">
                      {{ subItem.title }}
                    </NavLink>
                  </SidebarMenuSubButton>
                </SidebarMenuSubItem>
              </div>
            </SidebarMenuSub>
          </CollapsibleContent>
        </SidebarMenuItem>
      </Collapsible>
    </SidebarMenu>
  </SidebarGroup>
</template>

<style scoped>
.arrow {
  transform: rotate(270deg);
}

:deep([data-state="open"]) .arrow {
  transform: rotate(360deg);
}
</style>
