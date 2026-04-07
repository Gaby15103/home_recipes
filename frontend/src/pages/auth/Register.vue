<script setup lang="ts">
import {ref} from "vue"
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Alert, AlertTitle} from "@/components/ui/alert"
import {FieldError, FieldLabel} from "@/components/ui/field"
import {Field as VeeField, useForm} from "vee-validate"
import {toTypedSchema} from "@vee-validate/zod"
import {registerSchema} from "@/validators/auth.ts"
import {registerUser} from "@/api/auth.ts"
import {useRouter} from "vue-router"
import {useI18n} from "vue-i18n"
import {ROUTES} from "@/router/routes.ts";

const { t } = useI18n()
const router = useRouter()
const error = ref("")

const {handleSubmit} = useForm({
  validationSchema: toTypedSchema(registerSchema),
  initialValues: {
    username: "",
    email: "",
    email_confirmation: "",
    password: "",
    first_name: "",
    last_name: "",
  },
})

const submit = handleSubmit(async (values) => {
  error.value = ""
  try {
    await registerUser(
        values.username,
        values.email,
        values.password,
        values.first_name,
        values.last_name
    )
    await router.push("/login")
  } catch (e: any) {
    error.value = e.message || t('auth.register.error.failed')
  }
})
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-background p-4">
    <div class="w-full max-w-md rounded-xl border bg-card p-8 shadow-lg transition-all">
      <div class="flex flex-col space-y-2 text-center mb-8">
        <h2 class="text-2xl font-bold tracking-tight">
          {{ t('auth.register.title') }}
        </h2>
        <p class="text-sm text-muted-foreground">
          {{ t('auth.register.subtitle') || 'Enter your details below to create your account' }}
        </p>
      </div>

      <Alert
          variant="destructive"
          v-if="error"
          class="mb-6 animate-in fade-in zoom-in duration-200"
      >
        <AlertTitle>{{ error }}</AlertTitle>
      </Alert>

      <form id="register-form" @submit="submit" class="space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <vee-field name="first_name" v-slot="{ field, errors }">
            <div class="space-y-1">
              <FieldLabel class="text-xs font-semibold uppercase text-muted-foreground/70">
                {{ t('auth.register.firstName') }}
              </FieldLabel>
              <Input
                  v-bind="field"
                  autocomplete="given-name"
                  placeholder="Jean"
                  :class="{ 'border-destructive': errors.length }"
              />
              <FieldError v-if="errors.length" :errors="errors" class="text-[10px]"/>
            </div>
          </vee-field>

          <vee-field name="last_name" v-slot="{ field, errors }">
            <div class="space-y-1">
              <FieldLabel class="text-xs font-semibold uppercase text-muted-foreground/70">
                {{ t('auth.register.lastName') }}
              </FieldLabel>
              <Input
                  v-bind="field"
                  autocomplete="family-name"
                  placeholder="Dupont"
                  :class="{ 'border-destructive': errors.length }"
              />
              <FieldError v-if="errors.length" :errors="errors" class="text-[10px]"/>
            </div>
          </vee-field>
        </div>

        <vee-field name="username" v-slot="{ field, errors }">
          <div class="space-y-1">
            <FieldLabel class="text-xs font-semibold uppercase text-muted-foreground/70">
              {{ t('auth.register.username') }}
            </FieldLabel>
            <Input
                v-bind="field"
                autocomplete="username"
                placeholder="gaby15103"
                :class="{ 'border-destructive': errors.length }"
            />
            <FieldError v-if="errors.length" :errors="errors" class="text-[10px]"/>
          </div>
        </vee-field>

        <vee-field name="email" v-slot="{ field, errors }">
          <div class="space-y-1">
            <FieldLabel class="text-xs font-semibold uppercase text-muted-foreground/70">
              {{ t('auth.register.email') }}
            </FieldLabel>
            <Input
                v-bind="field"
                type="email"
                autocomplete="email"
                placeholder="name@example.com"
                :class="{ 'border-destructive': errors.length }"
            />
            <FieldError v-if="errors.length" :errors="errors" class="text-[10px]"/>
          </div>
        </vee-field>

        <vee-field name="email_confirmation" v-slot="{ field, errors }">
          <div class="space-y-1">
            <FieldLabel class="text-xs font-semibold uppercase text-muted-foreground/70">
              {{ t('auth.register.confirmEmail') }}
            </FieldLabel>
            <Input
                v-bind="field"
                type="email"
                autocomplete="email"
                placeholder="Confirm your email"
                :class="{ 'border-destructive': errors.length }"
            />
            <FieldError v-if="errors.length" :errors="errors" class="text-[10px]"/>
          </div>
        </vee-field>

        <vee-field name="password" v-slot="{ field, errors }">
          <div class="space-y-1">
            <FieldLabel class="text-xs font-semibold uppercase text-muted-foreground/70">
              {{ t('auth.register.password') }}
            </FieldLabel>
            <Input
                v-bind="field"
                type="password"
                autocomplete="new-password"
                placeholder="••••••••"
                :class="{ 'border-destructive': errors.length }"
            />
            <FieldError v-if="errors.length" :errors="errors" class="text-[10px]"/>
          </div>
        </vee-field>

        <Button form="register-form" type="submit" class="w-full h-11 font-semibold transition-all hover:bg-primary/90">
          {{ t('auth.register.submit') }}
        </Button>
      </form>

      <div class="relative mt-8">
        <div class="absolute inset-0 flex items-center">
          <span class="w-full border-t"></span>
        </div>
        <div class="relative flex justify-center text-xs uppercase">
          <span class="bg-card px-2 text-muted-foreground">
            {{ t('auth.register.already') }}
          </span>
        </div>
      </div>

      <p class="mt-4 text-center text-sm">
        <RouterLink :to="ROUTES.LOGIN" class="font-medium text-primary hover:underline underline-offset-4">
          {{ t('auth.register.login') }}
        </RouterLink>
      </p>
    </div>
  </div>
</template>