<script setup lang="ts">
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"
defineProps<{ text: string; maxWidth?: string }>()
</script>

<template>
  <TooltipProvider :delay-duration="200">
    <Tooltip>
      <TooltipTrigger as-child>
        <span :class="['block truncate cursor-help', maxWidth || 'max-w-[200px]']">
          {{ text }}
        </span>
      </TooltipTrigger>
      <TooltipContent class="max-w-sm bg-popover text-popover-foreground shadow-xl border p-3 rounded-xl">
        <p class="text-sm leading-relaxed">{{ text }}</p>
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
</template>