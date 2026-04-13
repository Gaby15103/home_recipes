<script setup lang="ts">
import { Icon } from '@iconify/vue'
import {type BasicColorMode, useColorMode} from '@vueuse/core'
import { Button } from '@/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import {useI18n} from "vue-i18n";
import {updateProfile} from "@/api/user.ts";
import {useAuthStore} from "@/stores/auth.ts";
const { t, locale, availableLocales } = useI18n()
const authStore = useAuthStore()

const mode = useColorMode()
async function setTheme(newTheme: BasicColorMode | 'auto') {
  mode.value = newTheme

  // 2. If user is logged in, persist to backend
  if (authStore.user) {
    try {
      const updatedUser = await updateProfile({
        ...authStore.user,
        preferences: {
          ...authStore.user.preferences,
          theme: newTheme
        }
      })

      // 3. Sync the store with the response from Rust
      authStore.setUser(updatedUser)
    } catch (err) {
      console.error('Failed to save theme preference:', err)
      // Optional: notify user, but usually overkill for a theme toggle
    }
  }
}
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="outline" class="rounded-xl border-2">
        <Icon icon="radix-icons:moon" class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
        <Icon icon="radix-icons:sun" class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
        <span class="sr-only">{{ t('theme.toggle') }}</span>
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent align="end" class="rounded-xl border-2">
      <DropdownMenuItem @click="setTheme('light')">
        <Icon icon="radix-icons:sun" class="mr-2 h-4 w-4" />
        {{ t('theme.light') }}
      </DropdownMenuItem>
      <DropdownMenuItem @click="setTheme('dark')">
        <Icon icon="radix-icons:moon" class="mr-2 h-4 w-4" />
        {{ t('theme.dark') }}
      </DropdownMenuItem>
      <DropdownMenuItem @click="setTheme('auto')">
        <Icon icon="radix-icons:laptop" class="mr-2 h-4 w-4" />
        {{ t('theme.system') }}
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>