import { api } from "./client";
import type { LoginRequest, LoginResponse, RegisterRequest } from "@/models/Auth";

export function login(email: string, password: string) {
    const payload: LoginRequest = {
        user: { email, password },
    };

    return api<LoginResponse>("/users/login", {
        method: "POST",
        data: payload, // Axios uses `data` instead of `body`
    });
}

export function registerUser(
    username: string,
    email: string,
    password: string,
    first_name: string,
    last_name: string
) {
    const payload: RegisterRequest = {
        user: { username, email, password, first_name, last_name },
    };

    return api<LoginResponse>("/users", {
        method: "POST",
        data: payload,
    });
}

export function getCurrentUser() {
    return api<LoginResponse>("/user", {
        method: "GET",
    });
}
