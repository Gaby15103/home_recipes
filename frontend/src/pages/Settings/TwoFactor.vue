<script setup lang="ts">
import HeadingSmall from '@/components/HeadingSmall.vue';
import TwoFactorSetupModal from '@/components/TwoFactorSetupModal.vue';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { useTwoFactorAuth } from '@/composables/useTwoFactorAuth';
import SettingsLayout from '@/components/layout/settings/Layout.vue';
import { ShieldBan, ShieldCheck } from 'lucide-vue-next';
import {ref, onUnmounted, onMounted} from 'vue';
import {enableTwoFactor, disableTwoFactor, getTwoFactorStatus} from '@/api';

const twoFactorEnabled = ref(false)
const requiresConfirmation = ref(false)
const statusLoading = ref(true)


// Local state
const showSetupModal = ref(false);
const loading = ref(false);
const errors = ref<string[]>([]);

// 2FA composable
const { hasSetupData, clearTwoFactorAuthData } = useTwoFactorAuth();

onUnmounted(() => {
  clearTwoFactorAuthData();
});

// Enable / Disable 2FA
const handleEnable = async () => {
  loading.value = true
  errors.value = []

  try {
    await enableTwoFactor()
    twoFactorEnabled.value = true
    showSetupModal.value = true
  } catch (e: any) {
    errors.value.push(e.message || 'Failed to enable 2FA')
  } finally {
    loading.value = false
  }
}

const handleDisable = async () => {
  loading.value = true
  errors.value = []

  try {
    await disableTwoFactor()
    twoFactorEnabled.value = false
  } catch (e: any) {
    errors.value.push(e.message || 'Failed to disable 2FA')
  } finally {
    loading.value = false
  }
}

onMounted(async () => {
  try {
    const status = await getTwoFactorStatus()
    twoFactorEnabled.value = status.enabled
    requiresConfirmation.value = status.requires_confirmation
  } catch (e) {
    errors.value.push('Failed to load 2FA status')
  } finally {
    statusLoading.value = false
  }
})
</script>

<template>
    <SettingsLayout>
      <div class="space-y-6">
        <HeadingSmall
            title="Two-Factor Authentication"
            description="Manage your two-factor authentication settings"
        />

        <!-- Disabled -->
        <div v-if="!twoFactorEnabled" class="flex flex-col space-y-4">
          <Badge variant="destructive">Disabled</Badge>
          <p class="text-muted-foreground">
            When you enable two-factor authentication, you will be prompted for a secure pin during login.
          </p>

          <div class="flex flex-col gap-2">
            <Button v-if="hasSetupData" @click="showSetupModal = true">
              <ShieldCheck /> Continue Setup
            </Button>
            <Button v-else @click="handleEnable" :disabled="loading">
              <ShieldCheck /> {{ loading ? 'Enabling...' : 'Enable 2FA' }}
            </Button>
          </div>

          <div v-if="errors.length" class="text-destructive mt-2">
            <ul>
              <li v-for="(err, i) in errors" :key="i">{{ err }}</li>
            </ul>
          </div>
        </div>

        <!-- Enabled -->
        <div v-else class="flex flex-col space-y-4">
          <Badge variant="default">Enabled</Badge>
          <p class="text-muted-foreground">
            With two-factor authentication enabled, you will be prompted for a secure, random pin during login.
          </p>

          <div class="relative inline">
            <Button variant="destructive" @click="handleDisable" :disabled="loading">
              <ShieldBan /> {{ loading ? 'Disabling...' : 'Disable 2FA' }}
            </Button>
          </div>

          <div v-if="errors.length" class="text-destructive mt-2">
            <ul>
              <li v-for="(err, i) in errors" :key="i">{{ err }}</li>
            </ul>
          </div>
        </div>

        <!-- Modal -->
        <TwoFactorSetupModal
            v-model:isOpen="showSetupModal"
            :requiresConfirmation="requiresConfirmation"
            :twoFactorEnabled="twoFactorEnabled"
        />
      </div>
    </SettingsLayout>
</template>
