<script setup lang="ts">
import { MoreVertical, Edit3, Eye, Trash2 } from 'lucide-vue-next'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSeparator
} from '@/components/ui/dropdown-menu'
import NavLink from "@/components/navigation/NavLink.vue";
import {ROUTES} from "@/router/routes.ts";

defineProps<{ id: string }>()
const emit = defineEmits(['edit', 'view', 'delete', 'archive'])
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <button class="p-2 rounded-lg hover:bg-neutral-800 text-neutral-600 hover:text-white transition-all">
        <MoreVertical class="h-4 w-4" />
      </button>
    </DropdownMenuTrigger>

    <DropdownMenuContent align="end" class="w-52 bg-[#0f0f0f] border-neutral-800 text-neutral-400 rounded-2xl shadow-2xl p-2">
      <NavLink :to="ROUTES.RECIPE(id)" target="_blank">
        <DropdownMenuItem @click="emit('view', id)" class="rounded-xl px-3 py-2.5 gap-3 focus:bg-primary/10 focus:text-primary cursor-pointer">
          <Eye class="h-4 w-4" />
          <span class="text-xs font-bold uppercase tracking-tight">View Publicly</span>
        </DropdownMenuItem>
      </NavLink>

      <NavLink :to="ROUTES.STUDIO.VIEW(id)">
        <DropdownMenuItem @click="emit('view', id)" class="rounded-xl px-3 py-2.5 gap-3 focus:bg-primary/10 focus:text-primary cursor-pointer">
          <Eye class="h-4 w-4" />
          <span class="text-xs font-bold uppercase tracking-tight">View Recipe</span>
        </DropdownMenuItem>
      </NavLink>

      <NavLink :to="ROUTES.STUDIO.EDIT(id)">
        <DropdownMenuItem @click="emit('edit', id)" class="rounded-xl px-3 py-2.5 gap-3 focus:bg-primary/10 focus:text-primary cursor-pointer">
          <Edit3 class="h-4 w-4" />
          <span class="text-xs font-bold uppercase tracking-tight">Edit Recipe</span>
        </DropdownMenuItem>
      </NavLink>

      <DropdownMenuSeparator class="bg-neutral-800 my-1 mx-2" />

      <DropdownMenuItem @click="emit('delete', id)" class="rounded-xl px-3 py-2.5 gap-3 text-red-500 focus:bg-red-500/10 focus:text-red-500 cursor-pointer">
        <Trash2 class="h-4 w-4" />
        <span class="text-xs font-bold uppercase tracking-tight">Delete Permanent</span>
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>