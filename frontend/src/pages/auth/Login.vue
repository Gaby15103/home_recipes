<script setup lang="ts">
import {ref} from "vue"
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Alert, AlertTitle} from "@/components/ui/alert"
import {FieldError, FieldLabel,} from '@/components/ui/field'
import {loginSchema} from "@/validators/auth.ts"
import {Field as VeeField, useForm} from 'vee-validate'
import {useAuthStore} from "@/stores/auth.ts"
import {useRouter} from "vue-router"
import {toTypedSchema} from "@vee-validate/zod";
import {ROUTES} from "@/router/routes.ts";
import {login} from "@/api";
import {useI18n} from "vue-i18n"

const router = useRouter()
const error = ref("")
const authStore = useAuthStore()
const { t } = useI18n()

const {handleSubmit} = useForm({
  validationSchema: toTypedSchema(loginSchema),
  initialValues: {
    email: '',
    password: '',
  },
})

const submit = handleSubmit(async (values) => {
  error.value = ""

  try {

    const res = await login(values.email, values.password);

    if (res.two_factor_required) {
      if (!res.two_factor_token){
        error.value = t("auth.login.error.missing2fa")
        return
      }
      authStore.setPendingTwoFactor(res.two_factor_token)
      await router.push(ROUTES.TWO_FACTOR)
      return
    }
    if (!res.user)
      return

    authStore.setUser(res.user)
    await router.push(ROUTES.HOME)
  } catch (e: any) {
    error.value = e.message || t("auth.login.error.failed")
  }
})

</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-background p-4">
    <div class="w-full max-w-md rounded-xl border bg-card p-8 shadow-lg transition-all">
      <div class="flex flex-col space-y-2 text-center mb-8">
        <h2 class="text-2xl font-bold tracking-tight">
          {{ t('auth.login.title') }}
        </h2>
        <p class="text-sm text-muted-foreground">
          {{ t('auth.login.subtitle') }}
        </p>
      </div>

      <Alert
          variant="destructive"
          v-if="error"
          class="mb-6 animate-in fade-in zoom-in duration-200"
      >
        <AlertTitle class="whitespace-normal break-words !line-clamp-none">
          {{ error }}
        </AlertTitle>
      </Alert>

      <form id="login-form" @submit="submit" class="space-y-4">
        <vee-field name="email" v-slot="{ field, errors }">
          <div class="space-y-1">
            <FieldLabel class="text-xs font-semibold uppercase text-muted-foreground/70">
              {{ t('auth.login.email') }}
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

        <vee-field name="password" v-slot="{ field, errors }">
          <div class="space-y-1">
            <div class="flex items-center justify-between">
              <FieldLabel class="text-xs font-semibold uppercase text-muted-foreground/70">
                {{ t('auth.login.password') }}
              </FieldLabel>
              <RouterLink
                  :to="ROUTES.FORGOT_PASSWORD"
                  class="text-xs font-medium text-primary hover:underline underline-offset-4"
              >
                {{ t('auth.login.forgotPassword') }}
              </RouterLink>
            </div>
            <Input
                v-bind="field"
                type="password"
                placeholder="••••••••"
                autocomplete="current-password"
                :class="{ 'border-destructive': errors.length }"
                required
            />
            <FieldError v-if="errors.length" :errors="errors" class="text-[10px]"/>
          </div>
        </vee-field>

        <Button form="login-form" type="submit" class="w-full h-11 font-semibold transition-all hover:bg-primary/90 mt-2">
          {{ t('auth.login.submit') }}
        </Button>
      </form>

      <div class="relative mt-8">
        <div class="absolute inset-0 flex items-center">
          <span class="w-full border-t"></span>
        </div>
        <div class="relative flex justify-center text-xs uppercase">
          <span class="bg-card px-2 text-muted-foreground">
            {{ t('auth.login.noAccount') }}
          </span>
        </div>
      </div>

      <p class="mt-4 text-center text-sm">
        <RouterLink :to="ROUTES.REGISTER" class="font-medium text-primary hover:underline underline-offset-4">
          {{ t('auth.login.register') }}
        </RouterLink>
      </p>
    </div>
  </div>
</template>
