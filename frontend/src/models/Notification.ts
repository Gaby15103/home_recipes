/**
 * Represents the main Notification object from the backend
 */
export interface Notification {
    id: string;
    user_id: string;
    actor_id?: string;
    actor_name?: string;
    category: NotificationCategory | string;
    title: string;
    message: string;
    target_id?: string;
    is_read: boolean;
    created_at: string;
}

/**
 * Helper enum for standardizing notification types
 * in the frontend (icons, colors, navigation)
 */
// @ts-ignore
export enum NotificationCategory {
    RecipeFavorite = "recipe_favorite",
    RecipeComment = "recipe_comment",
    SystemAlert = "system_alert",
    AdminUpdate = "admin_update"
}

/**
 * Standard response for the notification inbox list
 */
export interface NotificationListResponse {
    items: Notification[];
    unread_count: number;
}

/**
 * Data required to trigger a notification (useful if testing
 * or for admin-initiated alerts)
 */
export interface NotificationTrigger {
    recipient_id: string;
    actor_id?: string;
    category: string;
    target_id?: string;
    variables: Record<string, string>;
}

/**
 * Template structure for Admin management
 */
export interface NotificationTemplate {
    id: string;
    category: string;
    language_code: string;
    title_template: string;
    message_template: string;
}