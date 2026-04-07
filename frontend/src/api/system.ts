import { api } from "./index"; // Adjust based on your actual api client location
import { SystemRoutes } from "@/api/routes.ts";

export interface HealthStatus {
    status: "healthy" | "unhealthy";
    database: "up" | "down";
    version: string;
}


export async function getHealth(): Promise<HealthStatus> {
    try {
        // Calling the /api/health endpoint
        return await api<HealthStatus>(SystemRoutes.health(), {
            method: "GET",
        });
    } catch (error) {
        // Fallback if the server is completely unreachable
        return {
            status: "unhealthy",
            database: "down",
            version: "unknown",
        };
    }
}