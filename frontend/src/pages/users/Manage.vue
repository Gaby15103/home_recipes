<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import { getAllRecipes } from '@/api/recipe' // Ensure this exists in your api/recipe.ts
import type { RecipeView } from '@/models/Recipe'
import RecipeTable from '@/components/recipe/RecipeTable.vue'
import MainLayout from '@/layouts/MainLayout.vue'
import { Button } from '@/components/ui/button'
import { ROUTES } from '@/router/routes'
import { Plus, UtensilsCrossed } from 'lucide-vue-next'

const { t } = useI18n()
const authStore = useAuthStore()

// State management matching RecipeTable props
const recipes = ref<RecipeView[]>([])
const loading = ref(true)
const page = ref(1)
const perPage = ref(10)
const total = ref(0)

async function fetchUserRecipes() {
  loading.value = true
  try {
    // We pass the current user's ID to filter the results
    // Assuming your API supports an author_id filter
    const response = await getAllRecipes({
      page: page.value,
      per_page: perPage.value,
      author_id: authStore.user?.id
    })

    recipes.value = response.data.items
    total.value = response.data.total
  } catch (error) {
    console.error("Failed to fetch recipes:", error)
  } finally {
    loading.value = false
  }
}

// Pagination handlers emitted by RecipeTable
const handleNext = () => {
  page.value++
  fetchUserRecipes()
}

const handlePrev = () => {
  if (page.value > 1) {
    page.value--
    fetchUserRecipes()
  }
}

onMounted(() => {
  fetchUserRecipes()
})
</script>

<template>
  <MainLayout>
    <div class="container py-10 space-y-8">
      <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <div>
          <h1 class="text-3xl font-bold tracking-tight">{{ t('user.recipes.title') }}</h1>
          <p class="text-muted-foreground">{{ t('user.recipes.description') }}</p>
        </div>

        <Button as-child>
          <RouterLink :to="ROUTES.USER.CREATE_RECIPE">
            <Plus class="mr-2 h-4 w-4" /> {{ t('user.recipes.add_new') }}
          </RouterLink>
        </Button>
      </div>

      <div v-if="!loading && recipes.length === 0" class="flex flex-col items-center justify-center py-20 border-2 border-dashed rounded-xl">
        <UtensilsCrossed class="h-12 w-12 text-muted-foreground mb-4" />
        <h3 class="text-lg font-medium">{{ t('user.recipes.empty_title') }}</h3>
        <p class="text-muted-foreground mb-6">{{ t('user.recipes.empty_description') }}</p>
        <Button variant="outline" as-child>
          <RouterLink :to="ROUTES.USER.CREATE_RECIPE">{{ t('user.recipes.create_first') }}</RouterLink>
        </Button>
      </div>

      <RecipeTable
          v-else
          mode="user"
          :recipes="recipes"
          :loading="loading"
          :page="page"
          :per-page="perPage"
          :total="total"
          @next-page="handleNext"
          @previous-page="handlePrev"
      />
    </div>
  </MainLayout>
</template>