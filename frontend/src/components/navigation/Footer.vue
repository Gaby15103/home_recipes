<script setup lang="ts">
import { ROUTES } from "@/router/routes.ts";
import { useI18n } from "vue-i18n";
import { Github, Mail, Globe } from "lucide-vue-next";
import ApiStatus from "@/components/navigation/ApiStatus.vue"; // Optional icons

const { t } = useI18n();
const currentYear = new Date().getFullYear();
</script>

<template>
  <footer class="w-full border-t bg-card mt-auto">
    <div class="mx-auto max-w-7xl px-6 py-10 lg:px-8">
      <div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-4">

        <div class="col-span-1 sm:col-span-2 lg:col-span-1">
          <div class="flex items-center gap-2 mb-4">
            <span class="text-xl font-bold tracking-tight text-primary">
              Home<span class="text-muted-foreground">Recipes</span>
            </span>
          </div>
          <p class="text-sm text-muted-foreground max-w-xs">
            {{ t('footer.description') }}
          </p>
          <div class="flex gap-4 mt-6">
            <a href="#" class="text-muted-foreground hover:text-primary transition-colors">
              <Github class="h-5 w-5" />
            </a>
            <a href="#" class="text-muted-foreground hover:text-primary transition-colors">
              <Mail class="h-5 w-5" />
            </a>
          </div>
        </div>

        <div>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-foreground mb-4">
            {{ t('footer.navigation') }}
          </h3>
          <ul class="space-y-2">
            <li><RouterLink :to="ROUTES.HOME" class="text-sm text-muted-foreground hover:text-primary transition-colors">{{ t('footer.home') }}</RouterLink></li>
            <li><RouterLink :to="ROUTES.HOME" class="text-sm text-muted-foreground hover:text-primary transition-colors">{{ t('footer.allRecipes') }}</RouterLink></li>
          </ul>
        </div>

        <div>
          <h3 class="text-xs font-semibold uppercase tracking-wider text-foreground mb-4">
            {{ t('footer.legal') }}
          </h3>
          <ul class="space-y-2">
            <li><RouterLink to="/privacy" class="text-sm text-muted-foreground hover:text-primary transition-colors">{{ t('footer.privacy') }}</RouterLink></li>
            <li><RouterLink to="/terms" class="text-sm text-muted-foreground hover:text-primary transition-colors">{{ t('footer.terms') }}</RouterLink></li>
            <li><RouterLink to="/contact" class="text-sm text-muted-foreground hover:text-primary transition-colors">{{ t('footer.contact') }}</RouterLink></li>
          </ul>
        </div>

        <ApiStatus/>
      </div>

      <div class="mt-10 border-t pt-8 flex flex-col md:flex-row justify-between items-center gap-4">
        <p class="text-xs text-muted-foreground">
          &copy; {{ currentYear }} HomeRecipes. {{ t('footer.rights') }}
        </p>
        <div class="flex items-center gap-4 text-xs text-muted-foreground">
          <span class="flex items-center gap-1"><Globe class="h-3 w-3"/> {{ t('footer.madeInCanada') }}</span>
        </div>
      </div>
    </div>
  </footer>
</template>