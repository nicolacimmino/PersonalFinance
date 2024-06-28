import {createMemoryHistory, createRouter} from 'vue-router'

import TransactionsListView from './views/TransactionsListView.vue'
import TransactionDetailsView from './views/TransactionDetailsView.vue'
import HomeView from './views/HomeView.vue'


const routes = [
    {path: '/', component: HomeView},
    {path: '/transactions', component: TransactionsListView, props: true},
    {path: '/transactions/:id', component: TransactionDetailsView, props: true},
]

export default createRouter({
    history: createMemoryHistory(),
    routes,
})
