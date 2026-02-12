import {defineStore} from "pinia";
import {getCurrentUser, login, logout} from "@/api/auth";
import type {User} from "@/models/User";
import {useRouter} from "vue-router";
import {ROUTES} from "@/router/routes.ts";

const router = useRouter()

export const useAuthStore = defineStore("auth", {
    state: () => ({
        user: null as User | null,
        loading: false,
        twoFactorPending: false,
        twoFactorToken: null as string | null,
    }),

    getters: {
        isAuthenticated: (state) => !!state.user,
        hasRole: (state) => (role: string) =>
            state.user?.roles?.some(r => r.name === role) ?? false,
    },

    actions: {
        async loadUser(): Promise<User | null> {
            this.loading = true;
            try {
                this.user = await getCurrentUser();
            } catch {
                this.user = null;
            } finally {
                this.loading = false;
            }
            return this.user;
        },

        async login(email: string, password: string) {
            this.loading = true;
            try {
                const res = await login(email, password);
                if (res.user)
                    this.user = res.user;
            } finally {
                this.loading = false;
            }
        },

        async logout() {
            this.loading = true;
            try {
                await logout();
                await router.push(ROUTES.HOME);
            } finally {
                this.user = null;
                this.loading = false;
            }
        },

        setUser(user: User) {
            this.user = user;
        },

        clearUser() {
            this.user = null;
        },

        setPendingTwoFactor(token: string) {
            this.twoFactorPending = true
            this.twoFactorToken = token
        },

        clearTwoFactor() {
            this.twoFactorPending = false
            this.twoFactorToken = null
        },
    },
});
