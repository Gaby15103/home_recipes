import axios, {AxiosHeaders, type AxiosRequestConfig} from "axios";

export const API_URL = import.meta.env.VITE_API_URL;

const apiClient = axios.create({
    baseURL: API_URL,
    withCredentials: true,
    headers: { "Content-Type": "application/json" },
});

apiClient.interceptors.request.use((config) => {
    const lang = navigator.language || "en";

    if (!config.headers) {
        config.headers = new AxiosHeaders();
    }

    config.headers.set("Accept-Language", lang);

    return config;
});


export async function api<T>(url: string, options: AxiosRequestConfig = {}): Promise<T> {
    try {
        const response = await apiClient.request<T>({
            url,
            ...options,
            headers: {
                // Keep the default Content-Type from apiClient
                ...apiClient.defaults.headers.common,
                // Spread the method-specific defaults if they exist
                ...(apiClient.defaults.headers[options.method?.toLowerCase() || 'get'] as any),
                // Spread any headers passed in the options to allow overriding
                ...options.headers,
            },
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
