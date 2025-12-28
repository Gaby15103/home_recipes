import { createRouter, createWebHistory } from "vue-router";
import Home from "@/pages/Home.vue";
import Login from "@/pages/Login.vue";
import Register from "@/pages/Register.vue";
import Recipes from "@/pages/Recipes.vue";
import RecipesListAdmin from "@/pages/Admin/Recipes/RecipesListAdmin.vue";
import Admin from "@/pages/Admin/Admin.vue";



const routes = [
    {
        path: "/admin",
        component: Admin,
        meta: { roles: ["ADMIN", "MODERATOR"] },
        children: [
            {
                path: "recipes",
                component: RecipesListAdmin,
            },
        ],
    },

    { path: "/", component: Home },
    { path: "/login", component: Login },
    { path: "/register", component: Register },
    { path: "/recipes", component: Recipes },
    { path: "/recipes/:id", component: Recipes },
]

export const router = createRouter({
    history: createWebHistory(),
    routes: routes,
});
