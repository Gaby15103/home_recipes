<script setup lang="ts">
import { useI18n } from "vue-i18n"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu"
import { setLanguage } from "@/utils/setLanguage.ts"
import { useAuthStore } from '@/stores/auth'
import { updateProfile } from '@/api/user'
import { Languages } from 'lucide-vue-next'

const { locale, availableLocales } = useI18n()
const authStore = useAuthStore()

async function handleLanguageChange(newLocale: string) {
  // 1. Update the UI locally (via your existing utility)
  setLanguage(newLocale)

  // 2. If user is logged in, persist the choice to the database
  if (authStore.user) {
    try {
      const updatedUser = await updateProfile({
        ...authStore.user,
        preferences: {
          ...authStore.user.preferences,
          language: newLocale
        }
      })

      // 3. Sync the store with the updated preferences from the backend
      authStore.setUser(updatedUser)
    } catch (err) {
      console.error('Failed to save language preference:', err)
    }
  }
}
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="outline" class="gap-2 rounded-xl border-2 uppercase font-bold">
        {{ locale }}
      </Button>
    </DropdownMenuTrigger>

    <DropdownMenuContent align="end" class="rounded-xl border-2">
      <DropdownMenuItem
          v-for="loc in availableLocales"
          :key="loc"
          class="uppercase font-medium cursor-pointer"
          @click="handleLanguageChange(loc)"
      >
        <span :class="{ 'text-primary font-bold': locale === loc }">
          {{ loc }}
        </span>
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>