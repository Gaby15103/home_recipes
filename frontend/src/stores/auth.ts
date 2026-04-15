import { defineStore } from "pinia";
import { ref, computed } from "vue";
import {getCurrentUser, login as apiLogin, logout as apiLogout, updateCurrentUser} from "@/api/auth";
import type {User, UserPreferences} from "@/models/User";
import { useRouter } from "vue-router";
import { ROUTES } from "@/router/routes.ts";
import { setLanguage } from "@/utils/setLanguage.ts";

import { useColorMode } from "@vueuse/core";
import i18n from "../../i18n.ts";
import {api} from "@/api";

export const useAuthStore = defineStore("auth", () => {
    // --- Setup Logic ---
    const router = useRouter();
    const mode = useColorMode();

    // --- State ---
    const user = ref<User | null>(null);
    const loading = ref(false);
    const twoFactorPending = ref(false);
    const twoFactorToken = ref<string | null>(null);

    // --- Getters ---
    const isAuthenticated = computed(() => !!user.value);
    const hasRole = computed(() => (role: string) =>
        user.value?.roles?.some(r => r.name === role) ?? false
    );

    // --- Actions ---
    async function loadUser(): Promise<User | null> {
        loading.value = true;
        try {
            user.value = await getCurrentUser();
        } catch {
            user.value = null;
        } finally {
            loading.value = false;
        }
        return user.value;
    }

    async function login(email: string, password: string) {
        loading.value = true;
        try {
            const res = await apiLogin(email, password);
            if (res.user) {
                user.value = res.user;

                // Sync Preferences
                const prefs = res.user.preferences;
                if (prefs) {
                    // Update Language
                    if (i18n.global.availableLocales.includes(prefs.language as any)) {
                        setLanguage(prefs.language);
                    }
                    // Update Theme
                    mode.value = prefs.theme as any;
                }
            }
        } finally {
            loading.value = false;
        }
    }

    async function logout() {
        loading.value = true;
        try {
            await apiLogout();
            await router.push(ROUTES.HOME);
        } finally {
            user.value = null;
            loading.value = false;
        }
    }

    function setUser(newUser: User) {
        user.value = newUser;
    }

    function clearUser() {
        user.value = null;
    }
    function  setPendingTwoFactor(token: string) {
        twoFactorPending.value = true
        twoFactorToken.value = token
    }

    function  clearTwoFactor() {
        twoFactorPending.value = false
        twoFactorToken.value = null
    }

    async function updatePreference(preferences: UserPreferences) {
        if (!user.value) return;
        const oldPreferences = user.value.preferences
        user.value.preferences = preferences;

        try {
            await updateCurrentUser(user.value);
        } catch (error) {
            // 4. Rollback on failure
            if (user.value) {
                user.value.preferences = oldPreferences;
            }
            console.error("Failed to sync preference", error);
        }
    }

    // --- Return everything the app needs ---
    return {
        user, loading, setPendingTwoFactor, clearTwoFactor,
        twoFactorPending,twoFactorToken,
        isAuthenticated, hasRole,
        loadUser, login, logout, setUser, clearUser,
        updatePreference
    };
});