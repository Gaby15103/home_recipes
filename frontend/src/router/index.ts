import {createRouter, createWebHistory} from "vue-router";
import {ROUTES} from "./routes";
import {useAuthStore} from "@/stores/auth.ts";

// Pages
import Home from "@/pages/Home.vue";
import Login from "@/pages/auth/Login.vue";
import Register from "@/pages/auth/Register.vue";
import Recipes from "@/pages/Recipes.vue";
import Recipe from "@/pages/Recipe.vue";

// Admin Pages
import AdminDashBoard from "@/pages/Admin/AdminDashBoard.vue";
import RecipeDashboard from "@/pages/Admin/Recipes/RecipeDashboard.vue";
import List from "@/pages/Admin/Recipes/List.vue";
import Create from "@/pages/Admin/Recipes/Create.vue";
import Edit from "@/pages/Admin/Recipes/Edit.vue";
import Categories from "@/pages/Admin/Recipes/Categories.vue";

// User Settings Pages
import Profile from "@/pages/Settings/Profile.vue";
import Password from "@/pages/Settings/Password.vue";
import TwoFactor from "@/pages/Settings/TwoFactor.vue";
import Appearance from "@/pages/Settings/Appearance.vue";
import TwoFactorChallenge from "@/pages/auth/TwoFactorChallenge.vue";

const routes = [

    {path: ROUTES.HOME, component: Home},
    {path: ROUTES.LOGIN, component: Login},
    {path: ROUTES.TWO_FACTOR, component: TwoFactorChallenge},
    {path: ROUTES.REGISTER, component: Register},
    {path: ROUTES.RECIPES, component: Recipes},
    {path: "/recipe/:id", component: Recipe},

    {
        path: ROUTES.ADMIN.BASE,
        meta: {requiresAuth: true, roles: ["ADMIN", "MODERATOR"], layout: "AdminLayout"},
        redirect: ROUTES.ADMIN.DASHBOARD,
        children: [
            // Dashboard
            {
                path: "dashboard",
                component: AdminDashBoard,
                meta: {
                    breadcrumb: [
                        {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                        {title: "Dashboard"},
                    ],
                },
            },

            // Recipe Management
            {
                path: "recipe",
                component: RecipeDashboard,
                meta: {
                    breadcrumb: [
                        {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                        {title: "Recipe"},
                    ],
                },
            },
            {
                path: "recipe/list",
                component: List,
                meta: {
                    breadcrumb: [
                        {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                        {title: "Recipe", href: ROUTES.ADMIN.RECIPE.BASE},
                        {title: "List"},
                    ],
                },
            },
            {
                path: "recipe/create",
                component: Create,
                meta: {
                    breadcrumb: [
                        {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                        {title: "Recipe", href: ROUTES.ADMIN.RECIPE.BASE},
                        {title: "Create"},
                    ],
                },
            },
            {
                path: "recipe/edit/:id",
                component: Edit,
                meta: {
                    breadcrumb: [
                        {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                        {title: "Recipe", href: ROUTES.ADMIN.RECIPE.BASE},
                        {title: "Edit"},
                    ],
                },
            },
            {
                path: "recipe/categories",
                component: Categories,
                meta: {
                    breadcrumb: [
                        {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                        {title: "Categories"},
                    ],
                },
            },

            // User Settings
            {
                path: "user",
                meta: {requiresAuth: true, roles: ["ADMIN", "MODERATOR"]},
                redirect: ROUTES.ADMIN.USER.PROFILE,
                children: [
                    {
                        path: "edit/profile",
                        component: Profile,
                        meta: {
                            breadcrumb: [
                                {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                                {title: "User Settings", href: ROUTES.ADMIN.USER.PROFILE},
                                {title: "Profile"},
                            ],
                        },
                    },
                    {
                        path: "edit/password",
                        component: Password,
                        meta: {
                            breadcrumb: [
                                {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                                {title: "User Settings", href: ROUTES.ADMIN.USER.PROFILE},
                                {title: "Password"},
                            ],
                        },
                    },
                    {
                        path: "edit/two-factor",
                        component: TwoFactor,
                        meta: {
                            breadcrumb: [
                                {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                                {title: "User Settings", href: ROUTES.ADMIN.USER.PROFILE},
                                {title: "Two-Factor"},
                            ],
                        },
                    },
                    {
                        path: "edit/appearance",
                        component: Appearance,
                        meta: {
                            breadcrumb: [
                                {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                                {title: "User Settings", href: ROUTES.ADMIN.USER.PROFILE},
                                {title: "Appearance"},
                            ],
                        },
                    },
                ],
            },
        ],
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
    scrollBehavior(to, from, savedPosition) {
        if (savedPosition) return savedPosition;
        if (to.hash) {
            const el = document.querySelector(to.hash);
            if (el) return {el: to.hash, behavior: "smooth"};
        }
        return {top: 0};
    },
});

router.beforeEach(async (to, from) => {
    const authStore = useAuthStore();

    if (to.meta.requiresAuth) {
        if (!authStore.user && !authStore.loading) {
            try {
                await authStore.loadUser();
            } catch {
                return ROUTES.LOGIN;
            }
        }

        if (to.meta.roles) {
            const allowed = (to.meta.roles as string[]).some(role =>
                authStore.hasRole(role)
            );
            if (!allowed) return from.path;
        }
    }

    return true;
});

export default router;
