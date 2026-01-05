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
import {loginSchema} from "@/validators/auth"
import {useForm, Field as VeeField} from 'vee-validate'
import {login} from "@/api/auth"
import {useUserStore} from "@/stores/user"
import {useRouter} from "vue-router"
import {toTypedSchema} from "@vee-validate/zod";

const router = useRouter()
const error = ref("")
const userStore = useUserStore()


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
    const res = await login(values.email, values.password)
    userStore.setUser(res.user)
    await router.push("/home")
  } catch (e: any) {
    error.value = e.message || "Login failed"
  }
})
</script>

<template>
  <div class="min-h-screen flex items-center justify-center">
    <div class="w-full max-w-sm space-y-6 p-6 rounded-lg border bg-card shadow">
      <h2 class="text-2xl font-semibold text-center">
        Login
      </h2>

      <Alert variant="destructive" v-if="error">
        <AlertTitle>{{ error }}</AlertTitle>
      </Alert>


      <form id="login-form" @submit="submit" class="flex flex-col space-y-3">
        <FieldGroup>
          <vee-field v-slot="{ field, errors }" name="email">
            <Field :data-invalid="!!errors.email">
              <FieldLabel>Email</FieldLabel>
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
              <FieldLabel>Password</FieldLabel>
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
            Login
          </Button>
        </FieldGroup>

      </form>

      <p class="text-sm text-center text-muted-foreground">
        No account?
        <RouterLink class="underline" to="/register">Register</RouterLink>
      </p>
    </div>
  </div>
</template>
