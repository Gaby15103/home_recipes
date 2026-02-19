<script setup lang="ts">
import {ref} from "vue"
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Alert, AlertTitle} from "@/components/ui/alert"
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
} from '@/components/ui/field'
import {loginSchema} from "@/validators/auth.ts"
import {useForm, Field as VeeField} from 'vee-validate'
import {useAuthStore} from "@/stores/auth.ts"
import {useRouter} from "vue-router"
import {toTypedSchema} from "@vee-validate/zod";
import {ROUTES} from "@/router/routes.ts";
import {login} from "@/api";

const router = useRouter()
const error = ref("")
const authStore = useAuthStore()
import { useI18n } from "vue-i18n"
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
  <div class="min-h-screen flex items-center justify-center">
    <div class="w-full max-w-sm space-y-6 p-6 rounded-lg border bg-card shadow">
      <h2 class="text-2xl font-semibold text-center">
        {{ t('auth.login.title') }}
      </h2>

      <Alert
          variant="destructive"
          v-if="error"
          class="whitespace-normal break-words !line-clamp-none"
      >
        <AlertTitle class="whitespace-normal break-words !line-clamp-none">
          {{ error }}
        </AlertTitle>
      </Alert>



      <form id="login-form" @submit="submit" class="flex flex-col space-y-3">
        <FieldGroup>
          <vee-field v-slot="{ field, errors }" name="email">
            <Field :data-invalid="!!errors.email">
              <FieldLabel>{{ t('auth.login.email') }}</FieldLabel>
              <Input
                  v-bind="field"
                  type="email"
                  placeholder="email@example.com"
                  autocomplete="email"
                  required
              />
              <FieldError v-if="errors.email" :errors="errors"/>
            </Field>
          </vee-field>

          <vee-field v-slot="{ field, errors }" name="password">
            <Field :data-invalid="!!errors.length">
              <FieldLabel>{{ t('auth.login.password') }}</FieldLabel>
              <Input
                  v-bind="field"
                  type="password"
                  placeholder="password"
                  autocomplete="current-password"
                  required
              />
              <FieldError v-if="errors.length" :errors="errors"/>
            </Field>
          </vee-field>

          <Button type="submit" form="login-form" class="w-full mt-4">
            {{ t('auth.login.submit') }}
          </Button>
        </FieldGroup>

      </form>

      <p class="text-sm text-center text-muted-foreground">
        {{ t('auth.login.noAccount') }}
        <RouterLink class="underline" to="/register">
          {{ t('auth.login.register') }}
        </RouterLink>
      </p>
    </div>
  </div>
</template>
