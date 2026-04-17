<script setup lang="ts">
import { TrendingUp } from 'lucide-vue-next'
import { ref, onMounted } from 'vue'
import { getStudioStats, type DashboardStats } from '@/api/studio'

const stats = ref<DashboardStats | null>(null)

onMounted(async () => {
  stats.value = await getStudioStats()
})
</script>

<template>
  <div class="space-y-8 animate-in fade-in duration-500">
    <section class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="lg:col-span-2 p-8 rounded-3xl bg-[#0a0a0a] border border-neutral-800 flex flex-col justify-center min-h-[300px]">
        <h2 class="text-[10px] font-black uppercase tracking-widest text-neutral-500 mb-2">Total Impressions</h2>
        <div class="flex items-baseline gap-4">
          <h1 class="text-6xl font-black text-white tracking-tighter">{{ stats?.total_views || 0 }}</h1>
          <div class="flex items-center gap-1 text-green-500 text-[10px] font-black bg-green-500/10 px-2 py-0.5 rounded-full">
            <TrendingUp class="h-3 w-3" />
            LIVE
          </div>
        </div>
        <p class="text-xs text-neutral-600 mt-4 max-w-xs">
          Your recipes are being discovered across the platform. This represents total unique views.
        </p>
      </div>

      <div class="p-6 rounded-3xl bg-neutral-900/30 border border-neutral-800">
        <h3 class="text-[10px] font-black uppercase tracking-widest text-neutral-500 mb-6">Distribution</h3>
        <div class="space-y-4">
          <div v-for="source in ['Direct Search', 'Feed', 'Favorites']" :key="source" class="group">
            <div class="flex justify-between text-[10px] font-bold uppercase mb-1.5">
              <span class="text-neutral-400">{{ source }}</span>
              <span class="text-neutral-500 group-hover:text-primary transition-colors">--%</span>
            </div>
            <div class="h-1 w-full bg-neutral-800 rounded-full overflow-hidden">
              <div class="h-full bg-neutral-700 group-hover:bg-primary transition-all duration-500" :style="{width: '33%'}"></div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>