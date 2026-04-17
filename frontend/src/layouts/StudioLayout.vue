<script setup lang="ts">
import {SidebarInset, SidebarProvider, SidebarTrigger} from '@/components/ui/sidebar';
import StudioSidebar from '@/components/navigation/Sidebar/StudioSidebar.vue';
import {useRoute} from 'vue-router';
import {computed} from 'vue';
import {useAuthStore} from "@/stores/auth.ts";
import {storeToRefs} from "pinia";
import {useI18n} from "vue-i18n";
import NavUserComponents from "@/components/navigation/Navbar/NavUserComponents.vue";

const authStore = useAuthStore()
const {user} = storeToRefs(authStore)
const {t} = useI18n()

const route = useRoute();
const pageTitle = computed(() => route.name?.toString().replace('Studio', '') || 'Dashboard');
</script>

<template>
  <SidebarProvider default-open>
    <div class="flex h-screen w-full bg-[#050505] text-neutral-300 overflow-hidden">

      <StudioSidebar />

      <SidebarInset class="flex flex-col flex-1 bg-transparent border-none min-w-0">

        <header class="h-16 border-b border-neutral-800 bg-[#0a0a0a]/50 backdrop-blur-md sticky top-0 z-10 px-8 flex items-center justify-between">
          <div class="flex items-center gap-4">
            <SidebarTrigger class="text-neutral-500 hover:text-primary transition-colors" />
            <div class="text-xs font-black uppercase tracking-tighter text-neutral-500 flex items-center gap-2">
              Studio <span class="text-neutral-800">/</span> <span class="text-neutral-200">{{ pageTitle }}</span>
            </div>
          </div>

          <div class="flex items-center gap-4">
            <NavUserComponents/>
          </div>
        </header>

        <main class="flex-1 overflow-y-auto p-8 custom-scrollbar">
          <div class="mx-auto">
            <slot />
          </div>
        </main>
      </SidebarInset>
    </div>
  </SidebarProvider>
</template>