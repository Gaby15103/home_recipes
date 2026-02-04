<script setup lang="ts">
import {ref, computed, onMounted, watch} from 'vue'
import { Check, ChevronsUpDown, X, Plus } from 'lucide-vue-next'
import { getTags } from '@/api/tag'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from '@/components/ui/command'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { cn } from '@/lib/utils'
import type {InputTag, Tag} from "@/models/Tag.ts";
import {useI18n} from "vue-i18n";
const { t } = useI18n()

const props = defineProps<{ modelValue: InputTag[] }>()
const emit = defineEmits(['update:modelValue'])

const open = ref(false)
const allTags = ref<Tag[]>([])
const searchQuery = ref('')

onMounted(async () => {
  allTags.value = await getTags()
  syncImportedTags()
})

watch(() => props.modelValue, () => {
  if (allTags.value.length > 0) syncImportedTags()
}, { deep: true })

function syncImportedTags() {
  let hasChanges = false
  const synced = props.modelValue.map(item => {
    if (item.type === 'new') {
      const match = allTags.value.find(t => t.name.toLowerCase() === item.name.toLowerCase())
      if (match) {
        hasChanges = true
        return { type: 'existing' as const, id: match.id }
      }
    }
    return item
  })

  if (hasChanges) {
    emit('update:modelValue', synced)
  }
}

const getTagName = (item: InputTag) => {
  if (item.type === 'new') return item.name
  return allTags.value.find((t: { id: any }) => t.id === item.id)?.name || t('Admin.tags.loading')
}

// Logic to check if the current search term is unique
const canCreate = computed(() => {
  const term = searchQuery.value.trim()
  if (!term) return false

  const existsInDb = allTags.value.some(t => t.name.toLowerCase() === term.toLowerCase())
  const existsInSelected = props.modelValue.some(t =>
      t.type === 'new' && t.name.toLowerCase() === term.toLowerCase()
  )
  return !existsInDb && !existsInSelected
})

function addExisting(tag: Tag) {
  const exists = props.modelValue.some(t => t.type === 'existing' && t.id === tag.id)
  if (exists) {
    emit('update:modelValue', props.modelValue.filter(t => t.id !== tag.id))
  } else {
    emit('update:modelValue', [...props.modelValue, { type: 'existing', id: tag.id }])
  }
  searchQuery.value = ''
}

function addNew() {
  const name = searchQuery.value.trim()
  if (!name) return

  emit('update:modelValue', [...props.modelValue, { type: 'new', name }])
  searchQuery.value = ''
  open.value = false // Optional: close on create
}

function removeTag(index: number) {
  const next = [...props.modelValue]
  next.splice(index, 1)
  emit('update:modelValue', next)
}
</script>

<template>
  <div class="flex flex-col gap-3">
    <div class="flex flex-wrap gap-2">
      <Badge v-for="(tag, idx) in modelValue" :key="idx" variant="secondary">
        {{ getTagName(tag) }}
        <X class="ml-1 h-3 w-3 cursor-pointer" @click="removeTag(idx)" />
      </Badge>
    </div>

    <Popover v-model:open="open">
      <PopoverTrigger as-child>
        <Button variant="outline" class="w-full justify-between">
          {{
            modelValue.length > 0
                ? t('Admin.tags.selected', { count: modelValue.length })
                : t('Admin.tags.select')
          }}
          <ChevronsUpDown class="ml-2 h-4 w-4 opacity-50" />
        </Button>
      </PopoverTrigger>

      <PopoverContent class="w-[300px] p-0" align="start">
        <Command
        :filter-function="(list, search) => {
        // We look for items that contain the search string
        // OR we manually keep the one that matches our searchQuery state
        return list.filter(i => i.toLowerCase().includes(search.toLowerCase()) || i === searchQuery)
        }">
          <CommandInput
              v-model="searchQuery"
              :placeholder="t('Admin.tags.search_or_create')"
          />

          <CommandList>
            <CommandEmpty v-if="!canCreate">
              {{ t('Admin.tags.none_found') }}
            </CommandEmpty>

            <CommandGroup v-if="allTags.length > 0" :title="t('Admin.tags.suggestions')">
              <CommandItem
                  v-for="tag in allTags"
                  :key="tag.id"
                  :value="tag.name"
                  @select="() => addExisting(tag)"
              >
                <Check :class="cn('mr-2 h-4 w-4', modelValue.some(t => t.id === tag.id) ? 'opacity-100' : 'opacity-0')" />
                {{ tag.name }}
              </CommandItem>
            </CommandGroup>

            <CommandGroup v-if="canCreate" :title="t('Admin.tags.new_tag')">
              <CommandItem
                  :key="searchQuery"
                  :value="searchQuery"
                  @select="addNew"
                  class="cursor-pointer text-blue-600 font-medium"
              >
                <Plus class="mr-2 h-4 w-4" />
                {{ t('Admin.tags.create', { name: searchQuery }) }}
              </CommandItem>
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  </div>
</template>