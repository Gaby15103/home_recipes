<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { Search, Filter as FilterIcon, Plus, Loader2, RotateCcw } from 'lucide-vue-next'
import RecipeList from "@/pages/studio/RecipeList.vue"
import { getStudioRecipes } from '@/api/studio'
import type { RecipeFilter, RecipeView } from "@/models/Recipe"
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger } from "@/components/ui/sheet";

const recipes = ref<RecipeView[]>([])
const isLoading = ref(true)
const totalEntries = ref(0)
const page = ref(1)
const perPage = ref(10)

const defaultFilters: RecipeFilter = {
  search: null,
  ingredient: [],
  tags: [],
  minPrep: null,
  maxPrep: null,
  minCook: null,
  maxCook: null,
  minSteps: null,
  maxSteps: null,
  dateFrom: null,
  dateTo: null,
}

const filters = ref<RecipeFilter>({ ...defaultFilters })

async function loadRecipes() {
  try {
    isLoading.value = true
    const response = await getStudioRecipes(page.value, perPage.value, filters.value)
    recipes.value = response
    totalEntries.value = response?.length ?? 0
  } catch (e) {
    console.error("Failed to sync studio recipes:", e)
    recipes.value = []
    totalEntries.value = 0
  } finally {
    isLoading.value = false
  }
}

function resetFilters() {
  filters.value = { ...defaultFilters }
  page.value = 1
  loadRecipes()
}

let timeout: any
watch(filters, () => {
  clearTimeout(timeout)
  timeout = setTimeout(() => {
    page.value = 1
    loadRecipes()
  }, 400)
}, { deep: true })

watch(page, loadRecipes)

onMounted(loadRecipes)
</script>

<template>
  <div class="max-w-7xl mx-auto">
    <div class="space-y-6">
      <div class="flex items-center gap-4">
        <div class="relative flex-1">
          <Search class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-neutral-600" />
          <input
              v-model="filters.search"
              placeholder="Search your recipes..."
              class="w-full bg-[#0a0a0a] border border-neutral-800 rounded-xl py-2 pl-10 pr-4 text-xs text-neutral-200 outline-none focus:border-primary transition-all"
          />
        </div>

        <Sheet>
          <SheetTrigger as-child>
            <button class="p-2.5 bg-[#0a0a0a] border border-neutral-800 rounded-xl text-neutral-400 hover:text-primary transition-all relative">
              <FilterIcon class="h-4 w-4" />
              <span v-if="Object.values(filters).some(v => v !== null && v !== '' && (Array.isArray(v) ? v.length > 0 : true))"
                    class="absolute top-1 right-1 h-1.5 w-1.5 bg-primary rounded-full"></span>
            </button>
          </SheetTrigger>
          <SheetContent class="bg-[#0a0a0a] border-l border-neutral-800 text-neutral-200">
            <SheetHeader>
              <SheetTitle class="text-white font-black uppercase tracking-widest text-sm">Advanced Filters</SheetTitle>
            </SheetHeader>

            <div class="space-y-6 py-6">
              <div class="space-y-2">
                <label class="text-[10px] font-black uppercase text-neutral-500">Prep Time (Mins)</label>
                <div class="grid grid-cols-2 gap-2">
                  <input v-model.number="filters.minPrep" type="number" placeholder="Min" class="bg-neutral-900 border-neutral-800 rounded-lg p-2 text-xs" />
                  <input v-model.number="filters.maxPrep" type="number" placeholder="Max" class="bg-neutral-900 border-neutral-800 rounded-lg p-2 text-xs" />
                </div>
              </div>

              <div class="space-y-2">
                <label class="text-[10px] font-black uppercase text-neutral-500">Cook Time (Mins)</label>
                <div class="grid grid-cols-2 gap-2">
                  <input v-model.number="filters.minCook" type="number" placeholder="Min" class="bg-neutral-900 border-neutral-800 rounded-lg p-2 text-xs" />
                  <input v-model.number="filters.maxCook" type="number" placeholder="Max" class="bg-neutral-900 border-neutral-800 rounded-lg p-2 text-xs" />
                </div>
              </div>

              <div class="space-y-2">
                <label class="text-[10px] font-black uppercase text-neutral-500">Date Range</label>
                <div class="flex flex-col gap-2">
                  <input v-model="filters.dateFrom" type="date" class="bg-neutral-900 border-neutral-800 rounded-lg p-2 text-xs" />
                  <input v-model="filters.dateTo" type="date" class="bg-neutral-900 border-neutral-800 rounded-lg p-2 text-xs" />
                </div>
              </div>

              <div class="pt-4 border-t border-neutral-800 flex flex-col gap-2">
                <button @click="loadRecipes" class="w-full py-2 bg-primary text-black text-[10px] font-black uppercase rounded-lg">
                  Apply Filters
                </button>
                <button @click="resetFilters" class="w-full py-2 flex items-center justify-center gap-2 text-[10px] font-black uppercase text-neutral-500 hover:text-white transition-colors">
                  <RotateCcw class="h-3 w-3" />
                  Reset All
                </button>
              </div>
            </div>
          </SheetContent>
        </Sheet>

        <router-link to="/studio/recipes/create" class="p-2.5 bg-primary rounded-xl text-black hover:bg-primary/90 transition-all">
          <Plus class="h-4 w-4 stroke-3" />
        </router-link>
      </div>

      <div v-if="isLoading && (recipes?.length === 0)" class="flex flex-col items-center justify-center py-20 text-neutral-600">
        <Loader2 class="h-8 w-8 animate-spin mb-4" />
        <p class="text-[10px] font-black uppercase tracking-widest">Synchronizing Collection...</p>
      </div>

      <div v-else>
        <RecipeList :recipes="recipes" />

        <div class="mt-6 flex items-center justify-between border-t border-neutral-800 pt-4">
        <span class="text-[10px] font-bold text-neutral-600 uppercase tracking-widest">
          Showing {{ recipes?.length ?? 0 }} of {{ totalEntries }}
        </span>
          <div class="flex items-center gap-2">
            <button
                @click="page--"
                :disabled="page === 1"
                class="px-3 py-1 bg-[#0a0a0a] border border-neutral-800 rounded-lg text-[10px] font-black uppercase text-neutral-400 disabled:opacity-30"
            >
              Prev
            </button>
            <span class="text-xs font-mono text-primary px-2">{{ page }}</span>
            <button
                @click="page++"
                :disabled="(recipes?.length ?? 0) < perPage"
                class="px-3 py-1 bg-[#0a0a0a] border border-neutral-800 rounded-lg text-[10px] font-black uppercase text-neutral-400 disabled:opacity-30"
            >
              Next
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>