<script setup lang="ts">
import { ref } from "vue"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Alert, AlertTitle } from "@/components/ui/alert"
import { FieldGroup, FieldLabel, FieldError } from '@/components/ui/field'
import { useForm, Field as VeeField } from 'vee-validate'
import { toTypedSchema } from "@vee-validate/zod"
import { z } from "zod"
import { ROUTES } from "@/router/routes.ts"
import { useI18n } from "vue-i18n"
import { forgotPassword } from "@/api"

const { t } = useI18n()
const error = ref("")
const isSubmitted = ref(false)

const recoverySchema = z.object({
  email: z.string().email(t('auth.login.error.invalidEmail' || 'Invalid email')),
})

const { handleSubmit } = useForm({
  validationSchema: toTypedSchema(recoverySchema),
  initialValues: {
    email: '',
  },
})

const submit = handleSubmit(async (values) => {
  error.value = ""
  try {
    // This calls your Rust backend: auth_service::request_password_reset
    await forgotPassword(values.email)
    isSubmitted.value = true
  } catch (e: any) {
    error.value = e.message || t("auth.recovery.error.failed")
  }
})
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-background p-4">
    <div class="w-full max-w-md rounded-xl border bg-card p-8 shadow-lg transition-all">

      <div class="flex flex-col space-y-2 text-center mb-8">
        <h2 class="text-2xl font-bold tracking-tight">
          {{ t('auth.recovery.title') }}
        </h2>
        <p class="text-sm text-muted-foreground">
          {{ isSubmitted ? t('auth.recovery.successSubtitle') : t('auth.recovery.subtitle') }}
        </p>
      </div>

      <div v-if="isSubmitted" class="space-y-6 animate-in fade-in slide-in-from-bottom-2 duration-300">
        <Alert class="bg-primary/5 border-primary/20 h-auto py-6 flex flex-col items-center justify-center">
          <AlertTitle class="text-center leading-relaxed whitespace-normal wrap-break-word block w-full text-sm">
            {{ t('auth.recovery.checkEmail') }}
          </AlertTitle>
        </Alert>

        <Button variant="outline" class="w-full h-11" @click="isSubmitted = false">
          {{ t('auth.recovery.resend') }}
        </Button>
      </div>

      <template v-else>
        <Alert
            variant="destructive"
            v-if="error"
            class="mb-6 animate-in fade-in zoom-in duration-200"
        >
          <AlertTitle class="whitespace-normal break-words !line-clamp-none">
            {{ error }}
          </AlertTitle>
        </Alert>

        <form id="recovery-form" @submit="submit" class="space-y-4">
          <FieldGroup>
            <vee-field name="email" v-slot="{ field, errors }">
              <div class="space-y-1">
                <FieldLabel class="text-xs font-semibold uppercase text-muted-foreground/70">
                  {{ t('auth.recovery.email') }}
                </FieldLabel>
                <Input
                    v-bind="field"
                    type="email"
                    placeholder="name@example.com"
                    autocomplete="email"
                    :class="{ 'border-destructive': errors.length }"
                    required
                />
                <FieldError v-if="errors.length" :errors="errors" class="text-[10px]"/>
              </div>
            </vee-field>

            <Button form="recovery-form" type="submit" class="w-full h-11 font-semibold transition-all mt-2">
              {{ t('auth.recovery.submit') }}
            </Button>
          </FieldGroup>
        </form>
      </template>

      <p class="mt-8 text-center text-sm border-t pt-6">
        <RouterLink :to="ROUTES.LOGIN" class="font-medium text-muted-foreground hover:text-primary transition-colors flex items-center justify-center gap-2">
          <span>&larr;</span> {{ t('auth.recovery.backToLogin') }}
        </RouterLink>
      </p>
    </div>
  </div>
</template>