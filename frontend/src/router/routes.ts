export const ROUTES = {
    // Public
    HOME: "/",
    LOGIN: "/login",
    REGISTER: "/register",
    FORGOT_PASSWORD: "/forgot_password",
    RESET_PASSWORD: "/reset-password",
    EMAIL_CONFIRMATION: "/verify",
    RECIPES: "/recipes",
    RECIPE: (id: string ) => `/recipe/${id}`,

    USER: {
        SETTINGS: "/user/settings",
        MY_RECIPES: "/user/my_recipes",
        PROFILE: (id: string) =>`/user/profile/${id}`,
        SECURITY:"/user/security",
    },

    // Admin
    ADMIN: {
        BASE: "/admin",
        DASHBOARD: "/admin/dashboard",
        RECIPE: {
            BASE: "/admin/recipe",
            LIST: "/admin/recipe/list",
            CREATE: "/admin/recipe/create",
            VIEW: (id: string ) =>  `/admin/recipe/${id}`,
            EDIT: (id: string ) => `/admin/recipe/edit/${id}`,
            CATEGORIES: "/admin/recipe/categories",
            OCR_REVIEW: "/admin/recipe/ocr-review",
        },
        USER: {
            BASE: "/admin/user",
            PROFILE: "/admin/user/edit/profile",
            PASSWORD: "/admin/user/edit/password",
            TWO_FACTOR: "/admin/user/edit/two-factor",
            APPEARANCE: "/admin/user/edit/appearance",
        },
    },
    TWO_FACTOR: "/two_factor",
    GITHUB_REPO: "https://github.com/Gaby15103/home_recipes"
};
