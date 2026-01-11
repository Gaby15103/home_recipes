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
import UserSettings from "@/components/layout/settings/Layout.vue"
import {useAuthStore} from "@/stores/auth.ts";
import AdminDashBoard from "@/pages/Admin/AdminDashBoard.vue";
import Profile from "@/pages/Settings/Profile.vue";
import Password from "@/pages/Settings/Password.vue";
import TwoFactor from "@/pages/Settings/TwoFactor.vue";
import Appearance from "@/pages/Settings/Appearance.vue";

const routes = [
    {
        path: "/admin",
        meta: { requiresAuth: true, roles: ["ADMIN", "MODERATOR"], layout: "AdminLayout" },
        redirect: "/admin/dashboard",
        children: [
            { path: "dashboard", component: AdminDashBoard },
            { path: "recipes", component: List },
            { path: "recipe/create", component: Create },
            { path: "recipe/edit/:id", component: Edit },
            { path: "recipe/Categories", component: Categories },
        ],
    },
    {
        path: "/user",
        meta: {requiresAuth: true, roles: ["ADMIN", "MODERATOR"], layout: "AdminLayout"},
        redirect: "/user/edit",
        children: [
            { path: "edit", component: UserSettings },
            { path: "edit/profile", component: Profile },
            { path: "edit/password", component: Password },
            { path: "edit/two-factor'", component: TwoFactor },
            { path: "edit/appearance", component: Appearance },
        ]
    },

    { path: "/", component: Home },
    { path: "/login", component: Login },
    { path: "/register", component: Register },
    { path: "/recipes", component: Recipes },
    { path: "/recipe/:id", component: Recipe },
]

const router = createRouter({
    history: createWebHistory(),
    routes: routes,
    scrollBehavior(to, from, savedPosition) {
        if (savedPosition) {
            return savedPosition;
        }
        if (to.hash) {
            const el = document.querySelector(to.hash);
            if (el) {
                return {
                    el: to.hash,
                    behavior: 'smooth',
                };
            }
        }
        return {top: 0};
    },
});

router.beforeEach(async (to,from) => {
    const authStore = useAuthStore();

    if (to.meta.requiresAuth) {
        if (!authStore.user) {
            try {
                await authStore.loadUser(); // load user from /api/user
            } catch {
                return "/login"; // not logged in
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