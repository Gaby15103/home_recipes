import { api } from "./client";
import { NotificationRoutes } from "./routes";
import type {NotificationListResponse} from "@/models/Notification.ts";
export const API_HOST = import.meta.env.VITE_API_HOST;
/**
 * Fetch the user's notification inbox
 */
export function getNotifications() {
    return api<NotificationListResponse>(NotificationRoutes.list(), { method: "GET" });
}

/**
 * Mark a specific notification as read
 */
export function markNotificationAsRead(id: string) {
    return api<void>(NotificationRoutes.markRead(id), { method: "POST" });
}

/**
 * Mark all notifications as read for the current user
 */
export function markAllNotificationsAsRead() {
    return api<void>(NotificationRoutes.markAllRead(), { method: "POST" });
}

/**
 * Admin: Create a new notification template
 */
export function createNotificationTemplate(template: any) {
    return api(NotificationRoutes.createTemplate(), {
        method: "POST",
        data: template
    });
}

/**
 * Helper to get the absolute WebSocket URL
 */
export const getNotificationWsUrl = () => {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const return_type = `${protocol}//${API_HOST}${NotificationRoutes.ws()}`
    console.log(return_type)
    return return_type;
};