import {createWebHistory, createRouter} from 'vue-router'

import TransactionsListView from './views/TransactionsListView.vue'
import CategoryReportView from './views/CategoryReportView.vue'
import HomeView from './views/HomeView.vue'
import AccountsView from './views/AccountsView.vue'

const routes = [
    {
        name: 'home',
        path: '/',
        component: HomeView
    },
    {
        name: 'transactions',
        path: '/transactions',
        query: { category: '' },
        component: TransactionsListView,
        props: true
    },
    {
        name: 'category_report',
        path: '/category_report',
        component: CategoryReportView,
        props: true
    },
    {
        name: 'accounts',
        path: '/accounts',
        component: AccountsView,
        props: true
    },
]

export default createRouter({
    history: createWebHistory(),
    routes,
})
