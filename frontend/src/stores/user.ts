import { defineStore } from "pinia";
import type { User } from "@/models/User";

export const useUserStore = defineStore("user", {
    state: () => ({
        user: null as User | null,
        token: null as string | null,
    }),
    actions: {
        setUser(userData: User, jwt: string) {
            this.user = userData;
            this.token = jwt;
        },
        logout() {
            this.user = null;
            this.token = null;
        },
    },
});
