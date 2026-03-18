<template>
  <ToolBar
    @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
    @changeYear="loadAllTransactions(account_id, category_filter)"
    :eye-enabled="true"
  />
  <div v-if="loading">
    Loading...
  </div>
  <div v-else>
    <div class="transactions-table">
      <div v-if="editDialog">
        <transition name="modal">
          <TransactionEdit :transaction="transaction"
                           :accounts="accounts"
                           :categories="categories"
                           @cancel="editDialog = false; followEditReturn();"
                           @save="updatedValues => {
                             onTransactionValuesChange(updatedValues);
                             followEditReturn();
                           }">
          </TransactionEdit>
        </transition>
      </div>
      <div v-if="adding">
        <transition name="modal">
          <TransactionCreate :accounts="accounts"
                             :categories="categories"
                             @cancel="this.adding=false;"
                             @save="(newTransaction) => {onNewTransaction(newTransaction); this.adding=false}">
          </TransactionCreate>
        </transition>
      </div>
      <template v-for="(transactions, bookingDate) in byDate" v-bind:key="bookingDate">
        <div class="transactions-date-header pf-text-large">
          {{ moment(bookingDate).format('DD-MM-YYYY') }}
        </div>
        <div v-for="transaction in transactions" :key="transaction.id">
          <TransactionOverview
            :transaction="transaction"
            :id="transaction.id"
            :privacy="privacy"
            v-on:click="onTransactionClick(transaction)"
          >
          </TransactionOverview>
        </div>
      </template>
      <div class="add_button pi pi-plus-circle" v-on:click="addTransaction()"></div>
    </div>
  </div>
  <div id="snackbar" :class="{ show: snackbarVisible }">{{ snackbarMessage }}</div>
</template>

<script lang="ts">
import TransactionOverview from '@/components/TransactionOverview.vue'
import TransactionEdit from '@/components/TransactionEdit.vue'
import ToolBar from '@/components/ToolBar.vue'
import moment from 'moment'
import 'primeicons/primeicons.css'
import TransactionCreate from '@/components/TransactionCreate.vue'
import { usePrivacy, useLoading, useSnackbar, useYearFilter, useAccounts, useCategories } from '@/composables'
import { getAccount, getTransactions, getTransaction, updateTransaction, createTransaction } from '@/services/api'

export default {
  components: {
    ToolBar,
    TransactionOverview,
    TransactionEdit,
    TransactionCreate
  },
  props: {
    account_id: String,
    category_filter: String,
    edit_id: String,
    edit_return: String
  },
  setup() {
    // Use composables
    const { privacy, setPrivacy } = usePrivacy()
    const { loading, startLoading, stopLoading } = useLoading()
    const { message: snackbarMessage, isVisible: snackbarVisible, showSnackbar } = useSnackbar()
    const { selectedYear } = useYearFilter()
    const { accounts: allAccounts } = useAccounts()
    const { categories: allCategories } = useCategories()

    return {
      privacy,
      setPrivacy,
      loading,
      startLoading,
      stopLoading,
      snackbarMessage,
      snackbarVisible,
      showSnackbar,
      selectedYear,
      allAccounts,
      allCategories
    }
  },
  computed: {
    byDate() {
      return this.transactions.reduce((acc, transaction) => {
        ;(acc[transaction.bookingDate] = acc[transaction.bookingDate] || []).push(transaction)
        return acc
      }, {})
    },
    categories() {
      return this.allCategories
        .sort((a, b) => (a.code > b.code ? 1 : -1))
        .filter((categoryInfo) => categoryInfo.discontinued !== 'Y')
        .map((categoryInfo) => categoryInfo.code)
    },
    accounts() {
      return [...this.allAccounts].sort((a, b) => (a.description > b.description ? 1 : -1))
    }
  },
  watch: {
    $route: function() {
      this.loadAllTransactions(this.account_id, this.category_filter)
      this.updateFilterDescription()
    }
  },
  mounted() {
    this.loadAllTransactions(this.account_id, this.category_filter)
    this.updateFilterDescription()
  },
  data() {
    return {
      saving: false,
      editDialog: false,
      transaction: undefined,
      transactions: [],
      filter_description: 'All',
      adding: false
    }
  },
  methods: {
    moment: moment,
    addTransaction() {
      this.adding = true
    },
    onPrivacyChange(newPrivacy) {
      this.setPrivacy(newPrivacy)
    },
    async loadAllTransactions(account_id, category) {
      this.startLoading()

      if (!category) category = ''
      if (!account_id) account_id = ''

      this.transactions = await getTransactions(account_id, category, this.selectedYear)
      this.stopLoading()

      if (this.edit_id) {
        this.transaction = await getTransaction(this.edit_id)
        this.editDialog = true
      }
    },
    async updateFilterDescription() {
      if (this.account_id) {
        const account = await getAccount(this.account_id)
        this.filter_description = account.description
        return
      }

      if (this.category_filter) {
        this.filter_description = this.category_filter
        return
      }

      this.filter_description = 'All'
    },
    onTransactionClick(transaction) {
      this.transaction = transaction
      this.editDialog = true
    },
    async onTransactionValuesChange(updatedValues) {
      console.log(updatedValues)
      this.editDialog = false

      if (this.transaction == undefined) {
        return
      }

      this.saving = true

      let type: 'EXPENSE' | 'INCOME' | 'TRANSFER'

      if (updatedValues.isTransfer) {
        type = 'TRANSFER'
      } else if (this.transaction.amountCents <= 0) {
        type = 'EXPENSE'
      } else {
        type = 'INCOME'
      }
      const updated = await updateTransaction(
        this.transaction.id,
        updatedValues.category,
        type,
        updatedValues.description,
        updatedValues.accountTo
      )
      // Update in local list
      const index = this.transactions.findIndex(t => t.id === this.transaction.id)
      if (index >= 0) {
        this.transactions[index] = updated
      }

      this.saving = false
    },
    async onNewTransaction(newTransaction) {
      const response = await createTransaction(newTransaction)
      if (response !== 'ERROR') {
        this.showSnackbar('Created')
        if (typeof response !== 'string') {
          this.transactions.push(response)
        }
      } else {
        this.showSnackbar('Error')
      }
    },
    followEditReturn() {
      if (this.edit_return) {
        this.$router.push({
          path: this.edit_return
        })
      }
    }
  }
}
</script>

<style scoped>
.transactions-table {
  background-color: white;
}

.transactions-date-header {
  text-align: left;
  padding: 3px;
  margin-top: 12px;
  background-color: var(--color-negative-background);
  color: var(--color-negative-text);
}

.add_button {
  position: fixed;
  bottom: 60px;
  right: 60px;
  font-size: 40px;
  border-radius: 50%;
  background-color: var(--pf-c-dark-gray);
  color: white;
}
</style>