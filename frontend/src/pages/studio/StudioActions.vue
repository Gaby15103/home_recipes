<template>
  <div class="flex items-center gap-3">
    <button class="bg-neutral-900 border border-neutral-800 p-2 rounded-xl text-neutral-400 hover:text-white transition-all">
      <slot name="secondary" />
    </button>
    <button class="bg-primary hover:bg-primary/90 text-black px-5 py-2.5 rounded-xl text-xs font-black uppercase tracking-tight flex items-center gap-2 transition-transform active:scale-95">
      <PlusCircle class="h-4 w-4" />
      Create New
    </button>
  </div>
</template>

<script setup>
import { PlusCircle } from 'lucide-vue-next'
</script>