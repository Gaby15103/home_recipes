<script setup lang="ts">
import { ref } from 'vue'
import { Search, Filter as FilterIcon } from 'lucide-vue-next'
import RecipeList from "@/pages/studio/RecipeList.vue"

// Define the interface to match your SeaORM / Rust Backend
interface Recipe {
  id: number
  title: string
  status: 'draft' | 'published' | 'private'
  updated_at: string
  views: number
}

// Initialize with some mock data for the UI work
const mockRecipes = ref<Recipe[]>([
  {
    id: 1,
    title: "Grandma's Traditional Tourtière",
    status: 'published',
    updated_at: '2026-04-12',
    views: 1240
  },
  {
    id: 2,
    title: "Quick Rust-Powered Ramen",
    status: 'draft',
    updated_at: '2026-04-14',
    views: 0
  },
  {
    id: 3,
    title: "Victorian Gingerbread House",
    status: 'private',
    updated_at: '2026-02-20',
    views: 45
  }
])

const searchQueries = ref("")
</script>

<template>
  <div class="space-y-6 animate-in fade-in duration-500">
    <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">

      <div class="relative w-full sm:w-72 group">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-neutral-600 group-focus-within:text-primary transition-colors" />
        <input
            v-model="searchQueries"
            type="text"
            placeholder="Search your collection..."
            class="w-full bg-[#0a0a0a] border border-neutral-800 rounded-xl py-2 pl-10 pr-4 text-xs text-neutral-200 focus:ring-1 focus:ring-primary focus:border-primary outline-none transition-all"
        />
      </div>

      <div class="flex items-center gap-3 w-full sm:w-auto">
        <div class="relative group">
          <button class="p-2.5 bg-[#0a0a0a] border border-neutral-800 rounded-xl text-neutral-400 hover:text-primary hover:border-primary/50 transition-all">
            <FilterIcon class="h-4 w-4" />
          </button>
        </div>

        <button class="flex-1 sm:flex-none bg-primary hover:bg-primary/90 text-black px-4 py-2 rounded-xl text-xs font-black uppercase tracking-tight transition-transform active:scale-95">
          New Recipe
        </button>
      </div>
    </div>

    <RecipeList :recipes="mockRecipes" />

    <div class="flex justify-between items-center px-2">
      <p class="text-[10px] font-bold text-neutral-600 uppercase tracking-widest">
        Showing {{ mockRecipes.length }} entries
      </p>
      <div class="flex gap-1">
        <div v-for="i in 3" :key="i" class="w-1.5 h-1.5 rounded-full bg-neutral-800" :class="{'bg-primary': i === 1}" />
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Ensure the page feels like a desktop app */
input::placeholder {
  color: #525252;
}
</style>