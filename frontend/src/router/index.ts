import { createRouter, createWebHistory } from "vue-router";
import Home from "@/pages/Home.vue";
import Login from "@/pages/Login.vue";
import Register from "@/pages/Register.vue";
import Recipes from "@/pages/Recipes.vue";
import List from "@/pages/Admin/Recipes/List.vue";
import Create from "@/pages/Admin/Recipes/Create.vue";
import Edit from "@/pages/Admin/Recipes/Edit.vue";
import Categories from "@/pages/Admin/Recipes/Categories.vue";



const routes = [
    {
        path: "/admin",
        meta: { roles: ["ADMIN", "MODERATOR"] },
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
    { path: "/recipes/:id", component: Recipes },
]

export const router = createRouter({
    history: createWebHistory(),
    routes: routes,
});
