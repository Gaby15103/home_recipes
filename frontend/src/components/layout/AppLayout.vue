<script setup>
import Navbar from "./Navbar.vue";
import {SidebarTrigger } from '@/components/ui/sidebar'
import AdminSidebar from "@/components/layout/AdminSidebar.vue";
import {SidebarProvider} from "@/components/ui/sidebar/index.ts";

import { useSidebarStore } from '@/stores/sidebar'
import { storeToRefs } from 'pinia'

const sidebarStore = useSidebarStore()
const { isOpen } = storeToRefs(sidebarStore)
</script>

<template>
  <div class="min-h-screen bg-background text-foreground">
    <Navbar />
    <SidebarProvider :open="isOpen">
      <AdminSidebar/>
      <main class="container mx-auto p-6">
        <slot />
      </main>
    </SidebarProvider>
  </div>
</template>