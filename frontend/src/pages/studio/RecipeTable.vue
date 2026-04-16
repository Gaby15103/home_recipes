<script setup lang="ts">

</script>

<template>
  <div class="bg-[#0a0a0a] border border-neutral-800 rounded-3xl overflow-hidden">
    <table class="w-full text-left">
      <thead class="bg-neutral-900/80 border-b border-neutral-800">
      <tr class="text-[10px] uppercase font-black text-neutral-500 tracking-widest">
        <th class="px-6 py-4">Recipe</th>
        <th class="px-6 py-4">Visibility</th>
        <th class="px-6 py-4">Engagement</th>
        <th class="px-6 py-4">Status</th>
        <th class="px-6 py-4 text-right">Actions</th>
      </tr>
      </thead>
      <tbody class="divide-y divide-neutral-900">
      <tr v-for="i in 5" :key="i" class="group hover:bg-primary/2 transition-colors">
        <td class="px-6 py-4">
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded bg-neutral-800 border border-neutral-700" />
            <span class="text-sm font-bold text-neutral-200">Spicy Ramen Noodles</span>
          </div>
        </td>
        <td class="px-6 py-4">
          <div class="flex items-center gap-1.5">
            <div class="w-1.5 h-1.5 rounded-full bg-green-500" />
            <span class="text-[11px] font-medium text-neutral-400">Public</span>
          </div>
        </td>
        <td class="px-6 py-4 text-xs font-medium text-neutral-500 italic">4.8 (210 reviews)</td>
        <td class="px-6 py-4">
          <span class="text-[10px] font-black uppercase px-2 py-0.5 border border-primary/30 text-primary rounded">Verified Chef</span>
        </td>
        <td class="px-6 py-4 text-right">
          <button class="text-neutral-600 hover:text-white transition-colors">
            <MoreVertical class="h-4 w-4" />
          </button>
        </td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>

</style>