<script setup lang="ts">
import {ref} from "vue"
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Alert, AlertTitle} from "@/components/ui/alert"
import {Field, FieldError, FieldGroup, FieldLabel} from "@/components/ui/field"
import {useForm, Field as VeeField} from "vee-validate"
import {toTypedSchema} from "@vee-validate/zod"
import {registerSchema} from "@/validators/auth"
import {registerUser} from "@/api/auth"
import {useRouter} from "vue-router"

const router = useRouter()
const error = ref("")

const {handleSubmit} = useForm({
  validationSchema: toTypedSchema(registerSchema),
  initialValues: {
    username: "",
    email: "",
    password: "",
    first_name: "",
    last_name: "",
  },
})

const submit = handleSubmit(async (values) => {
  error.value = ""
  try {
    const res = await registerUser(
        values.username,
        values.email,
        values.password,
        values.first_name,
        values.last_name
    )
    await router.push("/login")
  } catch (e: any) {
    error.value = e.message || "Registration failed"
  }
})
</script>

<template>
  <div class="min-h-screen flex items-center justify-center">
    <div class="w-full max-w-sm rounded-lg border bg-card p-5 shadow">
      <h2 class="mb-4 text-center text-xl font-semibold">
        create account
      </h2>

      <Alert v-if="error" variant="destructive" class="mb-3">
        <AlertTitle>{{ error }}</AlertTitle>
      </Alert>

      <form id="register-form" @submit="submit" class="space-y-2">
        <FieldGroup>
          <vee-field name="username" v-slot="{ field, errors }">
            <Field :data-invalid="!!errors.length">
              <FieldLabel class="text-sm">Username</FieldLabel>
              <Input
                  v-bind="field"
                  type="text"
                  placeholder="Username"
                  autocomplete="username"
                  required
              />
              <FieldError v-if="errors.length" :errors="errors"/>
            </Field>
          </vee-field>

          <!-- Email -->
          <vee-field name="email" v-slot="{ field, errors }">
            <Field :data-invalid="!!errors.email">
              <FieldLabel class="text-sm">Email</FieldLabel>
              <Input
                  v-bind="field"
                  type="email"
                  placeholder="Email"
                  autocomplete="email"
                  required
              />
              <FieldError v-if="errors.email" :errors="errors"/>
            </Field>
          </vee-field>

          <!-- Names row -->
          <div class="grid grid-cols-2 gap-3">
            <vee-field name="first_name" v-slot="{ field, errors }">
              <Field :data-invalid="!!errors.length">
                <FieldLabel class="text-sm">First name</FieldLabel>
                <Input
                    v-bind="field"
                    type="text"
                    placeholder="First name"
                    autocomplete="first_name"
                    required
                />
                <FieldError v-if="errors.length" :errors="errors"/>
              </Field>
            </vee-field>

            <vee-field name="last_name" v-slot="{ field, errors }">
              <Field :data-invalid="!!errors.length">
                <FieldLabel class="text-sm">Last name</FieldLabel>
                <Input
                    v-bind="field"
                    type="text"
                    placeholder="Last name"
                    autocomplete="last_name"
                    required
                />
                <FieldError v-if="errors.length" :errors="errors"/>
              </Field>
            </vee-field>
          </div>

          <!-- Password -->
          <vee-field name="password" v-slot="{ field, errors }">
            <Field :data-invalid="!!errors.length">
              <FieldLabel class="text-sm">Password</FieldLabel>
              <Input
                  v-bind="field"
                  type="password"
                  placeholder="Password"
                  autocomplete="current-password"
                  required
              />
              <FieldError v-if="errors.length" :errors="errors"/>
            </Field>
          </vee-field>

          <Button form="register-form" type="submit" class="mt-3 w-full">
            Register
          </Button>
        </FieldGroup>
      </form>

      <p class="mt-3 text-center text-xs text-muted-foreground">
        Already registered?
        <RouterLink to="/login" class="underline">Login</RouterLink>
      </p>
    </div>
  </div>
</template>
