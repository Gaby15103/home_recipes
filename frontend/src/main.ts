import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";

import router from "./router";
import { createPinia } from "pinia";
import { useAuthStore } from "@/stores/auth";

const app = createApp(App);

app.config.globalProperties.$apiUrl = import.meta.env.VITE_STATIC_URL;

app.use(router);
app.use(createPinia());

const authStore = useAuthStore();

// load current user BEFORE mounting
await authStore.loadUser();
console.log(authStore.user)

app.mount("#app");