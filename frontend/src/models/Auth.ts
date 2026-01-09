import type {User} from "@/models/User.ts";

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

export type RegisterResponse = LoginResponse;
