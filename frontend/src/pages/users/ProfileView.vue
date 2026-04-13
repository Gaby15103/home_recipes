<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ROUTES } from '@/router/routes'
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Pencil, Utensils, Calendar } from 'lucide-vue-next'
import type { User } from '@/models/User'

const { t } = useI18n()

defineProps<{
  user: User | null
  isOwnProfile: boolean
  recipesCount?: number
}>()
</script>

<template>
  <div class="animate-in fade-in duration-500">
    <div class="w-full h-48 bg-linear-to-r from-primary/10 via-secondary/20 to-primary/5 border-b"></div>

    <div class="container max-w-6xl -mt-20 pb-20 relative z-10">
      <div class="flex flex-col md:flex-row gap-8 items-end justify-between">

        <div class="flex flex-col md:flex-row items-center md:items-end gap-6 text-center md:text-left">
          <Avatar class="h-40 w-40 border-[6px] border-background shadow-2xl">
            <AvatarImage :src="user?.avatar_url ? $apiUrl + user.avatar_url : undefined" />
            <AvatarFallback class="text-5xl font-bold bg-primary text-white">
              {{ user?.username?.charAt(0).toUpperCase() }}
            </AvatarFallback>
          </Avatar>

          <div class="pb-2 space-y-1">
            <h1 class="text-4xl font-black tracking-tighter">
              {{ user?.first_name }} {{ user?.last_name }}
            </h1>
            <div class="flex items-center justify-center md:justify-start gap-3 text-muted-foreground font-medium">
              <span class="text-primary font-semibold">@{{ user?.username }}</span>
              <span class="opacity-30">•</span>
              <div class="flex items-center gap-1.5 text-sm">
                <Calendar class="h-3.5 w-3.5" />
                {{ t('profile.public.member_since') }} 2026
              </div>
            </div>
          </div>
        </div>

        <div v-if="isOwnProfile" class="pb-2">
          <Button as-child variant="outline" class="rounded-xl border-2 hover:bg-primary hover:text-white transition-all">
            <RouterLink :to="{ path: ROUTES.USER.SETTINGS, query: { section: 'account', from: 'profile' } }">
              <Pencil class="mr-2 h-4 w-4" /> {{ t('profile.public.edit_profile') }}
            </RouterLink>
          </Button>
        </div>
      </div>

      <div class="mt-12 mb-8 flex items-center gap-4">
        <h2 class="text-2xl font-bold tracking-tight">{{ t('profile.public.recipes_title') }}</h2>
        <Badge variant="secondary" class="rounded-full px-3 font-mono">
          {{ recipesCount || 0 }}
        </Badge>
        <div class="h-px flex-1 bg-muted"></div>
      </div>

      <slot name="recipes">
        <div class="py-20 text-center border-2 border-dashed rounded-3xl bg-secondary/10">
          <Utensils class="h-12 w-12 mx-auto text-muted-foreground/20 mb-4" />
          <p class="text-muted-foreground italic">
            {{ t('profile.public.no_recipes') }}
          </p>
        </div>
      </slot>
    </div>
  </div>
</template>