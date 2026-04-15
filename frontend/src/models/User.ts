import type {BasicColorMode} from "@vueuse/core";

export interface UserPreferences {
    language: 'en' | 'fr';
    theme: BasicColorMode;
    recipe_favorite_enabled: boolean;
    recipe_comment_enabled: boolean;
    comment_reply_enabled: boolean;
}

export interface User {
    id: string;
    email: string;
    username: string;
    first_name: string;
    last_name: string;
    avatar_url: string;
    preferences: UserPreferences;
    email_verified: boolean;
    last_login: Date;
    created_at: Date;
    updated_at: Date;
    roles: Role[]
}
export interface Role {
    name: string;
    description: string;
}
export interface ProfileDto {
    id: string;
    username: string;
    first_name: string;
    last_name: string;
    avatar_url: string | File | null;
    preferences: UserPreferences;
}