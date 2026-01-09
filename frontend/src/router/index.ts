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
import {useAuthStore} from "@/stores/auth.ts";

const routes = [
    {
        path: "/admin",
        meta: { requiresAuth: true, roles: ["ADMIN", "MODERATOR"] },
        redirect: "/admin/recipes",
        children: [
            { path: "recipes", component: List },
            { path: "recipes/create", component: Create },
            { path: "recipes/edit", component: Edit },
            { path: "recipes/Categories", component: Categories },
        ],
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
            if (!allowed) return from.path; // forbidden page
        }
    }

    return true;
});

export default router;