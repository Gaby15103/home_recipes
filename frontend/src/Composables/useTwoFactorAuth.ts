// composables/useTwoFactorAuth.ts
import { ref, computed } from 'vue'
import {
    getQrCode,
    getSecretKey,
    getRecoveryCodes,
    enableTwoFactor,
    disableTwoFactor,
} from '@/api'

const errors = ref<string[]>([])
const manualSetupKey = ref<string | null>(null)
const qrCodeSvg = ref<string | null>(null)
const recoveryCodesList = ref<string[]>([])

const hasSetupData = computed(() => qrCodeSvg.value !== null && manualSetupKey.value !== null)

export const useTwoFactorAuth = () => {
    const fetchQrCode = async (): Promise<void> => {
        try {
            const { svg } = await getQrCode()
            qrCodeSvg.value = svg
        } catch {
            errors.value.push('Failed to fetch QR code')
            qrCodeSvg.value = null
        }
    }

    const fetchSetupKey = async (): Promise<void> => {
        try {
            const secretKey  = await getSecretKey()
            manualSetupKey.value = secretKey.secret_key
        } catch {
            errors.value.push('Failed to fetch setup key')
            manualSetupKey.value = null
        }
    }

    const fetchRecoveryCodes = async (): Promise<void> => {
        try {
            const { codes } = await getRecoveryCodes()
            recoveryCodesList.value = codes
        } catch {
            errors.value.push('Failed to fetch recovery codes')
            recoveryCodesList.value = []
        }
    }

    const fetchSetupData = async (): Promise<void> => {
        clearSetupData()
        clearErrors()

        await fetchSetupKey()
        await fetchQrCode()
    }

    const clearSetupData = (): void => {
        qrCodeSvg.value = null
        manualSetupKey.value = null
    }

    const clearErrors = (): void => {
        errors.value = []
    }

    const clearTwoFactorAuthData = (): void => {
        clearSetupData()
        clearErrors()
        recoveryCodesList.value = []
    }

    const enable = async (): Promise<void> => {
        try {
            await enableTwoFactor()
            await fetchSetupData()
        } catch {
            errors.value.push('Failed to enable 2FA')
        }
    }

    const disable = async (): Promise<void> => {
        try {
            await disableTwoFactor()
            clearTwoFactorAuthData()
        } catch {
            errors.value.push('Failed to disable 2FA')
        }
    }

    return {
        qrCodeSvg,
        manualSetupKey,
        recoveryCodesList,
        errors,
        hasSetupData,
        fetchQrCode,
        fetchSetupKey,
        fetchSetupData,
        fetchRecoveryCodes,
        enable,
        disable,
        clearSetupData,
        clearErrors,
        clearTwoFactorAuthData,
    }
}
