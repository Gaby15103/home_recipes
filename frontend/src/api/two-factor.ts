import { api } from './client';
import { TwoFactorRoutes } from './routes';
import type {User} from "@/models/User.ts";

export interface QrCodeResponse {
    svg: string;
    url: string;
}

export interface SecretKeyResponse {
    secret_key: string;
}

export interface RecoveryCodesResponse {
    codes: string[];
}

export interface TwoFactorStatusResponse {
    enabled: boolean;
    requires_confirmation: boolean;
}

export interface VerifyTwoFactorRequest {
    token: string
    code?: string
    recovery_code?: string
}
export interface VerifyTwoFactorResponse {
    user: User
}



export async function getQrCode(): Promise<QrCodeResponse> {
    return api<QrCodeResponse>(TwoFactorRoutes.qrCode(), {
        method: 'GET',
    });
}

export async function getSecretKey(): Promise<SecretKeyResponse> {
    return api<SecretKeyResponse>(TwoFactorRoutes.secretKey(), {
        method: 'GET',
    });
}

export async function getRecoveryCodes(): Promise<RecoveryCodesResponse> {
    return api<RecoveryCodesResponse>(TwoFactorRoutes.recoveryCodes(), {
        method: 'GET',
    });
}

export async function enableTwoFactor(): Promise<void> {
    return api<void>(TwoFactorRoutes.enable(), {
        method: 'POST',
    });
}

export async function disableTwoFactor(): Promise<void> {
    return api<void>(TwoFactorRoutes.disable(), {
        method: 'POST',
    });
}

// api/twoFactor.ts
export async function getTwoFactorStatus(): Promise<TwoFactorStatusResponse> {
    return api<TwoFactorStatusResponse>(TwoFactorRoutes.status(), {
        method: 'GET',
    });
}
export async function verifyTwoFactor(payload: VerifyTwoFactorRequest): Promise<VerifyTwoFactorResponse> {
    return api<VerifyTwoFactorResponse>(TwoFactorRoutes.verify(), {
        method: 'POST',
        data: payload,
    });
}
