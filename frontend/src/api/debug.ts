import { api } from "./client";
import { DebugRoutes } from "./routes";

export function debugMultipart(data: FormData) {
    return api(DebugRoutes.multipart(), {
        method: "POST",
        data,
        headers: { "Content-Type": "multipart/form-data" },
    });
}
