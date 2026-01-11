import type {User, UserPreferences} from "@/models/User.ts";

export interface LoginRequest {
    user: {
        email: string;
        password: string;
    };
}

export interface LoginResponse {
    user: User;
}

export interface RegisterRequest {
    user: {
        email: string;
        password: string;
        username: string;
        first_name: string;
        last_name: string;
    };
}
export interface EditUser {
    user: {
        username: string;
        email: string;
        password: string|null;
        first_name: string;
        last_name: string;
        avatar_url: string;
        preferences: UserPreferences;
    }
}

export type RegisterResponse = LoginResponse;
