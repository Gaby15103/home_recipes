import { api } from "./client";
import { AuthRoutes, UserRoutes } from "./routes";
import type {LoginRequest, LoginResponse, RegisterRequest} from "@/models/Auth";
import type { User } from "@/models/User";

// -------- AUTH --------
export function login(email: string, password: string) {
    const payload: LoginRequest = { user: { email, password } };
    return api<LoginResponse>(AuthRoutes.login(), { method: "POST", data: payload });
}

export function registerUser(
    username: string, email: string, password: string, first_name: string, last_name: string
) {
    const payload: RegisterRequest = { user: { username, email, password, first_name, last_name } };
    return api(AuthRoutes.register(), { method: "POST", data: payload });
}
export function confirmEmail(token: string) {
    return api<{ success: boolean; message: string }>(
        AuthRoutes.confirmEmail() + `?token=${encodeURIComponent(token)}`,
        {
            method: "POST",
        }
    );
}


export function logout() {
    return api(AuthRoutes.logout(), { method: "POST" });
}

// -------- USER --------
export function getCurrentUser() {
    return api<{ user: User }>(UserRoutes.me(), { method: "GET" });
}

export function updateCurrentUser(user: Partial<User>) {
    return api<{ user: User }>(UserRoutes.updateMe(), { method: "PUT", data: user });
}

export function editUser(user: User, password: string | null = null) {
    const payload = { user: { ...user, password } };
    return api<{ user: User }>(UserRoutes.updateMe(), { method: "PATCH", data: payload });
}

export function deleteUser(password: string) {
    return api(UserRoutes.me(), { method: "DELETE", data: { password } });
}

export function editPassword(current_password: string, password: string, password_confirmation: string) {
    return api(UserRoutes.updateMe(), {
        method: "PATCH",
        data: { current_password, password, password_confirmation },
    });
}
