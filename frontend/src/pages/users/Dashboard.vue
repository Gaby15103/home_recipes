<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useAuthStore } from '@/stores/auth';
import { ROUTES } from '@/router/routes';
import { Card, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { User, Lock, Utensils } from 'lucide-vue-next';

const { t } = useI18n();
const authStore = useAuthStore();
const user = authStore.user;
</script>

<template>
  <div class="container py-10 space-y-8">
    <section class="flex flex-col gap-2">
      <h1 class="text-3xl font-bold tracking-tight">{{ t('Welcome back') }}, {{ user?.first_name }}!</h1>
      <p class="text-muted-foreground">{{ t('Manage your recipes and account settings.') }}</p>
    </section>

    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <RouterLink :to="ROUTES.USER.MY_RECIPES">
        <Card class="hover:bg-accent transition-colors cursor-pointer h-full">
          <CardHeader>
            <Utensils class="h-8 w-8 mb-2 text-primary" />
            <CardTitle>{{ t('My Recipes') }}</CardTitle>
            <CardDescription>{{ t('Manage the recipes you have shared.') }}</CardDescription>
          </CardHeader>
        </Card>
      </RouterLink>

      <RouterLink :to="ROUTES.USER.PROFILE">
        <Card class="hover:bg-accent transition-colors cursor-pointer h-full">
          <CardHeader>
            <User class="h-8 w-8 mb-2 text-primary" />
            <CardTitle>{{ t('Profile') }}</CardTitle>
            <CardDescription>{{ t('Update your name and public info.') }}</CardDescription>
          </CardHeader>
        </Card>
      </RouterLink>

      <RouterLink :to="ROUTES.USER.SECURITY">
        <Card class="hover:bg-accent transition-colors cursor-pointer h-full">
          <CardHeader>
            <Lock class="h-8 w-8 mb-2 text-primary" />
            <CardTitle>{{ t('Security') }}</CardTitle>
            <CardDescription>{{ t('2FA and Password management.') }}</CardDescription>
          </CardHeader>
        </Card>
      </RouterLink>
    </div>
  </div>
</template>