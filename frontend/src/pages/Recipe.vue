<script setup lang="ts">
import {onMounted} from "vue"
import {useRoute} from "vue-router"
import {getRecipeById} from "@/api/recipe"

import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@/components/ui/card"

import {Badge} from "@/components/ui/badge"
import {Separator} from "@/components/ui/separator"
import {Skeleton} from "@/components/ui/skeleton"

const route = useRoute();
const {recipe, loading, error, fetchRecipe} = getRecipeById(route.params.id as string);

onMounted(fetchRecipe);
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 py-8">
    <!-- Loading -->
    <Skeleton v-if="loading" class="h-96 w-full rounded-xl"/>

    <!-- Error -->
    <p v-else-if="error" class="text-red-500 text-center">
      {{ error }}
    </p>

    <!-- Recipe -->
    <div v-else-if="recipe" class="space-y-8">
      <!-- Header -->
      <Card>
        <CardHeader class="space-y-2">
          <CardTitle class="text-4xl">
            {{ recipe.title }}
          </CardTitle>

          <CardDescription class="text-base">
            {{ recipe.description }}
          </CardDescription>

          <!-- Meta -->
          <div class="flex flex-wrap gap-2 pt-2">
            <Badge variant="secondary">
              Servings: {{ recipe.serving }}
            </Badge>
            <Badge variant="outline">
              Prep {{ recipe.prep_time_minutes }} min
            </Badge>
            <Badge variant="outline">
              Cook {{ recipe.cook_time_minutes }} min
            </Badge>
          </div>
          <Separator class="my-3" />

          <div
              v-if="recipe.tags?.length"
              class="flex flex-wrap gap-2"
          >
            <Badge
                v-for="tag in recipe.tags"
                :key="tag.id"
                variant="outline"
                class="text-xs"
            >
              {{ tag.name }}
            </Badge>
          </div>

        </CardHeader>

      </Card>

      <!-- Image -->
      <Card v-if="recipe.image_url">
        <CardContent class="p-0">
          <img
              :src="$apiUrl + recipe.image_url"
              alt="Recipe image"
              class="w-full max-h-[420px] object-cover rounded-xl"
          />
        </CardContent>
      </Card>

      <!-- Main content -->
      <div class="grid md:grid-cols-3 gap-6">
        <!-- Ingredients -->
        <Card class="md:col-span-1">
          <CardHeader>
            <CardTitle>Ingredients</CardTitle>
          </CardHeader>

          <CardContent class="space-y-4">
            <div
                v-for="group in recipe.ingredient_groups"
                :key="group.position"
            >
              <h4
                  v-if="group.title"
                  class="font-medium mb-1"
              >
                {{ group.title }}
              </h4>

              <ul class="space-y-1 text-sm">
                <li
                    v-for="ing in group.ingredients"
                    :key="ing.position"
                    class="flex items-start gap-2"
                >
                  <!-- Dash -->
                  <span class="text-muted-foreground mt-[2px]">–</span>

                  <!-- Ingredient content -->
                  <div class="flex-1">
      <span class="font-medium">
        {{ ing.quantity }} {{ ing.unit }}
      </span>
                    {{ ing.name }}

                    <RouterLink
                        v-if="ing.note"
                        :to="{ hash: `#ingredient-${ing.position}` }"
                        class="ml-1 text-primary font-bold hover:underline"
                        title="See ingredient note"
                    >
                      *
                    </RouterLink>
                  </div>
                </li>
              </ul>
            </div>
          </CardContent>
        </Card>

        <!-- Steps -->
        <Card class="md:col-span-2">
          <CardHeader>
            <CardTitle>Steps</CardTitle>
          </CardHeader>

          <CardContent class="space-y-8">
            <div
                v-for="group in recipe.step_groups"
                :key="group.position"
                class="space-y-6"
            >
              <h4 v-if="group.title" class="font-medium text-lg">
                {{ group.title }}
              </h4>

              <ol class="space-y-6 list-decimal pl-5">
                <li
                    v-for="step in group.steps"
                    :key="step.position"
                    class="space-y-3"
                >
                  <!-- Step text -->
                  <div>
                    {{ step.instruction }}
                    <span
                        v-if="step.duration_minutes"
                        class="text-sm text-muted-foreground ml-1"
                    >
              ({{ step.duration_minutes }} min)
            </span>
                  </div>

                  <!-- Step image -->
                  <div
                      v-if="step.image_url"
                      class="rounded-lg overflow-hidden border"
                  >
                    <img
                        :src="$apiUrl + step.image_url"
                        alt="Step image"
                        class="w-full max-h-80 object-cover"
                    />
                  </div>
                </li>
              </ol>
              <Separator/>
            </div>
          </CardContent>
        </Card>
      </div>
      <Card class="mt-10">
        <CardHeader>
          <CardTitle>Ingredient Notes</CardTitle>
          <CardDescription>
            Extra details referenced by * in the ingredient list
          </CardDescription>
        </CardHeader>

        <CardContent class="space-y-4">
          <template
              v-for="group in recipe.ingredient_groups"
              :key="group.position"
          >
            <template
                v-for="ing in group.ingredients"
                :key="ing.position"
            >
              <div
                  v-if="ing.note"
                  :id="`ingredient-${ing.position}`"
                  class="relative pl-8 py-3 rounded-md bg-muted/40"
              >
                <!-- Dash marker -->
                <span
                    class="absolute left-3 top-4 text-muted-foreground font-bold"
                >
            –
          </span>

                <p class="font-medium">
                  {{ ing.name }}
                </p>

                <p class="text-sm text-muted-foreground leading-relaxed">
                  {{ ing.note }}
                </p>
              </div>
            </template>
          </template>
        </CardContent>
      </Card>

    </div>
  </div>
</template>


<style scoped>
/* Optional: make images responsive */
img {
  max-width: 100%;
}

:target {
  scroll-margin-top: 100px;
  animation: highlight 1.5s ease-out;
}

@keyframes highlight {
  0% {
    background-color: hsl(var(--primary) / 0.15);
  }
  100% {
    background-color: transparent;
  }
}

</style>
