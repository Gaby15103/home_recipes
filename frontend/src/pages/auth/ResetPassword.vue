<script setup lang="ts">
import { ref, onMounted } from "vue"
import { useRoute, useRouter } from "vue-router"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Alert, AlertTitle } from "@/components/ui/alert"
import { FieldGroup, FieldLabel, FieldError } from '@/components/ui/field'
import { useForm, Field as VeeField } from 'vee-validate'
import { toTypedSchema } from "@vee-validate/zod"
import { z } from "zod"
import { ROUTES } from "@/router/routes.ts"
import { useI18n } from "vue-i18n"
import { resetPassword } from "@/api"

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const error = ref("")
const success = ref(false)
const token = ref(route.query.token as string)

// Validation schema matching your Rust ResetPasswordDto
const resetSchema = z.object({
  password: z.string().min(8, t('auth.register.error.passwordShort')),
  confirm_password: z.string()
}).refine(data => data.password === data.confirm_password, {
  message: t('auth.register.error.emailMismatch'),
  path: ["confirm_password"]
})

const { handleSubmit } = useForm({
  validationSchema: toTypedSchema(resetSchema),
  initialValues: {
    password: '',
    confirm_password: '',
  },
})

const submit = handleSubmit(async (values) => {
  if (!token.value) {
    error.value = t('auth.reset.error.missingToken')
    return
  }

  error.value = ""
  try {
    // Calls your Rust backend: auth_service::reset_password
    await resetPassword(token.value, values.password)
    success.value = true

    // Redirect to login after a brief delay so they can see the success message
    setTimeout(() => {
      router.push(ROUTES.LOGIN)
    }, 3000)
  } catch (e: any) {
    error.value = e.message || t("auth.reset.error.failed")
  }
})

onMounted(() => {
  if (!token.value) {
    error.value = t('auth.reset.error.missingToken')
  }
})
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-background p-4">
    <div class="w-full max-w-md rounded-xl border bg-card p-8 shadow-lg transition-all">

      <div class="flex flex-col space-y-2 text-center mb-8">
        <h2 class="text-2xl font-bold tracking-tight">
          {{ t('auth.reset.title') }}
        </h2>
        <p class="text-sm text-muted-foreground">
          {{ t('auth.reset.subtitle') }}
        </p>
      </div>

      <Alert v-if="success" class="bg-primary/5 border-primary/20 mb-6 animate-in fade-in zoom-in">
        <AlertTitle class="text-center text-primary font-medium wrap-break-word">
          {{ t('auth.reset.success') }}
        </AlertTitle>
      </Alert>

      <template v-else>
        <Alert
            variant="destructive"
            v-if="error"
            class="mb-6 animate-in fade-in zoom-in duration-200"
        >
          <AlertTitle class="whitespace-normal wrap-break-word line-clamp-none!">
            {{ error }}
          </AlertTitle>
        </Alert>

        <form id="reset-form" @submit="submit" class="space-y-4">
          <FieldGroup>
            <vee-field name="password" v-slot="{ field, errors }">
              <div class="space-y-1">
                <FieldLabel class="text-xs font-semibold uppercase text-muted-foreground/70">
                  {{ t('auth.reset.newPassword') }}
                </FieldLabel>
                <Input
                    v-bind="field"
                    type="password"
                    placeholder="••••••••"
                    autocomplete="new-password"
                    :class="{ 'border-destructive': errors.length }"
                    required
                />
                <FieldError v-if="errors.length" :errors="errors" class="text-[10px]"/>
              </div>
            </vee-field>

            <vee-field name="confirm_password" v-slot="{ field, errors }">
              <div class="space-y-1">
                <FieldLabel class="text-xs font-semibold uppercase text-muted-foreground/70">
                  {{ t('auth.reset.confirmPassword') }}
                </FieldLabel>
                <Input
                    v-bind="field"
                    type="password"
                    placeholder="••••••••"
                    autocomplete="new-password"
                    :class="{ 'border-destructive': errors.length }"
                    required
                />
                <FieldError v-if="errors.length" :errors="errors" class="text-[10px]"/>
              </div>
            </vee-field>

            <Button
                form="reset-form"
                type="submit"
                class="w-full h-11 font-semibold transition-all mt-2"
                :disabled="!!error && !token"
            >
              {{ t('auth.reset.submit') }}
            </Button>
          </FieldGroup>
        </form>
      </template>

      <p class="mt-8 text-center text-sm border-t pt-6">
        <RouterLink :to="ROUTES.LOGIN" class="font-medium text-muted-foreground hover:text-primary transition-colors">
          {{ t('auth.recovery.backToLogin') }}
        </RouterLink>
      </p>
    </div>
  </div>
</template>