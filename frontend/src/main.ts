import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";

import router from "./router";
import { createPinia } from "pinia";
import { useAuthStore } from "@/stores/auth";
import { createI18n } from 'vue-i18n'
import en from '@/i18n/en.json';
import fr from '@/i18n/fr.json';

const app = createApp(App);

function loadLocaleMessages(){
    const locales = [{en: en},{fr: fr}];
    const messages = {};
    locales.forEach(lang => {
        const key = Object.keys(lang)
        messages[key] = lang[key]
    });
    return messages;
}

const i18n = createI18n({
    locale: "fr",
    fallbackLocale: "en",
    messages: loadLocaleMessages(),
})

app.config.globalProperties.$apiUrl = import.meta.env.VITE_STATIC_URL;

app.use(router);
app.use(createPinia());

const authStore = useAuthStore();

// load current user BEFORE mounting
await authStore.loadUser();
console.log(authStore.user)

app.use(i18n);
app.mount("#app");