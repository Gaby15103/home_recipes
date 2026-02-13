<script setup lang="ts">
import {onMounted, ref, watch} from "vue"
import { BookOpen, Folder, LockIcon, LockOpen, Plus, Tag } from "lucide-vue-next"
import { Button } from "@/components/ui/button"
import RecipeTable from "@/pages/Admin/Recipes/RecipeTable.vue"
import { getAllRecipesByPage } from "@/api/recipe.ts"
import type { RecipeView } from "@/models/Recipe.ts"
import router from "@/router";
import {useI18n} from "vue-i18n";
const { t,locale } = useI18n()

watch(locale, () => {
  fetchRecipes();
});
// ------------------------- Stats -------------------------
const stats = ref([
  { label: t('Admin.dashboard.stats.totalRecipes'), value: 124, icon: BookOpen },
  { label: t('Admin.dashboard.stats.public'), value: 92, icon: LockOpen },
  { label: t('Admin.dashboard.stats.private'), value: 32, icon: LockIcon },
  { label: t('Admin.dashboard.stats.tags'), value: 8, icon: Tag },
])

// ------------------------- Pagination State -------------------------
const recipes = ref<RecipeView[]>([])
const totalRecipes = ref(0)
const page = ref(1)
const perPage = ref(10)
const loading = ref(false)

// ------------------------- Fetch Recipes -------------------------
async function fetchRecipes() {
  try {
    loading.value = true

    const res = await getAllRecipesByPage(page.value, perPage.value)

    recipes.value = res.data
    totalRecipes.value = res.total
  } catch (err) {
    console.error("Failed to load recipes", err)
  } finally {
    loading.value = false
  }
}

// ------------------------- Pagination Handlers -------------------------
function nextPage() {
  const maxPage = Math.ceil(totalRecipes.value / perPage.value)
  if (page.value < maxPage) {
    page.value++
    fetchRecipes()
  }
}

function previousPage() {
  if (page.value > 1) {
    page.value--
    fetchRecipes()
  }
}

// Initial load
onMounted(fetchRecipes)

function goToCreate() {
  router.push("/admin/recipe/create")
}
</script>

<template>
  <div class="container mx-auto p-4">
    <div class="space-y-8">

      <!-- ===== Top Section ===== -->
      <div class="grid grid-cols-1 gap-6 xl:grid-cols-4">

        <!-- Stats -->
        <div class="xl:col-span-3 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <div
              v-for="stat in stats"
              :key="stat.label"
              class="rounded-xl border bg-card p-4 shadow-sm flex items-center gap-4"
          >
            <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-muted">
              <component :is="stat.icon" class="h-6 w-6"/>
            </div>

            <div>
              <p class="text-2xl font-semibold">{{ stat.value }}</p>
              <p class="text-sm text-muted-foreground">{{ stat.label }}</p>
            </div>
          </div>
        </div>

        <!-- Quick Actions -->
        <div class="space-y-4">
          <h3 class="text-sm font-medium text-muted-foreground">
            {{ t('Admin.dashboard.quickActions') }}
          </h3>

          <div class="flex flex-col gap-2">
            <Button class="justify-start gap-2" @click="goToCreate">
              <Plus class="h-4 w-4"/>
              {{ t('Admin.dashboard.createRecipe') }}
            </Button>

            <Button variant="secondary" class="justify-start gap-2">
              <Folder class="h-4 w-4"/>
              {{ t('Admin.dashboard.manageCategories') }}
            </Button>

            <Button variant="secondary" class="justify-start gap-2">
              <Tag class="h-4 w-4"/>
              {{ t('Admin.dashboard.manageTags') }}
            </Button>
          </div>
        </div>
      </div>

      <!-- ===== Recipe Table ===== -->
      <div class="rounded-xl border bg-card p-4">
        <div class="flex items-center justify-between border-b px-4 py-3 mb-4">
          <h2 class="font-semibold">
            {{ t('Admin.dashboard.recentRecipes') }}
          </h2>
          <Button variant="ghost" size="sm">
            {{ t('Admin.dashboard.viewAll') }}
          </Button>
        </div>

        <RecipeTable
            :recipes="recipes"
            :page="page"
            :per-page="perPage"
            :total="totalRecipes"
            :loading="loading"
            @next-page="nextPage"
            @previous-page="previousPage"
        />

      </div>
    </div>
  </div>
</template>
