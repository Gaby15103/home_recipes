<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import { ROUTES } from '@/router/routes'
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { User, ShieldCheck, Utensils, Palette, ChevronRight, Activity } from 'lucide-vue-next'
import {Avatar} from "@/components/ui/avatar";

const { t } = useI18n()
const authStore = useAuthStore()
const user = authStore.user

const menuItems = computed(() => [
  {
    title: t('settings.hub.profile.title'),
    description: t('settings.hub.profile.desc'),
    status: user?.username, // Info dynamique
    icon: User,
    to: ROUTES.USER.PROFILE,
    color: 'text-blue-500'
  },
  {
    title: t('settings.hub.recipes.title'),
    description: t('settings.hub.recipes.desc'),
    status: `12 ${t('recipes.unit')}`, // Exemple de data
    icon: Utensils,
    to: ROUTES.USER.RECIPES,
    color: 'text-orange-500'
  },
  {
    title: t('settings.hub.security.title'),
    description: t('settings.hub.security.desc'),
    status: t('settings.status.protected'),
    icon: ShieldCheck,
    to: ROUTES.USER.SECURITY,
    color: 'text-green-500'
  },
  {
    title: t('settings.hub.appearance.title'),
    description: t('settings.hub.appearance.desc'),
    status: t('settings.status.theme_auto'),
    icon: Palette,
    to: ROUTES.USER.APPEARANCE,
    color: 'text-purple-500'
  }
])
</script>

<template>
  <div class="container max-w-6xl py-10">
    <div class="flex flex-col lg:flex-row gap-10">

      <aside class="lg:w-1/3 space-y-6">
        <div class="space-y-4">
          <h1 class="text-3xl font-bold tracking-tight">{{ t('settings.hub.title') }}</h1>
          <p class="text-muted-foreground">{{ t('settings.hub.subtitle') }}</p>
        </div>

        <Card class="bg-primary/5 border-none shadow-none">
          <CardHeader class="flex flex-row items-center gap-4">
            <Avatar class="h-12 w-12 border-2 border-background">
              <AvatarImage :src="user?.avatar_url" />
              <AvatarFallback>{{ user?.username[0] }}</AvatarFallback>
            </Avatar>
            <div class="flex flex-col">
              <span class="font-bold">{{ user?.first_name }} {{ user?.last_name }}</span>
              <span class="text-xs text-muted-foreground">{{ user?.email }}</span>
            </div>
          </CardHeader>
          <CardContent>
            <div class="text-xs space-y-2">
              <div class="flex justify-between">
                <span class="text-muted-foreground">{{ t('settings.hub.role') }}</span>
                <Badge variant="outline" class="capitalize">{{ user?.role || 'User' }}</Badge>
              </div>
            </div>
          </CardContent>
        </Card>
      </aside>

      <div class="lg:flex-1 grid gap-4 sm:grid-cols-2">
        <RouterLink v-for="item in menuItems" :key="item.to" :to="item.to" class="group">
          <Card class="h-full border-2 hover:border-primary/50 transition-all duration-300 shadow-sm hover:shadow-md">
            <CardHeader>
              <div class="flex justify-between items-start mb-2">
                <div :class="['p-2 rounded-lg bg-secondary', item.color]">
                  <component :is="item.icon" class="h-5 w-5" />
                </div>
                <Badge variant="secondary" class="text-[10px] font-mono">
                  {{ item.status }}
                </Badge>
              </div>
              <CardTitle class="text-lg group-hover:text-primary transition-colors">
                {{ item.title }}
              </CardTitle>
              <CardDescription class="text-xs leading-relaxed">
                {{ item.description }}
              </CardDescription>
            </CardHeader>
          </Card>
        </RouterLink>
      </div>

    </div>
  </div>
</template>