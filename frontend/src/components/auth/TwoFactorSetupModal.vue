<script setup lang="ts">
import { ref, computed, nextTick, watch, onUnmounted } from 'vue';
import { useTwoFactorAuth } from '@/composables/useTwoFactorAuth.ts';
import { Button } from '@/components/ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from '@/components/ui/dialog';
import { useClipboard } from '@vueuse/core';
import { Check, Copy } from 'lucide-vue-next';
import QRCode from 'qrcode'

interface Props {
  requiresConfirmation: boolean;
  twoFactorEnabled: boolean;
}

const props = defineProps<Props>();
const isOpen = defineModel<boolean>('isOpen');

const { qrCode, manualSetupKey, fetchSetupData, clearSetupData, errors } = useTwoFactorAuth();
const showVerificationStep = ref(false);
const code = ref('');
const pinInputContainerRef = ref<HTMLElement | null>(null);
const qrSvg = ref<string | null>(null)

const { copy, copied } = useClipboard();

// Modal config
const modalConfig = computed(() => {
  if (props.twoFactorEnabled) {
    return {
      title: 'Two-Factor Authentication Enabled',
      description: 'Two-factor authentication is enabled. Scan the QR code or use the setup key.',
      buttonText: 'Close',
    };
  }
  if (showVerificationStep.value) {
    return {
      title: 'Verify Authentication Code',
      description: 'Enter the 6-digit code from your authenticator services',
      buttonText: 'Continue',
    };
  }
  return {
    title: 'Enable Two-Factor Authentication',
    description: 'Scan the QR code or enter the setup key in your authenticator services',
    buttonText: 'Continue',
  };
});

watch(qrCode, async (qrCode) => {
  if (!qrCode) {
    qrSvg.value = null
    return
  }

  qrSvg.value = await QRCode.toString(qrCode.url, {
    type: 'svg',
    margin: 1,
    width: 256,
  })
})



// Next step / verification
const handleModalNextStep = () => {
  if (props.requiresConfirmation) {
    showVerificationStep.value = true;
    nextTick(() => {
      pinInputContainerRef.value?.querySelector('input')?.focus();
    });
    return;
  }
  clearSetupData();
  isOpen.value = false;
};

const resetModalState = () => {
  if (props.twoFactorEnabled) clearSetupData();
  showVerificationStep.value = false;
  code.value = '';
};

watch(isOpen, async (val) => {
  if (!val) return resetModalState();
  if (!qrCode.value) await fetchSetupData();
});

onUnmounted(() => clearSetupData());
</script>

<template>
  <Dialog :open="isOpen" @update:open="isOpen = $event">
    <DialogContent class="sm:max-w-md">
      <DialogHeader>
        <DialogTitle>{{ modalConfig.title }}</DialogTitle>
        <DialogDescription class="text-center">{{ modalConfig.description }}</DialogDescription>
      </DialogHeader>

      <div v-if="!showVerificationStep" class="flex flex-col items-center space-y-5">
        <div v-if="!qrSvg" class="flex items-center justify-center h-64 w-64 bg-muted">
          Loading QR...
        </div>
        <div v-else v-html="qrSvg" class="flex aspect-square items-center justify-center"></div>

        <div class="flex w-full items-center space-x-2">
          <input type="text" readonly :value="manualSetupKey" class="flex-1 p-2 border rounded" />
          <Button @click="copy(manualSetupKey || '')" variant="outline">
            <Check v-if="copied" /> <Copy v-else />
          </Button>
        </div>

        <Button class="w-full" @click="handleModalNextStep">{{ modalConfig.buttonText }}</Button>
      </div>

      <div v-else>
        <!-- verification code input -->
        <div ref="pinInputContainerRef">
          <input v-model="code" maxlength="6" placeholder="Enter 6-digit code" class="border p-2 rounded w-full"/>
        </div>
        <Button @click="handleModalNextStep" :disabled="code.length < 6">Confirm</Button>
      </div>
    </DialogContent>
  </Dialog>
</template>
