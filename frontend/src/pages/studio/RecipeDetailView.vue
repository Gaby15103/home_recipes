<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import {
  ChevronLeft, Clock, Edit3, Eye, Hash, Printer,
  Share2, Trash2, Zap, AlertTriangle, Loader2
} from 'lucide-vue-next'
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import RecipeDisplay from "@/components/recipe/RecipeDisplay.vue"
import { deleteRecipe, getRecipeById } from "@/api/recipe"
import { getRecipeAnalytics, type RecipeAnalytics } from "@/api/studio.ts";
import { ROUTES } from "@/router/routes.ts";

const route = useRoute()
const router = useRouter()
const recipe = ref<any>(null)
const analytics = ref<RecipeAnalytics | null>(null)
const isLoading = ref(true)

// Deletion State
const showDeleteConfirm = ref(false)
const isDeleting = ref(false)

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleDateString('en-CA', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  })
}

onMounted(async () => {
  try {
    let recipe_id = route.params.id as string
    const [recipeData, analyticsData] = await Promise.all([
      getRecipeById(recipe_id),
      getRecipeAnalytics(recipe_id)
    ])

    recipe.value = recipeData
    analytics.value = analyticsData
  } catch (error) {
    console.error("Failed to compile production data:", error)
  } finally {
    isLoading.value = false
  }
})

const handleDecommission = async () => {
  if (!recipe.value) return

  isDeleting.value = true
  try {
    await deleteRecipe(recipe.value.id)
    await router.push(ROUTES.STUDIO.MY_RECIPES)
  } catch (error) {
    console.error("Decommission failed:", error)
    alert("System error: Failed to purge record from PostgreSQL.")
  } finally {
    isDeleting.value = false
    showDeleteConfirm.value = false
  }
}
</script>

