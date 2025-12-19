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
    roles: Role[]
    token?: string;
}
export interface Role {
    name: string;
    description: string;
}