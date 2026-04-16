<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Book, Eye, Lock, Globe, Loader2 } from 'lucide-vue-next'
import {getStudioStats, type DashboardStats, getRecentRecipes} from '@/api/studio'
import type { RecipeView } from "@/models/Recipe"

const stats = ref<DashboardStats | null>(null)
const recentRecipes = ref<RecipeView[]>([])
const isLoading = ref(true)

async function loadDashboard() {
  try {
    isLoading.value = true
    const [statsRes, recipesRes] = await Promise.all([
      getStudioStats(),
      getRecentRecipes(4)
    ])
    stats.value = statsRes
    recentRecipes.value = recipesRes
  } catch (e) {
    console.error("Dashboard sync failed:", e)
  } finally {
    isLoading.value = false
  }
}

onMounted(loadDashboard)
</script>

<template>
  <div class="space-y-8 animate-in fade-in duration-500">
    <div v-if="isLoading" class="flex items-center gap-3 text-neutral-500 py-10">
      <Loader2 class="h-4 w-4 animate-spin text-primary" />
      <span class="text-[10px] font-black uppercase tracking-widest">Fetching Station Data...</span>
    </div>

    <template v-else>
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">

        <div class="p-5 rounded-2xl bg-[#0a0a0a] border border-neutral-800 transition-colors hover:border-neutral-700">
          <p class="text-[9px] font-black text-neutral-500 uppercase tracking-widest mb-1">Total Recipes</p>
          <div class="flex items-end justify-between">
            <h3 class="text-2xl font-black text-neutral-200 tracking-tighter">{{ stats?.total_recipes || 0 }}</h3>
            <Book class="h-4 w-4 text-neutral-700 mb-1" />
          </div>
        </div>

        <div class="p-5 rounded-2xl bg-[#0a0a0a] border border-neutral-800 transition-colors hover:border-neutral-700">
          <p class="text-[9px] font-black text-neutral-500 uppercase tracking-widest mb-1">Public Content</p>
          <div class="flex items-end justify-between">
            <h3 class="text-2xl font-black text-neutral-200 tracking-tighter">{{ stats?.public_recipes || 0 }}</h3>
            <Globe class="h-4 w-4 text-green-900/50 mb-1" />
          </div>
        </div>

        <div class="p-5 rounded-2xl bg-[#0a0a0a] border border-neutral-800 transition-colors hover:border-neutral-700">
          <p class="text-[9px] font-black text-neutral-500 uppercase tracking-widest mb-1">Private / Drafts</p>
          <div class="flex items-end justify-between">
            <h3 class="text-2xl font-black text-neutral-200 tracking-tighter">{{ stats?.private_recipes || 0 }}</h3>
            <Lock class="h-4 w-4 text-neutral-700 mb-1" />
          </div>
        </div>

        <div class="p-5 rounded-2xl bg-primary/2 border border-primary/20 transition-colors hover:border-primary/40">
          <p class="text-[9px] font-black text-primary/60 uppercase tracking-widest mb-1">Total Reach</p>
          <div class="flex items-end justify-between">
            <h3 class="text-2xl font-black text-white tracking-tighter">{{ stats?.total_views || 0 }}</h3>
            <Eye class="h-4 w-4 text-primary/40 mb-1" />
          </div>
        </div>
      </div>

      <section>
        <h2 class="text-[10px] font-black uppercase tracking-[0.2em] text-neutral-500 mb-6">Recent Work</h2>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <div
              v-for="recipe in recentRecipes"
              :key="recipe.id"
              class="flex gap-4 p-4 rounded-2xl bg-[#0f0f0f] border border-neutral-800 hover:border-neutral-700 transition-all group cursor-pointer"
          >
            <div class="w-16 h-16 rounded-xl bg-neutral-900 overflow-hidden shrink-0 border border-neutral-800">
              <img v-if="recipe.image_url" :src="$apiUrl + recipe.image_url" class="w-full h-full object-cover" />
              <div v-else class="w-full h-full flex items-center justify-center text-neutral-700 font-black bg-neutral-800/50">
                {{ recipe.title[0] }}
              </div>
            </div>

            <div class="flex-1 min-w-0 flex flex-col justify-center">
              <h4 class="font-bold text-neutral-200 group-hover:text-white transition-colors truncate">
                {{ recipe.title }}
              </h4>
              <div class="flex items-center gap-3 mt-1">
                <span class="text-[9px] font-black uppercase tracking-tighter text-neutral-500">
                  {{ recipe.servings }} Servings
                </span>
                <span class="w-1 h-1 rounded-full bg-neutral-800" />
                <span class="text-[9px] font-black uppercase tracking-tighter text-neutral-500">
                  {{ recipe.prep_time_minutes + recipe.cook_time_minutes }} MINS
                </span>
              </div>
            </div>
          </div>
        </div>
      </section>
    </template>
  </div>
</template>