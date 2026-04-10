<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getHealth } from '@/api/system' // Using the health check function we discussed
import { useI18n } from "vue-i18n"

const { t } = useI18n()
const isOnline = ref(true)
const version = ref('1.0.0')

onMounted(async () => {
  try {
    const health = await getHealth()
    isOnline.value = health.status === 'healthy'
    version.value = health.version
  } catch {
    isOnline.value = false
  }
})
</script>

<template>
  <div>
    <h3 class="text-xs font-semibold uppercase tracking-wider text-foreground mb-4">
      {{ t('footer.system') }}
    </h3>
    <ul class="space-y-2">
      <li class="flex items-center gap-2 text-sm text-muted-foreground">
        <span
            class="h-2 w-2 rounded-full transition-colors duration-500"
            :class="isOnline ? 'bg-green-500 animate-pulse' : 'bg-destructive'"
        ></span>
        {{ isOnline ? t('footer.statusOk') : t('footer.statusError') }}
      </li>
      <li class="text-xs text-muted-foreground/60 font-mono italic">
        v{{ version }}
      </li>
    </ul>
  </div>
</template>