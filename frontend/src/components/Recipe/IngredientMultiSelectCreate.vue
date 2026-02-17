<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { Check, ChevronsUpDown, X, Plus } from 'lucide-vue-next'
import { getIngredients } from '@/api/ingredient' // Your API
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from '@/components/ui/command'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { cn } from '@/lib/utils'
import { useI18n } from "vue-i18n"

const { t } = useI18n()

// Assuming InputIngredient matches your Tag structure: { type: 'new', name: string } | { type: 'existing', id: string }
interface Ingredient { id: number; name: string }
interface SelectedIngredient { type: 'existing' | 'new'; id?: number; name?: string }

const props = defineProps<{ modelValue: SelectedIngredient[] }>()
const emit = defineEmits(['update:modelValue'])

const open = ref(false)
const allIngredients = ref<Ingredient[]>([])
const searchQuery = ref('')

onMounted(async () => {
  allIngredients.value = await getIngredients()
})

const getDisplayName = (item: SelectedIngredient) => {
  if (item.type === 'new') return item.name
  return allIngredients.value.find(i => i.id === item.id)?.name || '...'
}

const canCreate = computed(() => {
  const term = searchQuery.value.trim().toLowerCase()
  if (!term) return false
  const existsInDb = allIngredients.value.some(i => i.name.toLowerCase() === term)
  const existsInSelected = props.modelValue.some(i => i.name?.toLowerCase() === term)
  return !existsInDb && !existsInSelected
})

function toggleIngredient(ing: Ingredient) {
  const isSelected = props.modelValue.some(i => i.type === 'existing' && i.id === ing.id)
  if (isSelected) {
    emit('update:modelValue', props.modelValue.filter(i => i.id !== ing.id))
  } else {
    emit('update:modelValue', [...props.modelValue, { type: 'existing', id: ing.id }])
  }
}

function addNew() {
  const name = searchQuery.value.trim()
  emit('update:modelValue', [...props.modelValue, { type: 'new', name }])
  searchQuery.value = ''
}

function remove(idx: number) {
  const next = [...props.modelValue]
  next.splice(idx, 1)
  emit('update:modelValue', next)
}
</script>

<template>
  <div class="flex flex-col gap-3">
    <div class="flex flex-wrap gap-2">
      <Badge v-for="(ing, idx) in modelValue" :key="idx" variant="secondary" class="py-1">
        {{ getDisplayName(ing) }}
        <X class="ml-1 h-3 w-3 cursor-pointer hover:text-destructive" @click="remove(idx)" />
      </Badge>
    </div>

    <Popover v-model:open="open">
      <PopoverTrigger as-child>
        <Button variant="outline" class="w-full justify-between shadow-sm">
          <span class="truncate">
            {{ modelValue.length > 0 ? t('Ingredients.selected', { count: modelValue.length }) : t('Ingredients.select') }}
          </span>
          <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent class="w-[300px] p-0" align="start">
        <Command>
          <CommandInput v-model="searchQuery" :placeholder="t('Ingredients.search_placeholder')" />
          <CommandList>
            <CommandEmpty v-if="!canCreate">{{ t('Ingredients.not_found') }}</CommandEmpty>

            <CommandGroup :title="t('Ingredients.suggestions')">
              <CommandItem
                  v-for="ing in allIngredients"
                  :key="ing.id"
                  :value="ing.name"
                  @select="toggleIngredient(ing)"
              >
                <Check :class="cn('mr-2 h-4 w-4', modelValue.some(i => i.id === ing.id) ? 'opacity-100' : 'opacity-0')" />
                {{ ing.name }}
              </CommandItem>
            </CommandGroup>

            <CommandGroup v-if="canCreate" :title="t('Ingredients.create_new')">
              <CommandItem :value="searchQuery" @select="addNew" class="text-blue-600 font-medium">
                <Plus class="mr-2 h-4 w-4" />
                {{ t('Ingredients.add', { name: searchQuery }) }}
              </CommandItem>
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  </div>
</template>