import i18n from "@/../i18n";
import { setLangCookie } from "@/utils/lang";

export function setLanguage(lang: string) {
    i18n.global.locale.value = lang;
    setLangCookie(lang);
}
