import {defineStore} from "pinia";
import {getCurrentUser, login, logout} from "@/api/auth";
import type {User} from "@/models/User";

export const useAuthStore = defineStore("auth", {
    state: () => ({
        user: null as User | null,
        loading: false,
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
                let res = await getCurrentUser()
                this.user = res.user;
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
                this.user = res.user;
            } finally {
                this.loading = false;
            }
        },

        async logout() {
            this.loading = true;
            try {
                await logout();
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
    },
});
