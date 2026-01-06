import {createApp} from "vue";
import App from "./App.vue";
import "./style.css";

import {router} from "./router";
import {createPinia} from "pinia";
import {useUserStore} from "@/stores/user.ts";
import {getCurrentUser} from "@/api/auth.ts";

const app = createApp(App);

app.config.globalProperties.$apiUrl = import.meta.env.VITE_STATIC_URL;

app.use(router);
app.use(createPinia());

app.mount("#app");

const userStore = useUserStore();

try {
    const res = await getCurrentUser();
    userStore.setUser(res.user)
} catch {
    userStore.clearUser();
}
router.beforeEach((to, from, next) => {
    const userStore = useUserStore()

    // Not logged in
    if (to.meta.requiresAuth && !userStore.isAuthenticated) {
        return next({path: "/login"})
    }

    // Role-based protection
    if (to.meta.roles) {
        const allowed = (to.meta.roles as string[]).some(role =>
            userStore.hasRole(role)
        )

        if (!allowed) {
            return next({path: "/403"}) // forbidden page
        }
    }

    next()
})