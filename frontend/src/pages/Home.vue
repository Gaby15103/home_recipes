<script setup lang="ts">
import {RouterLink} from "vue-router"
import {Button} from "@/components/ui/button"
import {Card, CardDescription, CardHeader, CardTitle} from "@/components/ui/card"
import {useAuthStore} from "@/stores/auth"
import {ROUTES} from "@/router/routes.ts"
import {useI18n} from "vue-i18n"
import {BookOpen, Clock, Flame, Plus, Search, Users, Utensils} from "lucide-vue-next"
import {onMounted, ref} from "vue";
import {getRecentRecipes} from "@/api";
import type {RecipeView} from "@/models/Recipe.ts";

const {t} = useI18n()
const authStore = useAuthStore()

const recentRecipes = ref<RecipeView[]>([])
const isLoading = ref(true)
const API_URL = import.meta.env.VITE_API_URL // Ensure this matches your env

onMounted(async () => {
  try {
    // Fetch last 4 recipes with translations
    recentRecipes.value = await getRecentRecipes(4, false)
  } catch (error) {
    console.error("Failed to fetch recent recipes:", error)
  } finally {
    isLoading.value = false
  }
})
</script>

<template>
  <div class="space-y-12 pb-10">
    <section v-if="authStore.isAuthenticated" class="space-y-12">
      <div class="flex flex-col gap-2">
        <div class="inline-flex items-center w-fit rounded-full border px-2.5 py-0.5 text-xs font-semibold bg-primary/10 text-primary mb-2">
          {{ t('Home.BadgeNew') }} 🚀
        </div>
        <h1 class="text-3xl font-bold tracking-tight md:text-5xl">
          {{ t('Home.WelcomeBack', {username: authStore.user?.username}) }} 🍳
        </h1>
        <p class="text-muted-foreground text-lg max-w-2xl">
          {{ t('Home.ReadyToCook') }}
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card class="border-primary/20 bg-primary/5 hover:bg-primary/10 transition-all hover:scale-[1.02] cursor-pointer group">
          <RouterLink :to="ROUTES.RECIPES + '/create'" class="h-full block">
            <CardHeader>
              <div class="p-2 w-fit rounded-lg bg-primary text-primary-foreground mb-2">
                <Plus class="h-6 w-6"/>
              </div>
              <CardTitle>{{ t('Home.CreateRecipe') }}</CardTitle>
              <CardDescription>{{ t('Home.CreateRecipeDesc') }}</CardDescription>
            </CardHeader>
          </RouterLink>
        </Card>

        <Card class="hover:bg-accent transition-all hover:scale-[1.02] cursor-pointer">
          <RouterLink :to="ROUTES.RECIPES" class="h-full block">
            <CardHeader>
              <div class="p-2 w-fit rounded-lg bg-orange-500 text-white mb-2">
                <BookOpen class="h-6 w-6"/>
              </div>
              <CardTitle>{{ t('Home.MyLibrary') }}</CardTitle>
              <CardDescription>{{ t('Home.MyLibraryDesc') }}</CardDescription>
            </CardHeader>
          </RouterLink>
        </Card>

        <Card class="hover:bg-accent transition-all hover:scale-[1.02] cursor-pointer">
          <RouterLink :to="ROUTES.RECIPES" class="h-full block">
            <CardHeader>
              <div class="p-2 w-fit rounded-lg bg-blue-500 text-white mb-2">
                <Search class="h-6 w-6"/>
              </div>
              <CardTitle>{{ t('Home.Explore') }}</CardTitle>
              <CardDescription>{{ t('Home.ExploreDesc') }}</CardDescription>
            </CardHeader>
          </RouterLink>
        </Card>
      </div>

      <div class="pt-4">
        <div class="flex items-center justify-between mb-6 border-b pb-4">
          <h2 class="text-2xl font-semibold tracking-tight">{{ t('Home.RecentHeading') }}</h2>
          <Button variant="ghost" class="text-primary hover:text-primary" as-child>
            <RouterLink :to="ROUTES.RECIPES">{{ t('Home.ViewAll') }} →</RouterLink>
          </Button>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <template v-if="isLoading">
            <div v-for="i in 4" :key="i" class="rounded-xl border bg-card p-2 animate-pulse">
              <div class="aspect-video rounded-lg bg-muted mb-3"></div>
              <div class="px-2 space-y-2">
                <div class="h-4 bg-muted rounded w-3/4"></div>
                <div class="h-3 bg-muted rounded w-1/2"></div>
              </div>
            </div>
          </template>

          <template v-else>
            <RouterLink
                v-for="recipe in recentRecipes"
                :key="recipe.id"
                :to="`${ROUTES.RECIPE(recipe.id)}`"
                class="group relative rounded-xl border bg-card p-2 shadow-sm transition-all hover:shadow-md hover:-translate-y-1"
            >
              <div class="aspect-video rounded-lg bg-muted mb-3 overflow-hidden">
                <img
                    v-if="recipe.image_url"
                    :src="`${$apiUrl}${recipe.image_url}`"
                    class="w-full h-full object-cover transition-transform group-hover:scale-110"
                />
                <div v-else class="w-full h-full flex items-center justify-center text-muted-foreground/40">
                  <Utensils class="h-10 w-10"/>
                </div>
              </div>
              <div class="px-2 pb-2">
                <h3 class="font-medium leading-none mb-2 truncate">{{ recipe.title }}</h3>
                <div class="flex items-center gap-3 text-xs text-muted-foreground">
                  <span class="flex items-center gap-1"><Clock class="h-3 w-3"/> {{ (recipe.prep_time_minutes || 0) + (recipe.cook_time_minutes || 0) }}m</span>
                  <span class="flex items-center gap-1"><Flame class="h-3 w-3"/> {{ recipe.servings }}</span>
                </div>
              </div>
            </RouterLink>
          </template>
        </div>
      </div>

      <div class="mt-16 bg-muted/30 rounded-3xl p-8 border border-dashed">
        <h3 class="text-lg font-medium mb-8 text-center text-muted-foreground uppercase tracking-widest">{{ t('Home.FeaturesTitle') || 'Discover More' }}</h3>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-12">
          <div class="flex flex-col items-center text-center space-y-3">
            <div class="p-3 bg-white rounded-2xl shadow-sm border">
              <Users class="h-6 w-6 text-primary"/>
            </div>
            <h4 class="font-bold">{{ t('Home.Feature1Title') }}</h4>
            <p class="text-sm text-muted-foreground">{{ t('Home.Feature1Desc') }}</p>
          </div>
          <div class="flex flex-col items-center text-center space-y-3">
            <div class="p-3 bg-white rounded-2xl shadow-sm border">
              <Flame class="h-6 w-6 text-orange-500"/>
            </div>
            <h4 class="font-bold">{{ t('Home.Feature2Title') }}</h4>
            <p class="text-sm text-muted-foreground">{{ t('Home.Feature2Desc') }}</p>
          </div>
          <div class="flex flex-col items-center text-center space-y-3">
            <div class="p-3 bg-white rounded-2xl shadow-sm border">
              <Clock class="h-6 w-6 text-blue-500"/>
            </div>
            <h4 class="font-bold">{{ t('Home.Feature3Title') }}</h4>
            <p class="text-sm text-muted-foreground">{{ t('Home.Feature3Desc') }}</p>
          </div>
        </div>
      </div>
    </section>

    <section v-else class="flex flex-col items-center justify-center text-center py-20 px-4">
      <div
          class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors bg-primary text-primary-foreground mb-6">
        {{ t('Home.BadgeNew') }} 🚀
      </div>
      <h1 class="text-4xl font-extrabold tracking-tight sm:text-6xl mb-6 bg-linear-to-r from-primary to-orange-600 bg-clip-text text-transparent">
        {{ t('Home.Title') }}
      </h1>
      <p class="text-xl text-muted-foreground mb-10 max-w-2xl leading-relaxed">
        {{ t('Home.Text') }}
      </p>
      
      <div class="flex flex-col sm:flex-row gap-4 w-full justify-center">
        <RouterLink :to="ROUTES.LOGIN">
          <Button size="lg" class="w-full sm:w-auto px-10 h-12 text-lg">
            {{ t('Login') }}
          </Button>
        </RouterLink>
        <RouterLink :to="ROUTES.REGISTER">
          <Button size="lg" variant="outline" class="w-full sm:w-auto px-10 h-12 text-lg">
            {{ t('Register') }}
          </Button>
        </RouterLink>
      </div>
      <div class="mt-24 grid grid-cols-1 sm:grid-cols-3 gap-8 w-full max-w-4xl text-left border-t pt-12">
        <div class="space-y-2">
          <Users class="h-8 w-8 text-primary"/>
          <h3 class="font-bold">{{ t('Home.Feature1Title') }}</h3>
          <p class="text-sm text-muted-foreground">{{ t('Home.Feature1Desc') }}</p>
        </div>
        <div class="space-y-2">
          <Flame class="h-8 w-8 text-orange-500"/>
          <h3 class="font-bold">{{ t('Home.Feature2Title') }}</h3>
          <p class="text-sm text-muted-foreground">{{ t('Home.Feature2Desc') }}</p>
        </div>
        <div class="space-y-2">
          <Clock class="h-8 w-8 text-blue-500"/>
          <h3 class="font-bold">{{ t('Home.Feature3Title') }}</h3>
          <p class="text-sm text-muted-foreground">{{ t('Home.Feature3Desc') }}</p>
        </div>
      </div>
    </section>
  </div>
</template>