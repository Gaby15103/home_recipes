import { defineStore } from "pinia"
import type { User } from "@/models/User"

export const useUserStore = defineStore("user", {
    state: () => ({
        user: null as User | null,
        loading: false,
    }),

    getters: {
        isAuthenticated: (state) => !!state.user,

        hasRole: (state) => {
            return (role: string) =>
                state.user?.roles?.some(r => r.name === role) ?? false
        },
    },

    actions: {
        setUser(user: User) {
            this.user = user
        },

        clearUser() {
            this.user = null
        },
    },
})
