<script setup lang="ts">
import { computed } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { Bell, MessageSquare, Heart, UserPlus, ShieldCheck, Mail } from 'lucide-vue-next'

const authStore = useAuthStore()

const prefs = computed(() => authStore.user?.preferences)

const handleToggle = (key: string, value: boolean) => {
  if (!prefs.value) return

  const newPreferences = {
    ...prefs.value,
    [key]: value
  }
  authStore.updatePreference(newPreferences)
}
</script>

<template>
  <div class="space-y-6 animate-in fade-in slide-in-from-bottom-2 duration-500">

    <Card class="rounded-3xl border-2 shadow-none overflow-hidden">
      <CardHeader class="pb-4">
        <div class="flex items-center gap-2 text-primary">
          <Bell class="h-5 w-5" />
          <CardTitle class="text-lg font-bold uppercase tracking-tight">Activity Preferences</CardTitle>
        </div>
        <CardDescription class="text-xs">
          Choose which social interactions trigger a notification.
        </CardDescription>
      </CardHeader>

      <CardContent class="space-y-2">
        <div class="flex items-center justify-between p-3 px-4 rounded-xl bg-secondary/10 border border-transparent transition-all">
          <div class="flex items-center gap-4">
            <div class="p-2 bg-background rounded-lg shadow-xs">
              <MessageSquare class="h-4 w-4 text-muted-foreground" />
            </div>
            <div class="space-y-0.5">
              <p class="text-sm font-bold">Comments</p>
              <p class="text-[10px] text-muted-foreground leading-none">When someone replies to your recipes</p>
            </div>
          </div>
          <Switch
              :defaultValue="prefs?.recipe_comment_enabled"
              @update:model-value="(v: boolean) => handleToggle('recipe_comment_enabled', v)"
          />
        </div>

        <div class="flex items-center justify-between p-3 px-4 rounded-xl bg-secondary/10 border border-transparent transition-all">
          <div class="flex items-center gap-4">
            <div class="p-2 bg-background rounded-lg shadow-xs">
              <MessageSquare class="h-4 w-4 text-muted-foreground" />
            </div>
            <div class="space-y-0.5">
              <p class="text-sm font-bold">Comments reply</p>
              <p class="text-[10px] text-muted-foreground leading-none">When someone replies to your recipes</p>
            </div>
          </div>
          <Switch
              :defaultValue="prefs?.comment_reply_enabled"
              @update:model-value="(v: boolean) => handleToggle('recipe_comment_enabled', v)"
          />
        </div>

        <div class="flex items-center justify-between p-3 px-4 rounded-xl bg-secondary/10 border border-transparent transition-all">
          <div class="flex items-center gap-4">
            <div class="p-2 bg-background rounded-lg shadow-xs">
              <Heart class="h-4 w-4 text-muted-foreground" />
            </div>
            <div class="space-y-0.5">
              <p class="text-sm font-bold">Recipe Likes</p>
              <p class="text-[10px] text-muted-foreground leading-none">When someone saves your recipe to favorites</p>
            </div>
          </div>
          <Switch
              :defaultValue="prefs?.recipe_favorite_enabled"
              @update:model-value="(v: boolean) => handleToggle('recipe_favorite_enabled', v)"
          />
        </div>

        <div class="flex items-center justify-between p-3 px-4 rounded-xl bg-secondary/10 border border-transparent transition-all">
          <div class="flex items-center gap-4">
            <div class="p-2 bg-background rounded-lg shadow-xs">
              <UserPlus class="h-4 w-4 text-muted-foreground" />
            </div>
            <div class="space-y-0.5">
              <p class="text-sm font-bold">New Followers</p>
              <p class="text-[10px] text-muted-foreground leading-none">When someone follows your profile</p>
            </div>
          </div>
          <Switch
              :checked="false"
              disabled
          />
        </div>
      </CardContent>
    </Card>

    <Card class="rounded-3xl border-2 shadow-none overflow-hidden">
      <CardHeader class="pb-4">
        <div class="flex items-center gap-2 text-primary">
          <Mail class="h-5 w-5" />
          <CardTitle class="text-lg font-bold uppercase tracking-tight">Email & Security</CardTitle>
        </div>
      </CardHeader>
      <CardContent class="space-y-2">
        <div class="flex items-center justify-between p-3 px-4 rounded-xl bg-secondary/10 border border-transparent transition-all">
          <div class="flex items-center gap-4">
            <div class="p-2 bg-background rounded-lg shadow-xs">
              <ShieldCheck class="h-4 w-4 text-primary" />
            </div>
            <div class="space-y-0.5">
              <p class="text-sm font-bold">Security Alerts</p>
              <p class="text-[10px] text-muted-foreground leading-none">Logins from new devices or password changes</p>
            </div>
          </div>
          <Switch :checked="true" disabled />
        </div>
      </CardContent>
    </Card>

  </div>
</template>