<script setup lang="ts">
import { ref, reactive } from "vue"
import { useRouter } from "vue-router"
import { useAuthStore } from "@/stores/auth.ts"
import { deleteUser } from "@/api/auth"

// Components
import HeadingSmall from "@/components/HeadingSmall.vue"
import InputError from "@/components/InputError.vue"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"

const router = useRouter()
const authStore = useAuthStore()

const password = ref("")
const loading = ref(false)
const errors = reactive<{ password?: string }>({})
const open = ref(false)

async function submit() {
  loading.value = true
  errors.password = undefined

  try {
    await deleteUser(password.value)

    // Logout locally
    await authStore.logout()

    // Redirect to login
    await router.push("/login")
  }
  catch (e: any) {
    if (e.response?.data?.errors?.password) {
      errors.password = e.response.data.errors.password
    } else {
      errors.password = "Failed to delete account."
      console.error(e)
    }
  }
  finally {
    loading.value = false
  }
}

function reset() {
  password.value = ""
  errors.password = undefined
}
</script>

<template>
  <div class="space-y-6">
    <HeadingSmall
        title="Delete account"
        description="Delete your account and all of its resources"
    />
    <div
        class="space-y-4 rounded-lg border border-red-100 bg-red-50 p-4 dark:border-red-200/10 dark:bg-red-700/10"
    >
      <div class="relative space-y-0.5 text-red-600 dark:text-red-100">
        <p class="font-medium">Warning</p>
        <p class="text-sm">
          Please proceed with caution, this cannot be undone.
        </p>
      </div>
      <Dialog v-model:open="open">
        <DialogTrigger as-child>
          <Button variant="destructive">
            Delete account
          </Button>
        </DialogTrigger>

        <DialogContent>
          <form @submit.prevent="submit" class="space-y-6">
            <DialogHeader class="space-y-3">
              <DialogTitle>
                Are you sure you want to delete your account?
              </DialogTitle>
              <DialogDescription>
                Once your account is deleted, all data will be permanently removed.
                Please enter your password to confirm.
              </DialogDescription>
            </DialogHeader>

            <div class="grid gap-2">
              <Label for="password" class="sr-only">Password</Label>
              <Input
                  id="password"
                  type="password"
                  v-model="password"
                  placeholder="Password"
                  required
              />
              <InputError :message="errors.password" />
            </div>

            <DialogFooter class="gap-2">
              <DialogClose as-child>
                <Button
                    type="button"
                    variant="secondary"
                    @click="reset"
                >
                  Cancel
                </Button>
              </DialogClose>

              <Button
                  type="submit"
                  variant="destructive"
                  :disabled="loading"
              >
                {{ loading ? "Deleting..." : "Delete account" }}
              </Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>

    </div>
  </div>
</template>
