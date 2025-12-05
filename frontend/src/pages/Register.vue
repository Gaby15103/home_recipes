<script setup lang="ts">
import { ref } from "vue"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { useRouter } from "vue-router"

const email = ref("")
const password = ref("")
const name = ref("")
const router = useRouter()

const register = async () => {
  const res = await fetch("http://localhost:8080/api/users", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      name: name.value,
      email: email.value,
      password: password.value
    })
  })

  if (res.ok) {
    router.push("/login") // redirect to login
  } else {
    alert("Registration failed")
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center">
    <div class="w-full max-w-sm space-y-6 p-6 rounded-lg border bg-card shadow">
      <h2 class="text-2xl font-semibold text-center">Create Account</h2>

      <div class="space-y-4">
        <div>
          <Label>Name</Label>
          <Input v-model="name" />
        </div>

        <div>
          <Label>Email</Label>
          <Input type="email" v-model="email" />
        </div>

        <div>
          <Label>Password</Label>
          <Input type="password" v-model="password" />
        </div>

        <Button class="w-full" @click="register">
          Register
        </Button>

        <p class="text-sm text-center text-muted-foreground">
          Already registered?
          <RouterLink class="underline" to="/login">Login</RouterLink>
        </p>
      </div>
    </div>
  </div>
</template>
