<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { getRecipesByAuthor } from '@/api/recipe'
import { getUserById } from '@/api/user.ts' // You'll need this endpoint
import ProfileView from './ProfileView.vue'
import RecipeCard from '@/components/recipe/RecipeCard.vue'
import type { RecipeView } from '@/models/Recipe'
import type {User} from "@/models/User.ts";

const route = useRoute()
const authStore = useAuthStore()

const user = ref<User | null>(null)
const recipes = ref<RecipeView[]>([])
const loading = ref(true)

// Check if the ID in the URL matches the logged-in user's ID
const isOwnProfile = computed(() => {
  return authStore.user?.id === route.params.id
})

const loadProfileData = async (userId: string) => {
  loading.value = true
  try {
    if (authStore.user?.id === userId) {
      user.value = authStore.user
    } else {
      user.value = await getUserById(userId)
    }

    recipes.value = await getRecipesByAuthor(userId)
  } catch (error) {
    console.error("Error loading profile:", error)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  if (route.params.id) {
    loadProfileData(route.params.id as string)
  }
})

watch(() => route.params.id, (newId) => {
  if (newId) loadProfileData(newId as string)
})
</script>

<template>
  <ProfileView
      :user="user"
      :is-own-profile="isOwnProfile"
      :recipes-count="recipes.length"
  >
    <template #recipes>
      <div v-if="loading" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
        <div v-for="i in 4" :key="i" class="h-64 rounded-xl bg-muted animate-pulse"></div>
      </div>

      <div v-else-if="recipes.length > 0" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
        <RecipeCard
            v-for="recipe in recipes"
            :key="recipe.id"
            :recipe="recipe"
        />
      </div>
    </template>
  </ProfileView>
</template>