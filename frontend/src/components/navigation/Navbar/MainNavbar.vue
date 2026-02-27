<script setup lang="ts">
import {ref} from "vue"
import {RouterLink} from "vue-router"
import {storeToRefs} from "pinia"
import {useI18n} from "vue-i18n"
import {LayoutDashboard, LogOut, Menu, User} from "lucide-vue-next"

import {useAuthStore} from "@/stores/auth.ts"
import router from "@/router"
import {ROUTES} from "@/router/routes.ts"

import ModeToggle from "@/components/navigation/Navbar/ModeToggle.vue"
import NavLink from "@/components/navigation/NavLink.vue"
import LanguageChanger from "@/components/navigation/Navbar/LanguageChanger.vue"
import {Button} from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import {Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger,} from "@/components/ui/sheet"

// State
const authStore = useAuthStore()
const {user} = storeToRefs(authStore)
const {t} = useI18n()
const isMobileMenuOpen = ref(false)

async function logout() {
  try {
    await authStore.logout()
    await router.push(ROUTES.HOME)
    isMobileMenuOpen.value = false
  } catch (e: any) {
    console.error(e.message)
  }
}
</script>

<template>
  <header
      class="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
    <div class="flex h-16 items-center justify-between px-4 md:px-8">

      <div class="flex items-center gap-6">
        <Sheet v-model:open="isMobileMenuOpen">
          <SheetTrigger as-child>
            <Button variant="ghost" size="icon" class="md:hidden">
              <Menu class="h-5 w-5"/>
              <span class="sr-only">Toggle menu</span>
            </Button>
          </SheetTrigger>
          <SheetContent side="left" class="flex flex-col w-[300px] justify-between">
            <div class="flex flex-col h-full">
              <SheetHeader class="border-b pb-4">
                <SheetTitle class="text-left">{{ t('Title') }}</SheetTitle>
              </SheetHeader>

              <nav class="flex flex-col gap-2 mt-6">
                <RouterLink :to="ROUTES.HOME" @click="isMobileMenuOpen = false"
                            class="flex items-center p-3 text-sm font-medium hover:bg-accent rounded-md">
                  {{ t('Home') }}
                </RouterLink>
                <RouterLink :to="ROUTES.RECIPES" @click="isMobileMenuOpen = false"
                            class="flex items-center p-3 text-sm font-medium hover:bg-accent rounded-md">
                  {{ t('Recipes') }}
                </RouterLink>
              </nav>

              <div class="mt-auto border-t pt-4">
                <div class="flex items-center justify-between px-2 py-2">
                  <span class="text-sm font-medium">{{ t('Appearance') }}</span>
                  <ModeToggle/>
                </div>
                <div class="flex items-center justify-between px-2 py-2">
                  <span class="text-sm font-medium">{{ t('Language') }}</span>
                  <LanguageChanger/>
                </div>
              </div>

              <div v-if="user" class="border-t mt-4 pt-4 pb-6 flex flex-col gap-2">
                <div class="flex items-center gap-3 px-2 mb-2">
                  <img :src="$apiUrl + user.avatar_url" alt="avatar"
                       class="h-10 w-10 rounded-full border object-cover"/>
                  <div class="flex flex-col overflow-hidden">
                    <span class="text-sm font-semibold truncate">{{ user.username }}</span>
                    <span class="text-xs text-muted-foreground truncate">{{ user.email }}</span>
                  </div>
                </div>
                <RouterLink :to="ROUTES.HOME" @click="isMobileMenuOpen = false"
                            class="flex items-center gap-2 p-2 text-sm hover:bg-accent rounded-md">
                  <User class="h-4 w-4"/>
                  {{ t('Profile') }}
                </RouterLink>
                <button @click="logout"
                        class="flex items-center gap-2 p-2 text-sm text-destructive hover:bg-destructive/10 rounded-md w-full text-left">
                  <LogOut class="h-4 w-4"/>
                  {{ t('Logout') }}
                </button>
              </div>
              <div v-else class="border-t mt-4 pt-4 pb-6 flex flex-col gap-2">
                <Button variant="outline" as-child @click="isMobileMenuOpen = false">
                  <RouterLink to="/login">{{ t('Login') }}</RouterLink>
                </Button>
                <Button as-child @click="isMobileMenuOpen = false">
                  <RouterLink to="/register">{{ t('Register') }}</RouterLink>
                </Button>
              </div>
            </div>
          </SheetContent>
        </Sheet>

        <RouterLink :to="ROUTES.HOME" class="font-bold text-xl tracking-tight flex items-center gap-2">
          <span>🍳</span>
          <span class="hidden lg:inline-block">{{ t('Title') }}</span>
        </RouterLink>

        <nav class="hidden md:flex items-center gap-6">
          <NavLink :to="ROUTES.RECIPES"
                   class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors">
            {{ t('Recipes') }}
          </NavLink>
        </nav>
      </div>

      <div class="flex items-center gap-4">
        <div class="hidden md:flex items-center gap-2">
          <LanguageChanger/>
          <ModeToggle/>
        </div>

        <div class="hidden md:block">
          <DropdownMenu v-if="user">
            <DropdownMenuTrigger as-child>
              <Button variant="ghost" class="flex items-center gap-2 px-2">
                <img :src="$apiUrl + user.avatar_url" alt="avatar" class="h-8 w-8 rounded-full object-cover"/>
                <span class="hidden sm:inline text-sm font-medium">{{ user.username }}</span>
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-56" align="end">
              <DropdownMenuLabel class="font-normal">
                <div class="flex flex-col space-y-1">
                  <p class="text-sm font-medium leading-none">{{ user.username }}</p>
                  <p class="text-xs leading-none text-muted-foreground">{{ user.email }}</p>
                </div>
              </DropdownMenuLabel>
              <DropdownMenuSeparator/>
              <DropdownMenuItem as-child>
                <RouterLink :to="ROUTES.HOME" class="cursor-pointer w-full flex items-center">
                  <User class="mr-2 h-4 w-4"/>
                  {{ t('Profile') }}
                </RouterLink>
              </DropdownMenuItem>
              <DropdownMenuItem v-if="authStore.hasRole('ADMIN') || authStore.hasRole('MODERATOR')" as-child>
                <RouterLink :to="ROUTES.ADMIN.DASHBOARD" class="cursor-pointer w-full flex items-center">
                  <LayoutDashboard class="mr-2 h-4 w-4"/>
                  {{ t('Admin Dashboard') }}
                </RouterLink>
              </DropdownMenuItem>
              <DropdownMenuSeparator/>
              <DropdownMenuItem @click="logout"
                                class="text-destructive focus:bg-destructive focus:text-destructive-foreground cursor-pointer">
                <LogOut class="mr-2 h-4 w-4"/>
                {{ t('Logout') }}
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>

          <div v-else class="flex items-center gap-2">
            <Button variant="ghost" as-child>
              <RouterLink to="/login">{{ t('Login') }}</RouterLink>
            </Button>
            <Button as-child>
              <RouterLink to="/register">{{ t('Register') }}</RouterLink>
            </Button>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>