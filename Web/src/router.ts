import {createWebHistory, createRouter} from 'vue-router'

import TransactionsListView from './views/TransactionsListView.vue'
import CategoryReportView from './views/CategoryReportView.vue'
import BudgetReportView from './views/BudgetReportView.vue'
import HomeView from './views/HomeView.vue'
import AccountsView from './views/AccountsView.vue'
import IndicatorsView from './views/IndicatorsView.vue'

const routes = [
    {
        name: 'home',
        path: '/',
        component: HomeView
    },
    {
        name: 'transactions',
        path: '/transactions',
        query: {
            account_id: '',
            category: '',
            edit_id: '',
            edit_return: '',
        },
        component: TransactionsListView,
        props: route => ({
            account_id: route.query.account_id,
            category_filter: route.query.category,
            edit_id: route.query.edit_id,
            edit_return: route.query.edit_return
        })
    },
    {
        name: 'category_report',
        path: '/category_report',
        query: {
            type: '',
        },
        component: CategoryReportView,
        props: route => ({
            type: route.query.type,
        })
    },
    {
        name: 'budgets',
        path: '/budgets',
        component: BudgetReportView,
        props: route => ({
            type: route.query.type,
        })
    },
    {
        name: 'accounts',
        path: '/accounts',
        component: AccountsView,
        props: true
    },
    {
        name: 'indicators',
        path: '/indicators',
        component: IndicatorsView,
        props: true
    },
]

export default createRouter({
    history: createWebHistory(),
    routes,
})
