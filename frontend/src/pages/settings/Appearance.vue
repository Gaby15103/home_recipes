<script setup lang="ts">

import AppearanceTabs from '@/layouts/AppearanceTabs.vue';
import HeadingSmall from '@/components/common/HeadingSmall.vue';
import {type BreadcrumbItem} from '@/types';

import MainLayout from '@/layouts/MainLayout.vue';
import SettingsSettingsLayout from '@/layouts/SettingsLayout.vue';

const breadcrumbItems: BreadcrumbItem[] = [
  {
    title: 'Appearance settings',
    href: '/edit/appearance',
  },
];
</script>

<template>

  <h1 class="sr-only">Appearance Settings</h1>

  <SettingsSettingsLayout>
    <div class="space-y-6">
      <HeadingSmall
          title="Appearance settings"
          description="Update your account's appearance settings"
      />
      <AppearanceTabs/>
    </div>
  </SettingsSettingsLayout>
</template>
