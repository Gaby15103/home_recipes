<script setup lang="ts">
import {onMounted, onUnmounted, reactive, ref} from 'vue'
import {useI18n} from 'vue-i18n'
import {Card, CardContent, CardDescription, CardHeader, CardTitle} from '@/components/ui/card'
import {Input} from '@/components/ui/input'
import {Label} from '@/components/ui/label'
import {Button} from '@/components/ui/button'
import {Badge} from '@/components/ui/badge'
import {KeyRound, Loader2, ShieldAlert, ShieldBan, ShieldCheck} from 'lucide-vue-next'
import {toast} from 'vue-sonner'

import {disableTwoFactor, editPassword, enableTwoFactor, getTwoFactorStatus} from '@/api'
import {useTwoFactorAuth} from '@/composables/useTwoFactorAuth'
import TwoFactorSetupModal from '@/components/auth/TwoFactorSetupModal.vue'

const { t } = useI18n()

const isSavingPassword = ref(false)
const passwordForm = reactive({
  current_password: '',
  new_password: '',
  confirm_password: '',
})

async function updatePassword() {
  if (!passwordForm.current_password) {
    toast.error("Please enter your current password")
    return
  }

  if (passwordForm.new_password.length < 8) {
    toast.error("New password must be at least 8 characters long")
    return
  }

  if (passwordForm.new_password !== passwordForm.confirm_password) {
    toast.error("New passwords do not match")
    return
  }

  isSavingPassword.value = true

  toast.promise(
      editPassword(
          passwordForm.current_password,
          passwordForm.new_password,
          passwordForm.confirm_password
      ),
      {
        loading: 'Updating your password...',
        success: () => {
          isSavingPassword.value = false
          passwordForm.current_password = ''
          passwordForm.new_password = ''
          passwordForm.confirm_password = ''
          return 'Password updated successfully!'
        },
        error: (err) => {
          isSavingPassword.value = false
          return err.response?.data?.message || 'Failed to update password. Please check your current password.'
        }
      }
  )
}

const twoFactorEnabled = ref(false)
const requiresConfirmation = ref(false)
const statusLoading = ref(true)
const showSetupModal = ref(false)
const isToggling2FA = ref(false)

const { hasSetupData, clearTwoFactorAuthData } = useTwoFactorAuth()

onMounted(async () => {
  try {
    const status = await getTwoFactorStatus()
    twoFactorEnabled.value = status.enabled
    requiresConfirmation.value = status.requires_confirmation
  } catch (e) {
    toast.error('Failed to load 2FA status')
  } finally {
    statusLoading.value = false
  }
})

onUnmounted(() => {
  clearTwoFactorAuthData()
})

const handleEnable2FA = async () => {
  isToggling2FA.value = true
  try {
    await enableTwoFactor()
    twoFactorEnabled.value = true
    showSetupModal.value = true
  } catch (e: any) {
    toast.error(e.message || 'Failed to enable 2FA')
  } finally {
    isToggling2FA.value = false
  }
}

const handleDisable2FA = async () => {
  isToggling2FA.value = true
  try {
    await disableTwoFactor()
    twoFactorEnabled.value = false
    toast.success("Two-factor authentication disabled")
  } catch (e: any) {
    toast.error(e.message || 'Failed to disable 2FA')
  } finally {
    isToggling2FA.value = false
  }
}
</script>

