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
import type {Recipe, Tag} from '@/models/Recipe'
import {Badge} from "@/components/ui/badge";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'

const props = defineProps<{
  recipes: Recipe[]
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
  recipe: Recipe
  onExpand: () => void
}>()

function copyId(id: string) {
  navigator.clipboard.writeText(id)
}

// ------------------------- Table columns -------------------------
const columns: ColumnDef<Recipe>[] = [
  {
    id: 'select',
    header: ({table}) => h(Checkbox, {
      'modelValue': table.getIsAllPageRowsSelected() || (table.getIsSomePageRowsSelected() && 'indeterminate'),
      'onUpdate:modelValue': value => table.toggleAllPageRowsSelected(!!value),
      'ariaLabel': 'Select all',
    }),
    cell: ({row}) => h(Checkbox, {
      'modelValue': row.getIsSelected(),
      'onUpdate:modelValue': value => row.toggleSelected(!!value),
      'ariaLabel': 'Select row',
    }),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: 'title',
    header: 'Title',
    cell: ({row}) => {
      const title = row.getValue('title') as string

      return h(
          TooltipProvider,
          {},
          () =>
              h(
                  Tooltip,
                  {},
                  {
                    default: () => [
                      h(
                          TooltipTrigger,
                          {asChild: true},
                          () =>
                              h(
                                  'div',
                                  {
                                    class:
                                        'max-w-[220px] truncate cursor-default font-medium',
                                  },
                                  title,
                              ),
                      ),
                      h(
                          TooltipContent,
                          {side: 'top', class: 'max-w-sm break-words'},
                          () => title,
                      ),
                    ],
                  },
              ),
      )
    },
  },
  {
    accessorKey: 'author',
    header: ({column}) => {
      return h(Button, {
        variant: 'ghost',
        onClick: () => column.toggleSorting(column.getIsSorted() === 'asc'),
      }, () => ['Author', h(ArrowUpDown, {class: 'ml-2 h-4 w-4'})])
    },
    cell: ({row}) => row.getValue('author'),
  },
  {
    accessorKey: 'cook_time_minutes',
    header: 'Cook Time',
    cell: ({row}) => row.getValue('cook_time_minutes') + ' min',
  },
  {
    accessorKey: 'tags',
    header: 'Tags',
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
              h(
                  Tooltip,
                  {},
                  {
                    default: () => [
                      h(
                          TooltipTrigger,
                          {asChild: true},
                          () =>
                              h(
                                  'div',
                                  {class: 'flex flex-wrap gap-1 max-w-[260px]'},
                                  [
                                    ...visible.map(renderBadge),

                                    hidden.length
                                        ? h(
                                            Badge,
                                            {variant: 'outline'},
                                            () => `+${hidden.length}`,
                                        )
                                        : null,
                                  ],
                              ),
                      ),

                      hidden.length
                          ? h(
                              TooltipContent,
                              {class: 'flex flex-wrap gap-1 max-w-sm'},
                              () => tags.map(renderBadge),
                          )
                          : null,
                    ],
                  },
              ),
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
  get data() {
    return props.recipes
  },
  columns,
  manualPagination: true,
  pageCount: Math.ceil(props.total / props.perPage),

  getCoreRowModel: getCoreRowModel(),
  getSortedRowModel: getSortedRowModel(),
  getFilteredRowModel: getFilteredRowModel(),
  getExpandedRowModel: getExpandedRowModel(),

  state: {
    get pagination() {
      return {
        pageIndex: props.page - 1,
        pageSize: props.perPage,
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
        <DropdownMenuLabel>Actions</DropdownMenuLabel>
        <DropdownMenuItem @click="copyId(recipe.id)">Copy recipe ID</DropdownMenuItem>
        <DropdownMenuSeparator/>
        <DropdownMenuItem>View recipe</DropdownMenuItem>
        <DropdownMenuItem>Edit recipe</DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  </DefineTemplate>

  <div class="flex items-center py-4 gap-2">
    <Input
        class="max-w-sm"
        placeholder="Filter recipes..."
        :model-value="table.getColumn('title')?.getFilterValue() as string"
        @update:model-value="table.getColumn('title')?.setFilterValue($event)"
    />
    <DropdownMenu>
      <DropdownMenuTrigger as-child>
        <Button variant="outline" class="ml-auto">
          Columns
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
            No recipes found.
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>

  <div class="flex items-center justify-end space-x-2 py-4">
    <div class="flex-1 text-sm text-muted-foreground">
      {{ table.getFilteredSelectedRowModel().rows.length }} of
      {{ table.getFilteredRowModel().rows.length }} recipe(s) selected.
    </div>
    <div class="space-x-2">
      <Button
          variant="outline"
          size="sm"
          :disabled="props.page === 1 || props.loading"
          @click="emit('previous-page')"
      >
        Previous
      </Button>

      <Button
          variant="outline"
          size="sm"
          :disabled="props.page * props.perPage >= props.total || props.loading"
          @click="emit('next-page')"
      >
        Next
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