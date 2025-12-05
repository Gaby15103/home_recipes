import { defineStore } from "pinia";
import { login, getCurrentUser } from "@/api/auth";

export const useAuthStore = defineStore("auth", {
    state: () => ({
        user: null as null | { id: number; email: string },
        loading: false,
    }),

    actions: {
        async loadUser() {
            try {
                this.user = await getCurrentUser();
            } catch {
                this.user = null;
            }
        },

        async login(email: string, password: string) {
            this.loading = true;
            try {
                const user = await login(email, password);
                this.user = user;
            } finally {
                this.loading = false;
            }
        },
    },
});
