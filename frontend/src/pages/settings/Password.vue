<script setup lang="ts">
import { ref } from "vue";
import HeadingSmall from "@/components/common/HeadingSmall.vue";
import InputError from "@/components/common/InputError.vue";
import SettingsSettingsLayout from '@/layouts/SettingsLayout.vue'
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { editPassword } from "@/api/auth";
import type { BreadcrumbItem } from "@/types";
import { useRoute } from "vue-router";

// --------------------
// Breadcrumbs
// --------------------
const route = useRoute();

const breadcrumbs: BreadcrumbItem[] = [
  { title: "User", href: "/admin/user/edit/profile" },
  { title: "Password", href: "/admin/user/edit/password" },
];

// expose breadcrumbs to AdminSidebarHeader if you use route.meta
route.meta.breadcrumbs = breadcrumbs;

// --------------------
// Form state
// --------------------
const currentPassword = ref("");
const password = ref("");
const passwordConfirmation = ref("");

const processing = ref(false);
const success = ref(false);
const errors = ref<Record<string, string>>({});

// --------------------
// Submit
// --------------------
async function submit() {
  processing.value = true;
  success.value = false;
  errors.value = {};

  try {
    await editPassword(
        currentPassword.value,
        password.value,
        passwordConfirmation.value
    );

    success.value = true;

    currentPassword.value = "";
    password.value = "";
    passwordConfirmation.value = "";
  } catch (e: any) {
    if (e.response?.data?.errors) {
      errors.value = e.response.data.errors;
    } else {
      console.error("Password update failed", e);
    }
  } finally {
    processing.value = false;
  }
}
</script>

<template>
  <h1 class="sr-only">Password Modification</h1>
  <SettingsSettingsLayout>
    <div class="flex flex-col space-y-6">
      <HeadingSmall
          title="Update password"
          description="Ensure your account is using a long, random password to stay secure"
      />
      <form class="space-y-6" @submit.prevent="submit">
        <!-- Current password -->
        <div class="grid gap-2">
          <Label for="current_password">Current password</Label>
          <Input
              id="current_password"
              v-model="currentPassword"
              type="password"
              autocomplete="current-password"
              placeholder="Current password"
          />
          <InputError :message="errors.current_password" />
        </div>

        <!-- New password -->
        <div class="grid gap-2">
          <Label for="password">New password</Label>
          <Input
              id="password"
              v-model="password"
              type="password"
              autocomplete="new-password"
              placeholder="New password"
          />
          <InputError :message="errors.password" />
        </div>

        <!-- Confirmation -->
        <div class="grid gap-2">
          <Label for="password_confirmation">
            Confirm password
          </Label>
          <Input
              id="password_confirmation"
              v-model="passwordConfirmation"
              type="password"
              autocomplete="new-password"
              placeholder="Confirm password"
          />
          <InputError :message="errors.password_confirmation" />
        </div>

        <!-- Actions -->
        <div class="flex items-center gap-4">
          <Button type="submit" :disabled="processing">
            Save password
          </Button>

          <Transition
              enter-active-class="transition ease-in-out"
              enter-from-class="opacity-0"
              leave-active-class="transition ease-in-out"
              leave-to-class="opacity-0"
          >
            <p v-if="success" class="text-sm text-green-600">
              Password updated successfully.
            </p>
          </Transition>
        </div>
      </form>
    </div>
  </SettingsSettingsLayout>
</template>
