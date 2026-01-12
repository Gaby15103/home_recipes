import { createRouter, createWebHistory } from "vue-router";
import Home from "@/pages/Home.vue";
import Login from "@/pages/Login.vue";
import Register from "@/pages/Register.vue";
import Recipes from "@/pages/Recipes.vue";
import List from "@/pages/Admin/Recipes/List.vue";
import Create from "@/pages/Admin/Recipes/Create.vue";
import Edit from "@/pages/Admin/Recipes/Edit.vue";
import Categories from "@/pages/Admin/Recipes/Categories.vue";
import Recipe from "@/pages/Recipe.vue";
import { useAuthStore } from "@/stores/auth.ts";
import AdminDashBoard from "@/pages/Admin/AdminDashBoard.vue";
import Profile from "@/pages/Settings/Profile.vue";
import Password from "@/pages/Settings/Password.vue";
import TwoFactor from "@/pages/Settings/TwoFactor.vue";
import Appearance from "@/pages/Settings/Appearance.vue";
import RecipeDashboard from "@/pages/Admin/Recipes/RecipeDashboard.vue";

const routes = [
    {
        path: "/admin",
        meta: {
            requiresAuth: true,
            roles: ["ADMIN", "MODERATOR"],
            layout: "AdminLayout",
            breadcrumb: [{ title: "Admin", href: "/admin/dashboard" }],
        },
        redirect: "/admin/dashboard",
        children: [
            {
                path: "dashboard",
                component: AdminDashBoard,
                meta: {
                    breadcrumb: [
                        { title: "Admin", href: "/admin/dashboard" },
                        { title: "Dashboard" },
                    ],
                },
            },
            {
                path: "recipe",
                component: RecipeDashboard,
                meta: {
                    breadcrumb: [
                        { title: "Admin", href: "/admin/recipe" },
                        { title: "Recipe" },
                    ],
                },
            },

            {
                path: "recipe/list",
                component: List,
                meta: {
                    breadcrumb: [
                        { title: "Admin", href: "/admin/dashboard" },
                        { title: "Recipe", href: "/admin/recipe" },
                        { title: "Recipe List" },
                    ],
                },
            },

            {
                path: "recipe/create",
                component: Create,
                meta: {
                    breadcrumb: [
                        { title: "Admin", href: "/admin/dashboard" },
                        { title: "Recipe", href: "/admin/recipe" },
                        { title: "Create" },
                    ],
                },
            },

            {
                path: "recipe/edit/:id",
                component: Edit,
                meta: {
                    breadcrumb: [
                        { title: "Admin", href: "/admin/dashboard" },
                        { title: "Recipe", href: "/admin/recipe" },
                        { title: "Edit" },
                    ],
                },
            },

            {
                path: "recipe/categories",
                component: Categories,
                meta: {
                    breadcrumb: [
                        { title: "Admin", href: "/admin/dashboard" },
                        { title: "Categories" },
                    ],
                },
            },

            // -------------------------
            // User Settings (inside admin layout)
            // -------------------------
            {
                path: "user",
                meta: {
                    requiresAuth: true,
                    roles: ["ADMIN", "MODERATOR"],
                    layout: "AdminLayout",
                },
                redirect: "/admin/user/edit/profile",
                children: [
                    {
                        path: "edit/profile",
                        component: Profile,
                        meta: {
                            breadcrumb: [
                                { title: "Admin", href: "/admin/dashboard" },
                                { title: "User Settings", href: "/admin/user/edit/profile" },
                                { title: "Profile" },
                            ],
                        },
                    },
                    {
                        path: "edit/password",
                        component: Password,
                        meta: {
                            breadcrumb: [
                                { title: "Admin", href: "/admin/dashboard" },
                                { title: "User Settings", href: "/admin/user/edit/profile" },
                                { title: "Password" },
                            ],
                        },
                    },
                    {
                        path: "edit/two-factor",
                        component: TwoFactor,
                        meta: {
                            breadcrumb: [
                                { title: "Admin", href: "/admin/dashboard" },
                                { title: "User Settings", href: "/admin/user/edit/profile" },
                                { title: "Two-Factor" },
                            ],
                        },
                    },
                    {
                        path: "edit/appearance",
                        component: Appearance,
                        meta: {
                            breadcrumb: [
                                { title: "Admin", href: "/admin/dashboard" },
                                { title: "User Settings", href: "/admin/user/edit/profile" },
                                { title: "Appearance" },
                            ],
                        },
                    },
                ],
            },
        ],
    },

    { path: "/", component: Home },
    { path: "/login", component: Login },
    { path: "/register", component: Register },
    { path: "/recipes", component: Recipes },
    { path: "/recipe/:id", component: Recipe },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
    scrollBehavior(to, from, savedPosition) {
        if (savedPosition) return savedPosition;

        if (to.hash) {
            const el = document.querySelector(to.hash);
            if (el) {
                return {
                    el: to.hash,
                    behavior: "smooth",
                };
            }
        }

        return { top: 0 };
    },
});

router.beforeEach(async (to, from) => {
    const authStore = useAuthStore();

    if (to.meta.requiresAuth) {
        if (!authStore.user) {
            try {
                await authStore.loadUser();
            } catch {
                return "/login";
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
