import {createApp} from "vue";
import App from "./App.vue";
import "./style.css";

import router from "./router";
import {createPinia} from "pinia";
import {useAuthStore} from "@/stores/auth";
import {getCurrentUser} from "@/api/auth.ts";

const app = createApp(App);

app.config.globalProperties.$apiUrl = import.meta.env.VITE_STATIC_URL;

app.use(router);
app.use(createPinia());

app.mount("#app");

const authStore = useAuthStore();

try {
    const res = await getCurrentUser();
    authStore.setUser(res.user)
} catch {
    authStore.clearUser();
}