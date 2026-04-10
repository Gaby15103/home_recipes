<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Button } from '@/components/ui/button'
import { ShieldCheck, Loader2, CheckCircle2, ArrowLeft, Lock } from 'lucide-vue-next'
import { ROUTES } from '@/router/routes'

const { t } = useI18n()
const isSaving = ref(false)
const showSuccess = ref(false)

const form = reactive({
  current_password: '',
  new_password: '',
  confirm_password: '',
})

async function updatePassword() {
  isSaving.value = true
  // Simulation API (à remplacer par ton appel editUser ou changePassword)
  setTimeout(() => {
    isSaving.value = false
    showSuccess.value = true
    form.current_password = ''
    form.new_password = ''
    form.confirm_password = ''
    setTimeout(() => (showSuccess.value = false), 3000)
  }, 1000)
}
</script>

<template>
  <div class="container max-w-5xl py-10 space-y-8 animate-in fade-in slide-in-from-bottom-3 duration-500">

    <RouterLink
        :to="ROUTES.USER.SETTINGS"
        class="inline-flex items-center text-sm font-medium text-muted-foreground hover:text-primary transition-colors group"
    >
      <ArrowLeft class="mr-2 h-4 w-4 transition-transform group-hover:-translate-x-1" />
      {{ t('settings.actions.back') }}
    </RouterLink>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8 items-start">

      <!-- Info Section -->
      <div class="space-y-6">
        <div class="px-2">
          <h1 class="text-3xl font-bold tracking-tight">{{ t('settings.hub.security.title') }}</h1>
          <p class="text-muted-foreground mt-2 italic border-l-2 border-primary/20 pl-4">
            {{ t('settings.hub.security.desc') }}
          </p>
        </div>

        <div class="p-6 rounded-2xl bg-secondary/30 flex items-center gap-4 border border-secondary">
          <div class="p-3 bg-primary/10 rounded-xl text-primary">
            <Lock class="h-6 w-6" />
          </div>
          <p class="text-xs text-muted-foreground leading-relaxed">
            {{ t('settings.hub.security.password_desc') }}
          </p>
        </div>
      </div>

      <!-- Form Section -->
      <div class="lg:col-span-2">
        <Card class="border shadow-sm bg-card/50 backdrop-blur-sm">
          <CardHeader>
            <CardTitle>{{ t('settings.hub.security.password_title') }}</CardTitle>
          </CardHeader>
          <CardContent>
            <form @submit.prevent="updatePassword" class="space-y-5">

              <div class="space-y-2">
                <Label>{{ t('settings.hub.security.current_password') }}</Label>
                <Input v-model="form.current_password" type="password" class="bg-background/50" />
              </div>

              <div class="grid grid-cols-1 sm:grid-cols-2 gap-5">
                <div class="space-y-2">
                  <Label>{{ t('settings.hub.security.new_password') }}</Label>
                  <Input v-model="form.new_password" type="password" class="bg-background/50" />
                </div>
                <div class="space-y-2">
                  <Label>{{ t('settings.hub.security.confirm_password') }}</Label>
                  <Input v-model="form.confirm_password" type="password" class="bg-background/50" />
                </div>
              </div>

              <div class="flex items-center gap-5 pt-6 border-t border-muted/30">
                <Button
                    type="submit"
                    class="min-w-[140px] shadow-lg shadow-primary/20"
                    :disabled="isSaving"
                >
                  <Loader2 v-if="isSaving" class="mr-2 h-4 w-4 animate-spin" />
                  {{ isSaving ? t('settings.actions.saving') : t('settings.actions.save') }}
                </Button>

                <Transition name="fade">
                  <span v-if="showSuccess" class="flex items-center gap-2 text-green-600 font-bold text-sm">
                    <CheckCircle2 class="h-4 w-4" />
                    {{ t('settings.hub.security.success') }}
                  </span>
                </Transition>
              </div>
            </form>
          </CardContent>
        </Card>
      </div>

    </div>
  </div>
</template>