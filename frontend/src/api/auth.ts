import { api } from "./client";
import type {EditUser, LoginRequest, LoginResponse, RegisterRequest} from "@/models/Auth";
import type {User} from "@/models/User.ts";

export function login(email: string, password: string) {
    const payload: LoginRequest = {
        user: { email, password },
    };

    return api<LoginResponse>("/user/login", {
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

    return api<LoginResponse>("/user/register", {
        method: "POST",
        data: payload,
    });
}

export function getCurrentUser() {
    return api<LoginResponse>("/user", {
        method: "GET",
    });
}

export function logout() {
    return api("/user/logout", {
        method: "POST"
    });
}
export function editUser(user: User, password: string|null = null){
    const payload: EditUser = {
        user: {
            username: user.username,
            email: user.email,
            password: password,
            first_name: user.first_name,
            last_name: user.last_name,
            avatar_url: user.avatar_url,
            preferences: user.preferences,
        }
    }
    return api("/user/edit",{
        method: "PATCH",
        data: payload,
    })
}
export function editPassword(
    current_password: string,
    password: string,
    password_confirmation: string
) {
    return api("/user/password", {
        method: "PATCH",
        data: {
            current_password,
            password,
            password_confirmation,
        },
    });
}

export function deleteUser(password: string) {
    return api("/user", {
        method: "DELETE",
        data: { password },
    })
}
