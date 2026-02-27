<script setup lang="ts">
import { computed, ref } from "vue"
import { useI18n } from "vue-i18n"
import {
  Search,
  SlidersHorizontal,
  RotateCcw,
  ChevronDown,
  Clock,
  ChefHat,
  Calendar
} from "lucide-vue-next"

import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Badge } from "@/components/ui/badge"
import TagsMultiSelect from "@/components/recipe/forms/TagsMultiSelect.vue"
import IngredientMultiSelect from "@/components/recipe/forms/IngredientMultiSelect.vue"
import type { RecipeFilter } from "@/models/Recipe.ts"


const { t } = useI18n()
const isOpen = ref(false)

const props = defineProps<{
  modelValue: RecipeFilter
}>()

const emit = defineEmits<{
  (e: "update:modelValue", value: RecipeFilter): void
  (e: "search"): void
}>()

const filters = computed({
  get: () => props.modelValue,
  set: value => emit("update:modelValue", value),
})

function handleSearch() {
  emit("search")
}

// Helper to count active advanced filters (excluding search and ingredients)
const activeAdvancedCount = computed(() => {
  const f = filters.value;
  let count = 0;
  if (f.tags?.length) count++;
  if (f.minPrep || f.maxPrep) count++;
  if (f.minCook || f.maxCook) count++;
  if (f.minSteps || f.maxSteps) count++;
  if (f.dateFrom || f.dateTo) count++;
  return count;
})

function reset() {
  emit("update:modelValue", {
    search: "",
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
  })
  handleSearch()
}
</script>

