// src/api/apiHelpers.ts
export function toQueryParams(obj: Record<string, any>): string {
    const params = new URLSearchParams();
    for (const key in obj) {
        if (obj[key] != null) params.append(key, obj[key]);
    }
    return params.toString();
}

export function formDataFromObject(obj: Record<string, any>): FormData {
    const fd = new FormData();
    for (const key in obj) {
        const value = obj[key];
        if (value !== undefined && value !== null) {
            if (value instanceof File) {
                fd.append(key, value);
            } else {
                fd.append(key, JSON.stringify(value));
            }
        }
    }
    return fd;
}
