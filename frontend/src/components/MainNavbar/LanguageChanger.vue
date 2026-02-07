<script setup lang="ts">
import { useI18n } from "vue-i18n"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu"
import { setLanguage } from "@/utils/setLanguage";
const { t, locale, availableLocales } = useI18n()

function changeLocale(newLocale: string) {
  locale.value = newLocale
}
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="outline">
        {{ locale.toUpperCase() }}
      </Button>
    </DropdownMenuTrigger>

    <DropdownMenuContent align="end">
      <DropdownMenuItem
          v-for="loc in availableLocales"
          :key="loc"
          @click="setLanguage(loc)"
      >
        {{ loc.toUpperCase() }}
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
