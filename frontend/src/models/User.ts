export interface UserPreferences {
    [key: string]: any;
}

export interface User {
    email: string;
    username: string;
    first_name: string;
    last_name: string;
    avatar_url: string | null;
    preferences: UserPreferences;
    token?: string; // optional — only returned after login
}
