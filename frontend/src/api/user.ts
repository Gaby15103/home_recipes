import { api } from "./client";
import { AuthRoutes, UserRoutes } from "./routes";
import type {LoginRequest, LoginResponse, RegisterRequest} from "@/models/Auth";
import type { User } from "@/models/User";



export function getUserById(id: string) {
    return api<User>(UserRoutes.getById(id), { method: "GET" });
}