<template>
  <div class="space-y-4">
    <div class="flex flex-col lg:flex-row gap-3">
      <div class="relative flex-1">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
        <Input
            v-model="filters.search"
            :placeholder="t('Home.SearchPlaceholder')"
            class="pl-10 h-11 bg-background shadow-sm focus-visible:ring-primary w-full"
            @keyup.enter="handleSearch"
        />
      </div>

      <div class="flex flex-wrap md:flex-nowrap gap-2 items-center">
        <IngredientMultiSelect
            v-model="filters.ingredient"
            class="flex-1 min-w-[200px] md:w-64"
        />

        <div class="flex gap-2 w-full md:w-auto">
          <Button
              @click="handleSearch"
              class="h-11 flex-1 md:flex-none md:px-6 font-bold shadow-md bg-primary hover:bg-primary/90 text-primary-foreground"
          >
            <Search class="mr-2 h-4 w-4" />
            <span class="hidden sm:inline">{{ t('Common.Search') }}</span>
            <span class="sm:hidden">{{ t('Common.Search') }}</span>
          </Button>

          <Button
              variant="outline"
              @click="reset"
              class="h-11 px-3 text-muted-foreground border-input hover:bg-muted"
              :title="t('Common.Reset')"
          >
            <RotateCcw class="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>

    <Collapsible v-model:open="isOpen" class="w-full border rounded-xl bg-muted/30 overflow-hidden transition-all">
      <CollapsibleTrigger as-child>
        <Button
            variant="ghost"
            class="w-full flex justify-between items-center py-6 px-4 hover:bg-muted/50 transition-colors"
        >
          <div class="flex items-center gap-2 text-foreground/80">
            <SlidersHorizontal class="h-4 w-4 text-primary" />
            <span class="font-semibold text-sm">{{ t('Admin.filters.advanced') }}</span>
            <Badge v-if="activeAdvancedCount > 0" variant="secondary" class="ml-2 bg-primary/10 text-primary border-none">
              {{ activeAdvancedCount }}
            </Badge>
          </div>
          <ChevronDown :class="['h-4 w-4 transition-transform duration-200', isOpen ? 'rotate-180' : '']" />
        </Button>
      </CollapsibleTrigger>

      <CollapsibleContent class="px-4 pb-6 pt-2">
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">

          <div class="space-y-4 bg-background/50 p-3 rounded-lg border border-transparent hover:border-primary/10 transition-colors">
            <div class="flex items-center gap-2 text-primary font-bold text-xs uppercase tracking-wider">
              <Clock class="h-4 w-4" />
              {{ t('recipe.meta.time') }}
            </div>
            <div class="space-y-3">
              <div class="grid grid-cols-2 gap-3">
                <div class="space-y-1.5">
                  <Label class="text-[10px] font-bold uppercase text-muted-foreground/70 ml-1">{{ t('Admin.filters.prep_min') }}</Label>
                  <Input type="number" v-model.number="filters.minPrep" placeholder="Min" class="bg-background h-9 text-sm" />
                </div>
                <div class="space-y-1.5">
                  <Label class="text-[10px] font-bold uppercase text-muted-foreground/70 ml-1">{{ t('Admin.filters.prep_max') }}</Label>
                  <Input type="number" v-model.number="filters.maxPrep" placeholder="Max" class="bg-background h-9 text-sm" />
                </div>
              </div>
              <div class="grid grid-cols-2 gap-3">
                <div class="space-y-1.5">
                  <Label class="text-[10px] font-bold uppercase text-muted-foreground/70 ml-1">{{ t('Admin.filters.cook_min') }}</Label>
                  <Input type="number" v-model.number="filters.minCook" placeholder="Min" class="bg-background h-9 text-sm" />
                </div>
                <div class="space-y-1.5">
                  <Label class="text-[10px] font-bold uppercase text-muted-foreground/70 ml-1">{{ t('Admin.filters.cook_max') }}</Label>
                  <Input type="number" v-model.number="filters.maxCook" placeholder="Max" class="bg-background h-9 text-sm" />
                </div>
              </div>
            </div>
          </div>

          <div class="space-y-4 bg-background/50 p-3 rounded-lg border border-transparent hover:border-primary/10 transition-colors">
            <div class="flex items-center gap-2 text-primary font-bold text-xs uppercase tracking-wider">
              <ChefHat class="h-4 w-4" />
              {{ t('recipe.meta.complexity') }}
            </div>
            <div class="grid grid-cols-2 gap-3">
              <div class="space-y-1.5">
                <Label class="text-[10px] font-bold uppercase text-muted-foreground/70 ml-1">{{ t('Admin.filters.steps_min') }}</Label>
                <Input type="number" v-model.number="filters.minSteps" placeholder="Min" class="bg-background h-9 text-sm" />
              </div>
              <div class="space-y-1.5">
                <Label class="text-[10px] font-bold uppercase text-muted-foreground/70 ml-1">{{ t('Admin.filters.steps_max') }}</Label>
                <Input type="number" v-model.number="filters.maxSteps" placeholder="Max" class="bg-background h-9 text-sm" />
              </div>
            </div>
            <div class="space-y-1.5">
              <Label class="text-[10px] font-bold uppercase text-muted-foreground/70 ml-1">{{ t('recipe.meta.tags') }}</Label>
              <TagsMultiSelect v-model="filters.tags" />
            </div>
          </div>

          <div class="space-y-4 bg-background/50 p-3 rounded-lg border border-transparent hover:border-primary/10 transition-colors">
            <div class="flex items-center gap-2 text-primary font-bold text-xs uppercase tracking-wider">
              <Calendar class="h-4 w-4" />
              {{ t('recipe.meta.dates') }}
            </div>
            <div class="space-y-3">
              <div class="space-y-1.5">
                <Label class="text-[10px] font-bold uppercase text-muted-foreground/70 ml-1">{{ t('Admin.filters.date_from') || 'Depuis' }}</Label>
                <Input type="date" v-model="filters.dateFrom" class="bg-background h-9 text-sm w-full" />
              </div>
              <div class="space-y-1.5">
                <Label class="text-[10px] font-bold uppercase text-muted-foreground/70 ml-1">{{ t('Admin.filters.date_to') || 'Jusqu\'à' }}</Label>
                <Input type="date" v-model="filters.dateTo" class="bg-background h-9 text-sm w-full" />
              </div>
            </div>
          </div>

        </div>
      </CollapsibleContent>
    </Collapsible>
  </div>
</template>