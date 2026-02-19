<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { debounce } from 'lodash-es'
import {
  ComboboxAnchor, ComboboxContent, ComboboxEmpty, ComboboxGroup,
  ComboboxInput, ComboboxItem, ComboboxItemIndicator, ComboboxLabel,
  ComboboxRoot, ComboboxTrigger, ComboboxViewport,
  TagsInputInput, TagsInputItem, TagsInputItemDelete, TagsInputItemText, TagsInputRoot
} from 'radix-vue'
import { Icon } from '@iconify/vue'
import type { IngredientView } from "@/models/Recipe.ts"
import { getIngredients } from "@/api/ingredient.ts"
import { useI18n } from "vue-i18n"

const { t } = useI18n()

const props = withDefaults(defineProps<{ modelValue: IngredientView[] }>(), {
  modelValue: () => []
})
const emit = defineEmits(['update:modelValue'])

const searchTerm = ref('')
const allIngredients = ref<IngredientView[]>([])
const loading = ref(false)

// We use a local ref to bridge Radix and the Parent's v-model
const selectedIngredients = ref<IngredientView[]>(props.modelValue)

// Keep local state in sync with external prop
watch(() => props.modelValue, (newVal) => {
  selectedIngredients.value = newVal
}, { deep: true })

// Emit changes back up
watch(selectedIngredients, (newVal) => {
  emit('update:modelValue', newVal)
  searchTerm.value = '' // Clear search when an item is selected
}, { deep: true })

async function fetchIngredients(query = '') {
  loading.value = true
  try {
    allIngredients.value = await getIngredients(query, 25)
  } finally {
    loading.value = false
  }
}

const debouncedSearch = debounce((val: string) => fetchIngredients(val), 300)
watch(searchTerm, (newVal) => debouncedSearch(newVal))
onMounted(() => fetchIngredients())
</script>

<template>
  <ComboboxRoot
      v-model="selectedIngredients"
      v-model:search-term="searchTerm"
      multiple
      by="id"
      class="relative w-full"
  >
    <ComboboxAnchor class="w-full inline-flex items-center justify-between rounded-lg border border-input p-2 min-h-10 gap-2 bg-background shadow-sm hover:bg-accent/50 focus-within:ring-2 focus-within:ring-ring outline-none">

      <TagsInputRoot
          v-slot="{ modelValue: tags }"
          v-model="selectedIngredients"
          delimiter=""
          class="flex gap-2 items-center rounded-lg flex-wrap flex-1"
      >
        <TagsInputItem
            v-for="item in (tags as IngredientView[])"
            :key="item.id"
            :value="item"
            class="flex items-center justify-center gap-2 text-secondary-foreground bg-secondary rounded px-2 py-1 data-[current=true]:ring-2 ring-ring"
        >
          <TagsInputItemText class="text-sm font-medium" >{{item.name}}</TagsInputItemText>
          <TagsInputItemDelete>
            <Icon icon="lucide:x" class="h-3 w-3" />
          </TagsInputItemDelete>
        </TagsInputItem>

        <ComboboxInput as-child>
          <TagsInputInput
              :placeholder="t('Ingredients.search_placeholder')"
              class="focus:outline-none flex-1 rounded bg-transparent placeholder:text-muted-foreground px-1 min-w-[120px]"
              @keydown.enter.prevent
          />
        </ComboboxInput>
      </TagsInputRoot>

      <ComboboxTrigger>
        <Icon
            icon="radix-icons:chevron-down"
            class="h-4 w-4 opacity-50"
        />
      </ComboboxTrigger>
    </ComboboxAnchor>

    <ComboboxContent
        class="absolute z-50 w-[var(--radix-combobox-trigger-width)] mt-2 bg-popover text-popover-foreground overflow-hidden rounded-md border shadow-md animate-in fade-in zoom-in-95"
    >


      <ComboboxViewport class="p-1">
        <div v-if="loading" class="text-xs text-center py-4 text-muted-foreground italic">
          {{ t('Loading') }}
        </div>

        <ComboboxEmpty class="text-muted-foreground text-xs font-medium text-center py-4">
          {{ t('Ingredients.no_match') }}
        </ComboboxEmpty>

        <ComboboxGroup>
          <ComboboxLabel class="px-2 py-1.5 text-xs font-semibold text-muted-foreground uppercase tracking-wider">
            {{ t('Ingredients.suggestions') }}
          </ComboboxLabel>

          <ComboboxItem
              v-for="ing in allIngredients"
              :key="ing.id"
              class="text-sm leading-none rounded-sm flex items-center h-9 pr-2 pl-8 relative select-none data-[disabled]:text-muted-foreground data-[disabled]:pointer-events-none data-[highlighted]:outline-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground cursor-pointer"
              :value="ing"
          >
            <ComboboxItemIndicator
                class="absolute left-2 w-5 inline-flex items-center justify-center"
            >
              <Icon icon="radix-icons:check" class="h-4 w-4" />
            </ComboboxItemIndicator>
            <span>{{ ing.name }}</span>
          </ComboboxItem>
        </ComboboxGroup>
      </ComboboxViewport>
    </ComboboxContent>
  </ComboboxRoot>
</template>