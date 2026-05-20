<template>
  <ToolBar
    @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
    @changeYear="loadAllTransactions(account_id, category_filter)"
    :eye-enabled="true"
    :compact-enabled="true"
    :ref-currency-enabled="compact"
  />
  <div v-if="loading">
    Loading...
  </div>
  <div v-else>
    <!-- Modals (shared between both views) -->
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

    <!-- Normal card view -->
    <div v-if="!compact" class="transactions-table">
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
    </div>

    <!-- Compact table view -->
    <div v-else class="transactions-compact">
      <div class="compact-header-row">
        <div>Date</div>
        <div>Category</div>
        <div>Beneficiary</div>
        <div class="compact-amount-header">Amount</div>
      </div>
      <div v-for="t in transactions" :key="t.id"
           class="compact-row"
           @click="onTransactionClick(t)">
        <div>{{ moment(t.bookingDate).format('DD-MM') }}</div>
        <div class="compact-cell-clip">{{ t.type === 'TRANSFER' ? (t.accountToName || '-') : (t.category || '-') }}</div>
        <div class="compact-cell-clip">{{ t.creditorName || '-' }}</div>
        <div v-if="!privacy" :class="t.amountCents < 0 ? 'compact-negative' : 'compact-positive'">
          {{ refCurrency
            ? formatMoney(t.amountCentsInRefCurrency) + ' ' + t.refCurrency
            : formatMoney(t.amountCents) + ' ' + t.currency }}
        </div>
        <div v-else>---</div>
      </div>
    </div>

    <div class="add_button pi pi-plus-circle" v-on:click="addTransaction()"></div>
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
import { usePrivacy, useLoading, useSnackbar, useYearFilter, useAccounts, useCategories, useSettings } from '@/composables'
import { formatMoney } from '@/utils/format'
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
    const { compact, refCurrency } = useSettings()

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
      allCategories,
      compact,
      refCurrency,
      formatMoney
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
        updatedValues.accountTo,
        updatedValues.creditorName
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
  background-color: var(--color-background);
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
  bottom: 30px;
  left: 50%;
  transform: translateX(-50%);
  font-size: 40px;
  border-radius: 50%;
  background-color: var(--pf-c-dark-gray);
  color: white;
}

.transactions-compact {
  width: 100%;
  font-size: var(--pf-text-medium-font-size);
}

.compact-header-row,
.compact-row {
  display: grid;
  grid-template-columns: 3fr 5fr 7fr 4fr;
  column-gap: 4px;
  padding: 3px 5px;
  align-items: center;
}

.compact-header-row {
  background-color: var(--color-negative-background);
  color: var(--color-negative-text);
  font-weight: bold;
  position: sticky;
  top: 0;
}

.compact-row:nth-child(even) {
  background-color: var(--color-row-alt);
}

.compact-cell-clip {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.compact-amount-header {
  text-align: right;
}

.compact-negative {
  color: #FF7070;
  text-align: right;
}

.compact-positive {
  color: #90A959;
  text-align: right;
}
</style>