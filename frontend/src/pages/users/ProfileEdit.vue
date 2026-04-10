<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {useAuthStore} from '@/stores/auth'
import {updateProfile} from '@/api/user'
import {toast} from 'vue-sonner'
import {Button} from '@/components/ui/button'
import {Input} from '@/components/ui/input'
import {Label} from '@/components/ui/label'
import {Avatar, AvatarFallback, AvatarImage} from '@/components/ui/avatar'
import {Separator} from '@/components/ui/separator'
import {Card, CardContent} from '@/components/ui/card'
import {Camera, Save, User as UserIcon} from 'lucide-vue-next'
import type {ProfileDto, User} from "@/models/User.ts"

const authStore = useAuthStore()
const fileInput = ref<HTMLInputElement | null>(null)
const previewUrl = ref<string | null>(null)

const form = ref<ProfileDto>({
  id: '',
  username: '',
  first_name: '',
  last_name: '',
  avatar_url: '',
  preferences: { language: 'en', theme: 'dark' }
})

onMounted(() => {
  if (authStore.user) {
    Object.assign(form.value, authStore.user)
  }
})

const triggerUpload = () => fileInput.value?.click()

const onFileSelected = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    const file = target.files[0]
    form.value.avatar_url = file
    previewUrl.value = URL.createObjectURL(file)
  }
}

async function handleSave() {
  toast.promise(updateProfile(form.value), {
    loading: 'Synchronizing with server...',
    success: (updatedUser: User) => {
      authStore.setUser(updatedUser)
      previewUrl.value = null
      return 'Profile updated!'
    },
    error: (err: { message: any }) => err.message || 'Update failed',
  })
}
</script>

<template>
  <div class="max-w-4xl mx-auto space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-3xl font-black tracking-tight">Edit Profile</h2>
        <p class="text-muted-foreground">Modify your public identity and preferences.</p>
      </div>
      <Button @click="handleSave" class="rounded-xl shadow-lg shadow-primary/20 gap-2">
        <Save class="h-4 w-4" /> Save Changes
      </Button>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <div class="lg:col-span-1 space-y-6">
        <Card class="overflow-hidden border-2">
          <CardContent class="pt-8 pb-6 text-center">
            <div class="relative inline-block group">
              <Avatar class="h-32 w-32 border-4 border-background shadow-xl ring-2 ring-muted group-hover:ring-primary/50 transition-all duration-300">
                <AvatarImage :src="previewUrl || (form.avatar_url as string)" />
                <AvatarFallback class="text-3xl font-bold bg-primary text-white">
                  {{ form.username?.charAt(0).toUpperCase() }}
                </AvatarFallback>
              </Avatar>

              <button
                  @click="triggerUpload"
                  class="absolute bottom-0 right-0 p-2 bg-primary text-white rounded-full shadow-lg hover:scale-110 transition-transform"
              >
                <Camera class="h-4 w-4" />
              </button>

              <input type="file" ref="fileInput" class="hidden" accept="image/*" @change="onFileSelected" />
            </div>

            <div class="mt-4">
              <h3 class="font-bold text-lg">@{{ form.username }}</h3>
              <p class="text-sm text-muted-foreground italic">Public Handle</p>
            </div>
          </CardContent>
        </Card>

        <div class="bg-secondary/20 p-4 rounded-2xl border border-dashed border-muted-foreground/20">
          <p class="text-xs text-muted-foreground text-center">
            Images should be square and under 2MB for best results.
          </p>
        </div>
      </div>

      <Card class="lg:col-span-2 border-2">
        <CardContent class="p-6 space-y-6">
          <div class="flex items-center gap-2 text-primary font-bold uppercase tracking-wider text-xs">
            <UserIcon class="h-4 w-4" /> Basic Information
          </div>

          <div class="grid gap-4">
            <div class="grid gap-2">
              <Label for="username" class="font-bold text-xs uppercase text-muted-foreground">Username</Label>
              <Input v-model="form.username" id="username" class="rounded-xl bg-secondary/5 border-2 focus-visible:ring-primary" />
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div class="grid gap-2">
                <Label for="first_name" class="font-bold text-xs uppercase text-muted-foreground">First Name</Label>
                <Input v-model="form.first_name" id="first_name" class="rounded-xl bg-secondary/5 border-2 focus-visible:ring-primary" />
              </div>
              <div class="grid gap-2">
                <Label for="last_name" class="font-bold text-xs uppercase text-muted-foreground">Last Name</Label>
                <Input v-model="form.last_name" id="last_name" class="rounded-xl bg-secondary/5 border-2 focus-visible:ring-primary" />
              </div>
            </div>
          </div>

          <Separator class="my-6" />

          <div class="grid grid-cols-2 gap-4 opacity-70">
            <div class="p-3 border-2 border-dashed rounded-xl">
              <p class="text-[10px] font-black uppercase text-muted-foreground">Language</p>
              <p class="text-sm font-medium">{{ form.preferences.language === 'en' ? 'English' : 'Français' }}</p>
            </div>
            <div class="p-3 border-2 border-dashed rounded-xl">
              <p class="text-[10px] font-black uppercase text-muted-foreground">Theme</p>
              <p class="text-sm font-medium capitalize">{{ form.preferences.theme }}</p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>