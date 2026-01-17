export const ROUTES = {
    // Public
    HOME: "/",
    LOGIN: "/login",
    REGISTER: "/register",
    EMAIL_CONFIRMATION: "/email_confirmation",
    RECIPES: "/recipes",
    RECIPE: (id: string | number) => `/recipe/${id}`,

    // Admin
    ADMIN: {
        BASE: "/admin",
        DASHBOARD: "/admin/dashboard",
        RECIPE: {
            BASE: "/admin/recipe",
            LIST: "/admin/recipe/list",
            CREATE: "/admin/recipe/create",
            EDIT: (id: string | number) => `/admin/recipe/edit/${id}`,
            CATEGORIES: "/admin/recipe/categories",
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
