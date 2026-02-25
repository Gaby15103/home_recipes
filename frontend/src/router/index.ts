import {createRouter, createWebHistory} from "vue-router";
import {ROUTES} from "./routes.ts";
import {useAuthStore} from "@/stores/auth.ts";

// Pages
import Home from "@/pages/Home.vue";
import Login from "@/pages/auth/Login.vue";
import Register from "@/pages/auth/Register.vue";
import Index from "@/pages/recipes/Index.vue";
import Show from "@/pages/recipes/Show.vue";
import AdminRecipe from "@/pages/recipes/Recipe.vue"

// Admin Pages
import AdminDashBoard from "@/pages/dashboard/AdminDashBoard.vue";
import RecipeDashboard from "@/pages/dashboard/RecipeDashboard.vue";
import Manage from "@/pages/recipes/Manage.vue";
import Create from "@/pages/recipes/Create.vue";
import Edit from "@/pages/recipes/Edit.vue";
import Categories from "@/pages/recipes/Categories.vue";

// User settings Pages
import Profile from "@/pages/settings/Profile.vue";
import Password from "@/pages/settings/Password.vue";
import TwoFactor from "@/pages/settings/TwoFactor.vue";
import Appearance from "@/pages/settings/Appearance.vue";
import TwoFactorChallenge from "@/pages/auth/TwoFactorChallenge.vue";
import ConfirmEmail from "@/pages/auth/ConfirmEmail.vue";

const routes = [

    {path: ROUTES.HOME, component: Home},
    {path: ROUTES.LOGIN, component: Login},
    {path: ROUTES.TWO_FACTOR, component: TwoFactorChallenge},
    {path: ROUTES.REGISTER, component: Register},
    {path: ROUTES.EMAIL_CONFIRMATION, component: ConfirmEmail },
    {path: ROUTES.RECIPES, component: Index},
    {path: "/recipe/:id", component: Show},

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

            // Show Management
            {
                path: "recipe",
                component: RecipeDashboard,
                meta: {
                    breadcrumb: [
                        {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                        {title: "Show"},
                    ],
                },
            },
            {
                path: "recipe/list",
                component: Manage,
                meta: {
                    breadcrumb: [
                        {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                        {title: "Show", href: ROUTES.ADMIN.RECIPE.BASE},
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
                        {title: "Show", href: ROUTES.ADMIN.RECIPE.BASE},
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
                        {title: "Show", href: ROUTES.ADMIN.RECIPE.BASE},
                        {title: "Edit"},
                    ],
                },
            },
            {
                path: "recipe/:id",
                component: AdminRecipe,
                meta: {
                    breadcrumb: [
                        {title: "Admin", href: ROUTES.ADMIN.DASHBOARD},
                        {title: "Show", href: ROUTES.ADMIN.RECIPE.BASE},
                        {title: ":id"},
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

            // User settings
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
                                {title: "User settings", href: ROUTES.ADMIN.USER.PROFILE},
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
                                {title: "User settings", href: ROUTES.ADMIN.USER.PROFILE},
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
                                {title: "User settings", href: ROUTES.ADMIN.USER.PROFILE},
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
                                {title: "User settings", href: ROUTES.ADMIN.USER.PROFILE},
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
