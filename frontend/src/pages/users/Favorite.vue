<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { getFavorites } from '@/api/recipe'
import { useI18n } from 'vue-i18n'
import { Heart, Utensils, Search, ChefHat } from 'lucide-vue-next'
import { Input } from '@/components/ui/input'
import { Separator } from '@/components/ui/separator'
import type { RecipeView } from '@/models/Recipe'
import RecipeCard from '@/components/recipe/RecipeCard.vue'

const { t } = useI18n()
const route = useRoute()

const recipes = ref<RecipeView[]>([])
const loading = ref(true)
const searchQuery = ref('')

// If route has an ID, we are looking at someone else's favorites
const targetUserId = computed(() => route.params.id as string | undefined)

onMounted(async () => {
  try {
    recipes.value = await getFavorites(targetUserId.value)
  } catch (error) {
    console.error("Failed to load favorites", error)
  } finally {
    loading.value = false
  }
})

const filteredRecipes = computed(() => {
  if (!searchQuery.value) return recipes.value
  return recipes.value.filter(r =>
      r.title.toLowerCase().includes(searchQuery.value.toLowerCase())
  )
})
</script>

<template>
  <div class="min-h-screen pb-20 animate-in fade-in duration-700">
    <div class="w-full h-32 bg-gradient-to-b from-primary/5 to-transparent border-b/50"></div>

    <div class="container max-w-7xl -mt-16 space-y-10 relative z-10">

      <div class="flex flex-col lg:flex-row lg:items-end justify-between gap-8 bg-background/60 backdrop-blur-md p-8 rounded-3xl border shadow-sm">
        <div class="space-y-4">
          <div class="flex items-center gap-4">
            <div class="p-3 bg-primary/10 rounded-2xl">
              <Heart class="h-8 w-8 text-primary fill-primary/20" />
            </div>
            <div>
              <h1 class="text-4xl font-black tracking-tight uppercase">
                {{ targetUserId ? t('profile.favorites.title_other', { name: 'Chef' }) : t('profile.favorites.title_mine') }}
              </h1>
              <p class="text-muted-foreground font-medium flex items-center gap-2">
                <ChefHat class="h-4 w-4" />
                {{ t('Ingredients.selected_count', { count: filteredRecipes.length }) }}
              </p>
            </div>
          </div>
        </div>

        <div class="relative w-full lg:w-96">
          <Search class="absolute left-4 top-1/2 -translate-y-1/2 h-5 w-5 text-muted-foreground" />
          <Input
              v-model="searchQuery"
              :placeholder="t('profile.favorites.search_placeholder')"
              class="pl-12 h-14 rounded-2xl bg-secondary/30 border-2 border-transparent focus-visible:border-primary/30 focus-visible:ring-0 transition-all text-lg"
          />
        </div>
      </div>

      <div v-if="!loading && filteredRecipes.length === 0" class="py-32 text-center border-4 border-dashed rounded-[3rem] bg-secondary/5 transition-all">
        <Utensils class="h-20 w-20 mx-auto text-muted-foreground/10 mb-6" />
        <h3 class="text-2xl font-bold text-muted-foreground">{{ t('Admin.table.noRecipes') }}</h3>
        <p class="text-muted-foreground/60 max-w-xs mx-auto mt-2">
          {{ t('profile.public.no_recipes') }}
        </p>
      </div>

      <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-8">
        <div v-for="i in 4" :key="i" class="h-[400px] rounded-[2rem] bg-secondary/20 animate-pulse"></div>
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-8">
        <RecipeCard
            v-for="recipe in filteredRecipes"
            :key="recipe.id"
            :recipe="recipe"
            class="hover:translate-y-[-4px] transition-transform duration-300"
        />
      </div>
    </div>
  </div>
</template>