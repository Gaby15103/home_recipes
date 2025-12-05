import axios from "axios";
import type { AxiosRequestConfig } from "axios";

export const API_URL = import.meta.env.VITE_API_URL;

const apiClient = axios.create({
    baseURL: API_URL,
    withCredentials: true,
    headers: {
        "Content-Type": "application/json",
    },
});

export async function api<T>(url: string, options: AxiosRequestConfig = {}): Promise<T> {
    try {
        const response = await apiClient.request<T>({
            url,
            ...options,
        });

        return response.data;
    } catch (error: any) {
        let msg = "Unknown error";

        if (error.response) {
            const data = error.response.data;
            msg = data?.error || data?.message || JSON.stringify(data);
        } else if (error.request) {
            msg = "No response from server";
        } else {
            msg = error.message;
        }

        throw new Error(msg);
    }
}
