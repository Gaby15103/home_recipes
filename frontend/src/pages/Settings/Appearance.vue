<script setup lang="ts">

import AppearanceTabs from '@/components/AppearanceTabs.vue';
import HeadingSmall from '@/components/HeadingSmall.vue';
import { type BreadcrumbItem } from '@/types';

import AppLayout from '@/components/layout/AppLayout.vue';
import SettingsLayout from '@/components/layout/settings/Layout.vue';

const breadcrumbItems: BreadcrumbItem[] = [
  {
    title: 'Appearance settings',
    href:'/edit/appearance',
  },
];
</script>

<template>
  <AppLayout :breadcrumbs="breadcrumbItems">

    <h1 class="sr-only">Appearance Settings</h1>

    <SettingsLayout>
      <div class="space-y-6">
        <HeadingSmall
            title="Appearance settings"
            description="Update your account's appearance settings"
        />
        <AppearanceTabs />
      </div>
    </SettingsLayout>
  </AppLayout>
</template>
