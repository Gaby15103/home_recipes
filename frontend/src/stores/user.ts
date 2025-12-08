import { defineStore } from "pinia";
import type { User } from "@/models/User";

export const useUserStore = defineStore("user", {
    state: () => ({
        user: JSON.parse(localStorage.getItem("user")!) as User | null,
        token: localStorage.getItem("token") as string | null,
    }),
    actions: {
        setUser(userData: User, jwt: string) {
            this.user = userData;
            this.token = jwt;

            localStorage.setItem("user", JSON.stringify(userData));
            localStorage.setItem("token", jwt);
        },
        logout() {
            this.user = null;
            this.token = null;

            localStorage.removeItem("user");
            localStorage.removeItem("token");
        },
    },
});
