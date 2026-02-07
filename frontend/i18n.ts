import { createI18n } from "vue-i18n"

import en from "@/locales/en.json"
import fr from "@/locales/fr.json"
import { getLangCookie } from "@/utils/lang";

const cookieLang = getLangCookie();
export default createI18n({
    legacy: false,
    locale: cookieLang || "fr",
    fallbackLocale: "en",
    messages: {
        en,
        fr
    }
})
