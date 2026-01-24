export interface UserPreferences {
    [key: string]: any;
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
    roles: Role[]
    token?: string;
}
export interface Role {
    name: string;
    description: string;
}