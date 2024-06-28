import {createWebHistory, createRouter} from 'vue-router'

import TransactionsListView from './views/TransactionsListView.vue'
import TransactionDetailsView from './views/TransactionDetailsView.vue'
import HomeView from './views/HomeView.vue'


const routes = [
    {
        name: 'home',
        path: '/',
        component: HomeView
    },
    {
        name: 'transactions',
        path: '/transactions',
        component: TransactionsListView,
        props: true
    },
    {
        name: 'transaction_details',
        path: '/transactions/:id',
        component: TransactionDetailsView,
        props: true
    },
]

export default createRouter({
    history: createWebHistory(),
    routes,
})
