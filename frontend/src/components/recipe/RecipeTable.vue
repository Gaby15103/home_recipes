<script setup lang="ts">
import type {
  ColumnDef,
} from '@tanstack/vue-table'
import {
  FlexRender,
  getCoreRowModel,
  getExpandedRowModel,
  getFilteredRowModel,
  getSortedRowModel,
  useVueTable,
} from '@tanstack/vue-table'
import {createReusableTemplate} from '@vueuse/core'
import {h} from 'vue'
import {ArrowUpDown, ChevronDown, MoreHorizontal} from 'lucide-vue-next'
import {Button} from '@/components/ui/button'
import {Checkbox} from '@/components/ui/checkbox'
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import {Input} from '@/components/ui/input'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import type {RecipeView, Tag} from '@/models/Recipe.ts'
import {Badge} from "@/components/ui/badge";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import {ROUTES} from "@/router/routes.ts";
import {useI18n} from "vue-i18n";
const { t } = useI18n()
const props = defineProps<{
  recipes: RecipeView[]
  page: number
  perPage: number
  total: number
  loading: boolean
}>()

const emit = defineEmits<{
  (e: "next-page"): void
  (e: "previous-page"): void
}>()


// ------------------------- Reusable dropdown template -------------------------
const [DefineTemplate, ReuseTemplate] = createReusableTemplate<{
  recipe: RecipeView
  onExpand: () => void
}>()

function copyId(id: string) {
  navigator.clipboard.writeText(id)
}

// ------------------------- Table columns -------------------------
// ------------------------- Table columns -------------------------
const columns: ColumnDef<RecipeView>[] = [
  {
    id: 'select',
    header: ({table}) => h(Checkbox, {
      modelValue: table.getIsAllPageRowsSelected() || (table.getIsSomePageRowsSelected() && 'indeterminate'),
      'onUpdate:modelValue': value => table.toggleAllPageRowsSelected(!!value),
      ariaLabel: t('Admin.table.selectAll'),
    }),
    cell: ({row}) => h(Checkbox, {
      modelValue: row.getIsSelected(),
      'onUpdate:modelValue': value => row.toggleSelected(!!value),
      ariaLabel: t('Admin.table.selectRow'),
    }),
    enableSorting: false,
    enableHiding: false,
  },

  {
    // Updated: Uses direct key from your JSON
    accessorKey: 'title',
    header: t('Admin.table.title'),
    cell: ({row}) => {
      const title = row.getValue('title') as string || '—'

      return h(
          TooltipProvider,
          {},
          () =>
              h(Tooltip, {}, {
                default: () => [
                  h(
                      TooltipTrigger,
                      {asChild: true},
                      () =>
                          h(
                              'div',
                              { class: 'max-w-[220px] truncate cursor-default font-medium' },
                              title
                          )
                  ),
                  h(
                      TooltipContent,
                      {side: 'top', class: 'max-w-sm break-words'},
                      () => title
                  ),
                ],
              })
      )
    },
  },

  {
    accessorKey: 'author',
    header: ({column}) =>
        h(Button, {
          variant: 'ghost',
          onClick: () => column.toggleSorting(column.getIsSorted() === 'asc'),
        }, () => [
          t('Admin.table.author'),
          h(ArrowUpDown, { class: 'ml-2 h-4 w-4' })
        ]),
    cell: ({row}) => row.getValue('author'),
  },

  {
    accessorKey: 'cook_time_minutes',
    header: t('Admin.table.cookTime'),
    // Added safety check for 0 or null
    cell: ({row}) => {
      const time = row.getValue('cook_time_minutes')
      return time !== undefined ? `${time} ${t('recipe.meta.minutes')}` : '—'
    },
  },

  {
    accessorKey: 'tags',
    header: t('Admin.table.tags'),
    cell: ({row}) => {
      const tags = row.getValue('tags') as Tag[]
      const MAX_VISIBLE = 3

      if (!tags?.length) {
        return h('span', {class: 'text-muted-foreground italic'}, '—')
      }

      const visible = tags.slice(0, MAX_VISIBLE)
      const hidden = tags.slice(MAX_VISIBLE)

      const renderBadge = (tag: Tag) =>
          h(Badge, {variant: 'secondary', key: tag.id}, () => tag.name)

      return h(
          TooltipProvider,
          {},
          () =>
              h(Tooltip, {}, {
                default: () => [
                  h(
                      TooltipTrigger,
                      {asChild: true},
                      () =>
                          h('div', {class: 'flex flex-wrap gap-1 max-w-[260px]'}, [
                            ...visible.map(renderBadge),

                            hidden.length
                                ? h(Badge, {variant: 'outline'}, () => `+${hidden.length}`)
                                : null,
                          ])
                  ),

                  hidden.length
                      ? h(TooltipContent, {class: 'flex flex-wrap gap-1 max-w-sm'},
                          () => tags.map(renderBadge))
                      : null,
                ],
              })
      )
    },
  },

  {
    id: 'actions',
    enableHiding: false,
    cell: ({row}) => h(ReuseTemplate, {
      recipe: row.original,
      onExpand: row.toggleExpanded,
    }),
  },
]


