<script setup lang="ts">
import Heading from '@/components/common/Heading.vue';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { useActiveUrl } from '@/composables/useActiveUrl.ts';
import { toUrl } from '@/lib/utils.ts';
import { type NavItem } from '@/types';
import NavLink from "@/components/navigation/NavLink.vue";

import { useRoute } from "vue-router";
import {ROUTES} from "@/router/routes.ts";

const route = useRoute();

const sidebarNavItems: NavItem[] = [
  {
    title: 'Profile',
    href: ROUTES.ADMIN.USER.BASE,
  },
  {
    title: 'Password',
    href: ROUTES.ADMIN.USER.PASSWORD,
  },
  {
    title: 'Two-Factor Auth',
    href: ROUTES.ADMIN.USER.TWO_FACTOR,
  },
  {
    title: 'Appearance',
    href: ROUTES.ADMIN.USER.APPEARANCE,
  },
];

const { urlIsActive } = useActiveUrl();
</script>

<template>
  <div class="px-4 py-6">
    <Heading
        title="Settings"
        description="Manage your profile and account settings"
    />

    <div class="flex flex-col lg:flex-row lg:space-x-12">
      <aside class="w-full max-w-xl lg:w-48">
        <nav class="flex flex-col space-y-1 space-x-0" aria-label="Settings">
          <Button
              v-for="item in sidebarNavItems"
              :key="toUrl(item.href)"
              variant="ghost"
              :class="[
                            'w-full justify-start',
                            { 'bg-muted': urlIsActive(item.href) },
                        ]"
              as-child
          >
            <NavLink :to="item.href">
              <component :is="item.icon" class="h-4 w-4" />
              {{ item.title }}
            </NavLink>
          </Button>
        </nav>
      </aside>

      <Separator class="my-6 lg:hidden" />

      <div class="flex-1 md:max-w-2xl">
        <section class="max-w-xl space-y-12">
          <slot />
        </section>
      </div>
    </div>
  </div>
</template>
