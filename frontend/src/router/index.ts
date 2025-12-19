import { createRouter, createWebHistory } from "vue-router";
import Home from "@/pages/Home.vue";
import Login from "@/pages/Login.vue";
import Register from "@/pages/Register.vue";
import Recipes from "@/pages/Recipes.vue";

export const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: "/", component: Home },
        { path: "/login", component: Login },
        { path: "/register", component: Register },
        { path: "/recipes", component: Recipes },
        { path: "/recipes/:id", component: Recipes },
    ],
});