const table = useVueTable({
  // Use a getter with a fallback array
  get data() {
    return props.recipes ?? []
  },
  columns,
  manualPagination: true,

  // CRITICAL: Ensure this never returns NaN
  get pageCount() {
    if (!props.total || !props.perPage) return 0
    return Math.ceil(props.total / props.perPage)
  },

  getCoreRowModel: getCoreRowModel(),
  getSortedRowModel: getSortedRowModel(),
  getFilteredRowModel: getFilteredRowModel(),
  getExpandedRowModel: getExpandedRowModel(),

  state: {
    get pagination() {
      return {
        // Page is 1-indexed from props, but 0-indexed in TanStack
        pageIndex: Math.max(0, props.page - 1),
        pageSize: props.perPage || 10,
      }
    },
  },
})


</script>

<template>
  <DefineTemplate v-slot="{ recipe }">
    <DropdownMenu>
      <DropdownMenuTrigger as-child>
        <Button variant="ghost" class="h-8 w-8 p-0">
          <span class="sr-only">Open menu</span>
          <MoreHorizontal class="h-4 w-4"/>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuLabel>{{ t('Admin.common.actions') }}</DropdownMenuLabel>
        <DropdownMenuItem @click="copyId(recipe.id)">{{ t('Admin.table.copyId') }}</DropdownMenuItem>
        <DropdownMenuSeparator/>
        <DropdownMenuItem>
          <RouterLink :to="ROUTES.ADMIN.RECIPE.VIEW(recipe.id)">
            {{ t('Admin.table.viewRecipe') }}
          </RouterLink>
          </DropdownMenuItem>
        <DropdownMenuItem>
          <RouterLink :to="ROUTES.ADMIN.RECIPE.EDIT(recipe.id)">
            {{ t('Admin.table.editRecipe') }}
          </RouterLink>
         </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  </DefineTemplate>

  <div class="flex items-center py-4 gap-2">
    <Input
        class="max-w-sm"
        :placeholder="t('Admin.table.filter')"
        :model-value="table.getColumn('title')?.getFilterValue() as string"
        @update:model-value="table.getColumn('title')?.setFilterValue($event)"
    />
    <DropdownMenu>
      <DropdownMenuTrigger as-child>
        <Button variant="outline" class="ml-auto">
          {{ t('Admin.table.columns') }}
          <ChevronDown class="ml-2 h-4 w-4"/>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuCheckboxItem
            v-for="column in table.getAllColumns().filter(c => c.getCanHide())"
            :key="column.id"
            class="capitalize"
            :model-value="column.getIsVisible()"
            @update:model-value="(value) => column.toggleVisibility(!!value)"
        >
          {{ column.id }}
        </DropdownMenuCheckboxItem>
      </DropdownMenuContent>
    </DropdownMenu>
  </div>

  <div class="rounded-md border">
    <Table>
      <TableHeader>
        <TableRow v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
          <TableHead v-for="header in headerGroup.headers" :key="header.id">
            <FlexRender v-if="!header.isPlaceholder" :render="header.column.columnDef.header"
                        :props="header.getContext()"/>
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <template v-if="table.getRowModel().rows.length">
          <template v-for="row in table.getRowModel().rows" :key="row.id">
            <TableRow :data-state="row.getIsSelected() && 'selected'">
              <TableCell v-for="cell in row.getVisibleCells()" :key="cell.id">
                <FlexRender :render="cell.column.columnDef.cell" :props="cell.getContext()"/>
              </TableCell>
            </TableRow>
            <TableRow v-if="row.getIsExpanded()">
              <TableCell :colspan="row.getAllCells().length">
                {{ JSON.stringify(row.original) }}
              </TableCell>
            </TableRow>
          </template>
        </template>
        <TableRow v-else>
          <TableCell :colspan="columns.length" class="h-24 text-center">
            {{ t('Admin.table.noRecipes') }}
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>

  <div class="flex items-center justify-end space-x-2 py-4">
    <div class="flex-1 text-sm text-muted-foreground">
      {{ table.getFilteredSelectedRowModel().rows.length }} {{ t('Admin.table.of') }}
      {{ table.getFilteredRowModel().rows.length }} {{ t('Admin.table.recipe_selected') }}
    </div>
    <div class="space-x-2">
      <Button
          variant="outline"
          size="sm"
          :disabled="props.page === 1 || props.loading"
          @click="emit('previous-page')"
      >
        {{ t('Admin.table.previous') }}
      </Button>

      <Button
          variant="outline"
          size="sm"
          :disabled="props.page * props.perPage >= props.total || props.loading"
          @click="emit('next-page')"
      >
        {{ t('Admin.table.next') }}
      </Button>
    </div>
  </div>
</template>
<style scoped>
.badge {
  font-size: 0.7rem;
  padding: 2px 6px;
}
</style>