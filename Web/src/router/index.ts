import { createRouter, createWebHistory } from 'vue-router'
import TransactionsList from "@/components/TransactionsList.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: TransactionsList
    },
  ]
})

export default router
