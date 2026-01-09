import {api} from "@/api/client.ts";
import type {Tag} from "@/models/Recipe.ts";


export function getAllTags(){
    return api<Tag[]>("/tag/get_all")
}