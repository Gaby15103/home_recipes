<script setup lang="ts">
import {ref, watch} from 'vue'
import {useI18n} from 'vue-i18n'
import {useAuthStore} from '@/stores/auth'
import {Card, CardContent, CardDescription, CardHeader, CardTitle} from '@/components/ui/card'
import {Bell, ChevronRight, Palette, ShieldCheck, User} from 'lucide-vue-next'

import LanguageChanger from "@/components/navigation/Navbar/LanguageChanger.vue"
import ModeToggle from "@/components/navigation/Navbar/ModeToggle.vue"
import ProfileEdit from "@/pages/users/ProfileEdit.vue";
import {useRoute} from "vue-router";
import Security from "@/pages/users/Security.vue";
import NotificationSettings from "@/pages/users/NotificationSettings.vue";

const { t } = useI18n()
const authStore = useAuthStore()

const route = useRoute()
const activeSection = ref(route.query.section?.toString() || 'appearance')

watch(() => route.query.section, (newSection) => {
  if (newSection) activeSection.value = newSection.toString()
})


const sections = [
  { id: 'appearance', label: t('Home.Appearance'), icon: Palette },
  { id: 'account', label: t('Profile'), icon: User },
  { id: 'security', label: 'Security', icon: ShieldCheck },
  { id: 'notifications', label: 'Notifications', icon: Bell },
]
</script>

<template>
  <div class="min-h-screen pb-20 animate-in fade-in duration-700">
    <div class="w-full h-32 bg-linear-to-b from-primary/5 to-transparent border-b/50"></div>

    <div class="container max-w-7xl -mt-16 space-y-6 relative z-10">
      <div class="bg-background/60 backdrop-blur-md p-6 rounded-3xl border shadow-xs">
        <h1 class="text-3xl font-black tracking-tight uppercase">{{ t('Home.Settings') }}</h1>
        <p class="text-muted-foreground text-sm font-medium">{{ t('settings.hub.desc') }}</p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
        <aside class="space-y-2">
          <nav class="flex flex-col gap-1">
            <button
                v-for="section in sections"
                :key="section.id"
                @click="activeSection = section.id"
                :class="[
                'w-full p-3 rounded-2xl flex items-center justify-between transition-all duration-200 group',
                activeSection === section.id
                  ? 'bg-primary text-primary-foreground shadow-lg shadow-primary/20'
                  : 'hover:bg-secondary/50 text-muted-foreground hover:text-foreground'
              ]"
            >
              <div class="flex items-center gap-3">
                <component :is="section.icon" class="h-4 w-4" />
                <span class="font-bold uppercase tracking-wider text-xs">{{ section.label }}</span>
              </div>
              <ChevronRight v-if="activeSection === section.id" class="h-4 w-4" />
            </button>
          </nav>
        </aside>

        <div class="lg:col-span-3 space-y-6">

          <Transition name="fade" mode="out-in">
            <div v-if="activeSection === 'appearance'" class="space-y-6">
              <Card class="rounded-3xl border-2 shadow-none overflow-hidden">
                <CardHeader class="pb-4">
                  <div class="flex items-center gap-2 text-primary">
                    <Palette class="h-5 w-5" />
                    <CardTitle class="text-lg font-bold uppercase tracking-tight">{{ t('Home.Appearance') }}</CardTitle>
                  </div>
                  <CardDescription class="text-xs">
                    Configure how the application looks and feels on your device.
                  </CardDescription>
                </CardHeader>
                <CardContent class="p-2 sm:p-6 pt-0 space-y-2">
                  <div class="flex items-center justify-between p-3 px-4 rounded-xl bg-secondary/10 border border-transparent transition-all">
                    <div class="space-y-0.5">
                      <p class="text-sm font-bold">{{ t('Home.Language') }}</p>
                      <p class="text-[10px] text-muted-foreground leading-none">Global display language</p>
                    </div>
                    <div class="scale-90 origin-right">
                      <LanguageChanger />
                    </div>
                  </div>

                  <div class="flex items-center justify-between p-3 px-4 rounded-xl bg-secondary/10 border border-transparent transition-all">
                    <div class="space-y-0.5">
                      <p class="text-sm font-bold">{{ t('theme.toggle') }}</p>
                      <p class="text-[10px] text-muted-foreground leading-none">Light, dark, or system</p>
                    </div>
                    <div class="scale-90 origin-right">
                      <ModeToggle />
                    </div>
                  </div>
                </CardContent>
              </Card>
            </div>

            <div v-else-if="activeSection === 'account'" class="space-y-6">
              <ProfileEdit />
            </div>
            <div v-else-if="activeSection === 'security'" class="space-y-6">
              <Security/>
            </div>
            <div v-else-if="activeSection === 'notifications'" class="space-y-6">
              <NotificationSettings/>
            </div>

            <div v-else class="flex flex-col items-center justify-center py-20 text-muted-foreground">
              <ShieldCheck class="h-12 w-12 mb-4 opacity-20" />
              <p class="font-bold uppercase tracking-widest text-xs">Section under construction</p>
            </div>
          </Transition>

        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>