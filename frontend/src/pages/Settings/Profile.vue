<script setup lang="ts">
import {ref, reactive, watch} from "vue"
import DeleteUser from '@/components/DeleteUser.vue'
import HeadingSmall from '@/components/HeadingSmall.vue'
import InputError from '@/components/InputError.vue'
import {Button} from '@/components/ui/button'
import {Input} from '@/components/ui/input'
import {Label} from '@/components/ui/label'
import SettingsLayout from '@/components/layout/settings/Layout.vue'
import {useAuthStore} from "@/stores/auth.ts"
import {editUser} from "@/api/auth"
import type {BreadcrumbItem} from "@/types";

const authStore = useAuthStore()
const user = authStore.user!

const loading = ref(false)
const success = ref(false)

const errors = reactive<Record<string, string>>({})

// form state
const form = reactive({
  username: user.username,
  first_name: user.first_name,
  last_name: user.last_name,
  email: user.email
})

// reset success message when typing
watch(form, () => {
  success.value = false
})

async function submit() {
  loading.value = true
  success.value = false
  Object.keys(errors).forEach(k => delete errors[k])

  try {
    const updatedUser = {
      ...user,
      username: form.username,
      first_name: form.first_name,
      last_name: form.last_name,
      email: form.email,
      avatar_url: user.avatar_url,
      preferences: user.preferences
    }

    const {data} = await editUser(updatedUser)

    // Update auth store user
    authStore.setUser(data.user)

    success.value = true
  } catch (e: any) {
    if (e.response?.data?.errors) {
      Object.assign(errors, e.response.data.errors)
    } else {
      console.error(e)
    }
  } finally {
    loading.value = false
  }
}

const breadcrumbItems: BreadcrumbItem[] = [
  {
    title: 'Profile settings',
    href: '/admin/user/edit',
  },
];
</script>

<template>
  <h1 class="sr-only">Profile Settings</h1>

  <SettingsLayout>
    <div class="flex flex-col space-y-6">
      <HeadingSmall
          title="Profile information"
          description="Update your profile"
      />

      <form class="space-y-6" @submit.prevent="submit">
        <div class="grid gap-2">
          <Label for="username">Username</Label>
          <Input
              id="username"
              class="mt-1 block w-full"
              v-model="form.username"
              required
              autocomplete="username"
              placeholder="Username"
          />
          <InputError class="mt-2" :message="errors.username"/>
        </div>

        <!-- First Name -->
        <div class="grid gap-2">
          <Label for="first_name">First Name</Label>
          <Input
              id="first_name"
              v-model="form.first_name"
              required
          />
          <InputError :message="errors.first_name"/>
        </div>

        <!-- Last Name -->
        <div class="grid gap-2">
          <Label for="last_name">Last Name</Label>
          <Input
              id="last_name"
              v-model="form.last_name"
              required
          />
          <InputError :message="errors.last_name"/>
        </div>

        <!-- Email -->
        <div class="grid gap-2">
          <Label for="email">Email address</Label>
          <Input
              id="email"
              type="email"
              v-model="form.email"
              required
          />
          <InputError :message="errors.email"/>
        </div>

        <!-- Actions -->
        <div class="flex items-center gap-4">
          <Button :disabled="loading">
            {{ loading ? "Saving..." : "Save" }}
          </Button>

          <p v-if="success" class="text-sm text-green-600">
            Saved successfully.
          </p>
        </div>
      </form>
    </div>

    <DeleteUser/>
  </SettingsLayout>
</template>
