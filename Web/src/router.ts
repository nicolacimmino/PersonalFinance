import {createWebHistory, createRouter} from 'vue-router'

import TransactionsListView from './views/TransactionsListView.vue'
import ModalTransactionDetailsView from './views/ModalTransactionDetailsView.vue'
import CategoryReportView from './views/CategoryReportView.vue'
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
        query: { category: '' },
        component: TransactionsListView,
        props: true
    },
    {
        name: 'transaction_details',
        path: '/transactions/:id',
        component: ModalTransactionDetailsView,
        props: true
    },
    {
        name: 'category_report',
        path: '/category_report',
        component: CategoryReportView,
        props: true
    },
]

export default createRouter({
    history: createWebHistory(),
    routes,
})