<template>
  <div class="space-y-6 animate-in fade-in slide-in-from-bottom-2 duration-500">

    <Card class="rounded-3xl border-2 shadow-none overflow-hidden">
      <CardHeader class="pb-4">
        <div class="flex items-center gap-2 text-primary">
          <KeyRound class="h-5 w-5" />
          <CardTitle class="text-lg font-bold uppercase tracking-tight">Password Management</CardTitle>
        </div>
        <CardDescription class="text-xs">
          Ensure your account stays secure by using a strong, unique password.
        </CardDescription>
      </CardHeader>

      <CardContent class="space-y-4">
        <div class="grid gap-4">
          <div class="grid gap-2">
            <Label class="text-[10px] font-black uppercase text-muted-foreground ml-1">Current Password</Label>
            <Input v-model="passwordForm.current_password" type="password" class="rounded-xl bg-secondary/10 border-transparent focus-visible:border-primary/50" />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div class="grid gap-2">
              <Label class="text-[10px] font-black uppercase text-muted-foreground ml-1">New Password</Label>
              <Input v-model="passwordForm.new_password" type="password" class="rounded-xl bg-secondary/10 border-transparent focus-visible:border-primary/50" />
            </div>
            <div class="grid gap-2">
              <Label class="text-[10px] font-black uppercase text-muted-foreground ml-1">Confirm New Password</Label>
              <Input v-model="passwordForm.confirm_password" type="password" class="rounded-xl bg-secondary/10 border-transparent focus-visible:border-primary/50" />
            </div>
          </div>
        </div>

        <div class="pt-2">
          <Button
              @click="updatePassword"
              :disabled="isSavingPassword"
              class="rounded-xl font-bold uppercase text-xs px-6 shadow-lg shadow-primary/10"
          >
            <Loader2 v-if="isSavingPassword" class="mr-2 h-3 w-3 animate-spin" />
            Update Password
          </Button>
        </div>
      </CardContent>
    </Card>

    <Card class="rounded-3xl border-2 shadow-none overflow-hidden">
      <CardHeader class="pb-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2 text-primary">
            <ShieldCheck class="h-5 w-5" />
            <CardTitle class="text-lg font-bold uppercase tracking-tight">Two-Factor Auth</CardTitle>
          </div>
          <Badge v-if="!statusLoading" :variant="twoFactorEnabled ? 'default' : 'destructive'" class="rounded-lg uppercase text-[10px]">
            {{ twoFactorEnabled ? 'Protected' : 'At Risk' }}
          </Badge>
        </div>
        <CardDescription class="text-xs">
          Adds an extra layer of security to your account by requiring a code from your phone.
        </CardDescription>
      </CardHeader>

      <CardContent class="space-y-6">
        <div v-if="statusLoading" class="py-4 flex justify-center">
          <Loader2 class="h-6 w-6 animate-spin text-muted-foreground" />
        </div>

        <div v-else class="space-y-4">
          <div class="p-4 rounded-2xl bg-secondary/10 border border-transparent flex items-start gap-4">
            <div :class="[
              'p-3 rounded-xl',
              twoFactorEnabled ? 'bg-primary/10 text-primary' : 'bg-destructive/10 text-destructive'
            ]">
              <ShieldAlert v-if="!twoFactorEnabled" class="h-5 w-5" />
              <ShieldCheck v-else class="h-5 w-5" />
            </div>
            <div class="space-y-1">
              <p class="text-sm font-bold">
                {{ twoFactorEnabled ? '2FA is currently active' : '2FA is currently disabled' }}
              </p>
              <p class="text-[10px] text-muted-foreground leading-tight max-w-sm">
                {{ twoFactorEnabled
                  ? 'Your account is secured with an additional verification step.'
                  : 'We highly recommend enabling 2FA to prevent unauthorized access to your recipes and data.'
                }}
              </p>
            </div>
          </div>

          <div class="flex gap-3">
            <Button
                v-if="twoFactorEnabled"
                variant="destructive"
                @click="handleDisable2FA"
                :disabled="isToggling2FA"
                class="rounded-xl font-bold uppercase text-xs px-6"
            >
              <ShieldBan class="mr-2 h-4 w-4" />
              Disable 2FA
            </Button>

            <Button
                v-else-if="hasSetupData"
                @click="showSetupModal = true"
                class="rounded-xl font-bold uppercase text-xs px-6 shadow-lg shadow-primary/20"
            >
              <ShieldCheck class="mr-2 h-4 w-4" />
              Continue Setup
            </Button>

            <Button
                v-else
                @click="handleEnable2FA"
                :disabled="isToggling2FA"
                class="rounded-xl font-bold uppercase text-xs px-6 shadow-lg shadow-primary/20"
            >
              <Loader2 v-if="isToggling2FA" class="mr-2 h-3 w-3 animate-spin" />
              <ShieldCheck v-else class="mr-2 h-4 w-4" />
              Enable 2FA
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>

    <TwoFactorSetupModal
        v-model:isOpen="showSetupModal"
        :requiresConfirmation="requiresConfirmation"
        :twoFactorEnabled="twoFactorEnabled"
    />
  </div>
</template>