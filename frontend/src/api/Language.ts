import {api} from "@/api/client.ts";
import {LanguageRoutes} from "@/api/routes.ts";
import type {Language} from "@/models/Language.ts";


export function getAllLanguage() {
    return api<Language[]>(LanguageRoutes.list(), {method: "GET"});
}