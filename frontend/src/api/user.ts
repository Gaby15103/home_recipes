import {api} from "./client";
import {UserRoutes} from "./routes";
import type {ProfileDto, User} from "@/models/User";
import {uploadSingleFile} from "@/api/upload.ts";


export async function getUserById(id: string) {
    return api<User>(UserRoutes.getById(id), { method: "GET" });
}
export async function updateProfile(profile: ProfileDto) {
    if (profile.avatar_url instanceof File){
        let res = await uploadSingleFile(profile.avatar_url)
        profile.avatar_url = res.temp_id
    }
    return api<User>(UserRoutes.update(profile.id), {
        method: "PUT",
        data: profile
    });
}