<template>
  <div v-if="!isLoading && recipe && analytics" class="min-h-screen space-y-6 relative">

    <div v-if="showDeleteConfirm" class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
      <div class="w-full max-w-md rounded-[2.5rem] border border-red-900/30 bg-[#0a0a0a] p-8 shadow-2xl">
        <div class="flex items-center gap-4 text-red-500 mb-6">
          <div class="p-3 rounded-2xl bg-red-500/10">
            <AlertTriangle class="h-6 w-6" />
          </div>
          <h2 class="text-xl font-black uppercase italic tracking-tighter">Confirm Purge<span class="text-white">.</span></h2>
        </div>

        <p class="text-[13px] text-neutral-400 leading-relaxed mb-8">
          You are about to decommission <span class="text-white font-mono bg-white/5 px-2 py-0.5 rounded">{{ recipe.id.split('-')[0] }}</span>.
          This action will permanently drop all associated OCR data, translations, and analytics from the production environment.
        </p>

        <div class="flex flex-col gap-3">
          <Button
              variant="destructive"
              class="w-full rounded-xl font-black uppercase py-6"
              :disabled="isDeleting"
              @click="handleDecommission"
          >
            <Loader2 v-if="isDeleting" class="mr-2 h-4 w-4 animate-spin" />
            <Trash2 v-else class="mr-2 h-4 w-4" />
            Confirm Decommission
          </Button>
          <Button
              variant="ghost"
              class="w-full rounded-xl font-black uppercase text-neutral-500 hover:text-white"
              :disabled="isDeleting"
              @click="showDeleteConfirm = false"
          >
            Abort Mission
          </Button>
        </div>
      </div>
    </div>

    <header class="flex flex-wrap items-center justify-between gap-4 border-b border-white/5 pb-6">
      <div class="flex items-center gap-4">
        <Button variant="ghost" size="icon" @click="router.back()" class="rounded-xl border border-white/5">
          <ChevronLeft class="h-4 w-4" />
        </Button>
        <div>
          <h1 class="text-2xl font-black tracking-tighter text-white uppercase italic">
            Production<span class="text-primary">.</span>Log
          </h1>
          <div class="flex items-center gap-2 text-[9px] font-bold uppercase tracking-widest text-neutral-500">
            <Hash class="h-3 w-3" /> {{ recipe.id.split('-')[0] }}
            <span class="text-neutral-800">/</span>
            <span :class="recipe.is_private ? 'text-orange-500' : 'text-green-500'">
              {{ recipe.is_private ? 'Internal Build' : 'Production Live' }}
            </span>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <Button variant="outline" class="rounded-xl border-neutral-800 text-[10px] font-black uppercase tracking-tight">
          <Share2 class="mr-2 h-3.5 w-3.5" /> {{ analytics.share_count }} Shares
        </Button>
        <Button @click="router.push(`/studio/recipes/${recipe.id}/edit`)" class="rounded-xl bg-white text-black hover:bg-neutral-200 text-[10px] font-black uppercase tracking-tight">
          <Edit3 class="mr-2 h-3.5 w-3.5" /> Modify Source
        </Button>
      </div>
    </header>

    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
      <section class="lg:col-span-7 xl:col-span-8 space-y-4">
        <div class="flex items-center justify-between px-2">
          <span class="text-[10px] font-black uppercase tracking-[0.2em] text-neutral-500">Output Preview</span>
          <Badge variant="secondary" class="text-[9px] font-mono opacity-50">HEALTH_SCORE: {{ analytics.health_score.toFixed(0) }}%</Badge>
        </div>
        <div class="rounded-[2.5rem] bg-[#0a0a0a] border border-white/5 p-8 md:p-12 shadow-2xl overflow-hidden">
          <RecipeDisplay :recipe="recipe" :multiplier="1" />
        </div>
      </section>

      <aside class="lg:col-span-5 xl:col-span-4 space-y-6">
        <div class="grid grid-cols-2 gap-3">
          <div class="p-4 rounded-2xl bg-[#0a0a0a] border border-white/5">
            <p class="text-[9px] font-black text-neutral-500 uppercase mb-2">Total Impressions</p>
            <div class="flex items-center gap-2">
              <Eye class="h-4 w-4 text-primary" />
              <span class="text-xl font-black text-white">{{ analytics.total_views.toLocaleString() }}</span>
            </div>
            <div class="mt-4 flex items-end gap-1 h-8">
              <div v-for="(v, i) in analytics.views_7d" :key="i"
                   class="bg-primary/20 w-full rounded-t-sm hover:bg-primary transition-colors"
                   :style="{ height: `${(v / Math.max(...analytics.views_7d, 1)) * 100}%` }"
                   :title="`${v} views`"
              ></div>
            </div>
          </div>
          <div class="p-4 rounded-2xl bg-[#0a0a0a] border border-white/5">
            <p class="text-[9px] font-black text-neutral-500 uppercase mb-2">Avg Session</p>
            <div class="flex items-center gap-2">
              <Clock class="h-4 w-4 text-primary" />
              <span class="text-xl font-black text-white">{{ analytics.avg_session_duration }}</span>
            </div>
            <div class="mt-4 pt-3 border-t border-white/5">
              <p class="text-[9px] font-black text-neutral-500 uppercase flex items-center gap-2">
                <Printer class="h-3 w-3" /> Hardcopies: {{ analytics.print_count }}
              </p>
            </div>
          </div>
        </div>

        <div class="rounded-2xl bg-[#0a0a0a] border border-white/5 overflow-hidden">
          <div class="p-4 border-b border-white/5 bg-white/2">
            <h3 class="text-[10px] font-black uppercase tracking-widest text-neutral-400 flex items-center gap-2">
              <Zap class="h-3.5 w-3.5 text-primary" /> Metadata Health
            </h3>
          </div>
          <div class="p-5 space-y-4">
            <div class="flex justify-between items-center">
              <span class="text-[11px] font-medium text-neutral-500">Primary Language</span>
              <Badge class="uppercase font-black text-[9px] bg-primary/10 text-primary border-none">{{ recipe.primary_language }}</Badge>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-[11px] font-medium text-neutral-500">Translations</span>
              <div class="flex gap-1">
                <span v-for="t in recipe.translations" :key="t.language_code" class="w-5 h-5 rounded-md bg-neutral-800 flex items-center justify-center text-[8px] font-black uppercase">
                  {{ t.language_code }}
                </span>
              </div>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-[11px] font-medium text-neutral-500">Last Synced</span>
              <span class="text-[11px] font-mono text-neutral-400">{{ formatDate(analytics.last_modified) }}</span>
            </div>
            <div class="space-y-1.5 pt-2">
              <div class="flex justify-between text-[9px] font-black uppercase">
                <span class="text-neutral-500">Optimization</span>
                <span class="text-primary">{{ analytics.health_score.toFixed(0) }}%</span>
              </div>
              <div class="h-1 w-full bg-neutral-900 rounded-full overflow-hidden">
                <div class="h-full bg-primary transition-all duration-1000" :style="{ width: `${analytics.health_score}%` }"></div>
              </div>
            </div>
          </div>
        </div>

        <div class="p-6 rounded-2xl border border-red-900/20 bg-red-950/5 space-y-4">
          <h4 class="text-[10px] font-black uppercase tracking-widest text-red-500/80">Danger Zone</h4>
          <p class="text-[11px] text-neutral-500 leading-relaxed">
            Deleting this production will permanently remove all OCR training data and associated imagery from the PostgreSQL instance.
          </p>
          <Button
              variant="destructive"
              @click="showDeleteConfirm = true"
              class="w-full rounded-xl bg-red-950/20 hover:bg-red-600 hover:text-white border border-red-900/30 text-[10px] font-black uppercase"
          >
            <Trash2 class="mr-2 h-3.5 w-3.5" /> Decommission Recipe
          </Button>
        </div>
      </aside>
    </div>
  </div>

  <div v-else class="flex flex-col items-center justify-center py-20 gap-4">
    <Zap class="h-8 w-8 text-primary animate-pulse" />
    <p class="text-[10px] font-black uppercase tracking-[0.4em] text-neutral-500">Compiling Production Data...</p>
  </div>
</template>