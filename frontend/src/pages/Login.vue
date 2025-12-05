<script setup lang="ts">
import { ref } from "vue"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { login } from "@/api/auth.ts";
import { useUserStore } from "@/stores/user";
import {Alert, AlertTitle} from "@/components/ui/alert";
import {router} from "@/router";

const email = ref("")
const password = ref("")
const error = ref("");

async function submit() {
  error.value = "";
  const userStore = useUserStore();
  try {
    const res = await login(email.value, password.value);
    userStore.setUser(res.user, res.user.token);
    await router.push("/Home");
  } catch (e: any) {
    error.value = e.message || "Login failed";
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center">
    <div class="w-full max-w-sm space-y-6 p-6 rounded-lg border bg-card shadow">
      <h2 class="text-2xl font-semibold text-center">Login</h2>
      <div class="space-y-4">
        <div>
          <Alert variant="destructive" v-if="error">
            <AlertTitle>{{ error }}</AlertTitle>
          </Alert>
        </div>
        <div>
          <Label>Email</Label>
          <Input type="email" v-model="email" />
        </div>

        <div>
          <Label>Password</Label>
          <Input type="password" v-model="password" />
        </div>

        <Button class="w-full" @click="submit">
          Login
        </Button>

        <p class="text-sm text-center text-muted-foreground">
          No account? <RouterLink class="underline" to="/register">Register</RouterLink>
        </p>
      </div>
    </div>
  </div>
</template>
