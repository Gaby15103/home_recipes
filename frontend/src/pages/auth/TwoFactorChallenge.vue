<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'

import InputError from '@/components/common/InputError.vue'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  InputOTP,
  InputOTPGroup,
  InputOTPSlot,
} from '@/components/ui/input-otp'

import AuthLayout from '@/layouts/AuthLayout.vue'
import { useAuthStore } from '@/stores/auth'
import { verifyTwoFactor } from '@/api'
import { ROUTES } from '@/router/routes'
import { useI18n } from "vue-i18n"
const { t } = useI18n()
interface AuthConfigContent {
  title: string
  description: string
  toggleText: string
}

const router = useRouter()
const authStore = useAuthStore()

const showRecoveryInput = ref(false)
const code = ref('')
const recoveryCode = ref('')
const error = ref('')
const processing = ref(false)

onMounted(() => {
  // Protect route access
  if (!authStore.twoFactorPending) {
    router.replace(ROUTES.LOGIN)
  }
})

const authConfigContent = computed(() => {
  if (showRecoveryInput.value) {
    return {
      title: t('auth.twoFactor.recoveryTitle'),
      description: t('auth.twoFactor.recoveryDescription'),
      toggleText: t('auth.twoFactor.toggleAuth'),
    }
  }

  return {
    title: t('auth.twoFactor.authCodeTitle'),
    description: t('auth.twoFactor.authCodeDescription'),
    toggleText: t('auth.twoFactor.toggleRecovery'),
  }
})


const toggleRecoveryMode = () => {
  showRecoveryInput.value = !showRecoveryInput.value
  error.value = ''
  code.value = ''
  recoveryCode.value = ''
}

const submit = async () => {
  error.value = ''
  processing.value = true

  try {
    if (!authStore.twoFactorToken) {
      throw new Error('Missing two-factor token')
    }

    const payload = showRecoveryInput.value
        ? {
          token: authStore.twoFactorToken,
          recovery_code: recoveryCode.value,
        }
        : {
          token: authStore.twoFactorToken,
          code: code.value,
        }

    const res = await verifyTwoFactor(payload)

    // Backend sets session cookie here ✅
    authStore.clearTwoFactor()
    authStore.setUser(res.user)

    await router.push(ROUTES.HOME)
  } catch (e: any) {
    error.value =
        e?.response?.data?.error ||
        'Invalid authentication code'
    code.value = ''
    recoveryCode.value = ''
  } finally {
    processing.value = false
  }
}
</script>

<template>
  <AuthLayout
      :title="authConfigContent.title"
      :description="authConfigContent.description"
  >
    <div class="space-y-6">
      <!-- OTP MODE -->
      <template v-if="!showRecoveryInput">
        <div class="space-y-4">
          <div
              class="flex flex-col items-center justify-center space-y-3 text-center"
          >
            <InputOTP
                v-model="code"
                :maxlength="6"
                :disabled="processing"
                autofocus
            >
              <InputOTPGroup>
                <InputOTPSlot
                    v-for="index in 6"
                    :key="index"
                    :index="index - 1"
                />
              </InputOTPGroup>
            </InputOTP>

            <InputError :message="error" />
          </div>

          <Button
              class="w-full"
              :disabled="processing || code.length !== 6"
              @click="submit"
          >
            {{ t('auth.twoFactor.continue') }}
          </Button>

          <div class="text-center text-sm text-muted-foreground">
            <span>or you can </span>
            <button
                type="button"
                class="text-foreground underline underline-offset-4"
                @click="toggleRecoveryMode"
            >
              {{ authConfigContent.toggleText }}
            </button>
          </div>
        </div>
      </template>

      <!-- RECOVERY MODE -->
      <template v-else>
        <div class="space-y-4">
          <Input
              v-model="recoveryCode"
              type="text"
              :placeholder="t('auth.twoFactor.recoveryPlaceholder')"
              :disabled="processing"
              autofocus
              required
          />

          <InputError :message="error" />

          <Button
              class="w-full"
              :disabled="processing || !recoveryCode"
              @click="submit"
          >
            {{ t('auth.twoFactor.continue') }}
          </Button>

          <div class="text-center text-sm text-muted-foreground">
            <span>or you can </span>
            <button
                type="button"
                class="text-foreground underline underline-offset-4"
                @click="toggleRecoveryMode"
            >
              {{ authConfigContent.toggleText }}
            </button>
          </div>
        </div>
      </template>
    </div>
  </AuthLayout>
</template>
