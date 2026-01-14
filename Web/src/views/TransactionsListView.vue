<template>
  <ToolBar
      @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
      @changeYear="loadAllTransactions(this.account_id, this.category_filter)"
      :eye-enabled="true"
  />
  <div v-if="loading > 0">
    Loading...
  </div>
  <div v-else>
    <div class="transactions-table">
      <div v-if="editDialog">
        <transition name="modal">
          <TransactionEdit :transaction="transaction"
                           :accounts="accounts"
                           :categories="categories"
                           @cancel="editDialog = false; this.followEditReturn();"
                           @save="(newCategory, newAccountTo, isTransfer, newDescription) => {onCategoryChange(newCategory, newAccountTo, isTransfer, newDescription); this.followEditReturn();}">
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
      <template v-for="(transactions, booking_date) in byDate" v-bind:key="booking_date">
        <div class="transactions-date-header pf-text-large">
          {{ moment(booking_date).format("DD-MM-YYYY") }}
        </div>
        <div v-for="transaction in transactions" :key="transaction.id">
          <TransactionOverview :transaction=transaction
                               :id=transaction.id
                               :privacy=privacy
                               v-on:click="onTransactionClick(transaction)">
          </TransactionOverview>
        </div>
      </template>
      <div class="add_button pi pi-plus-circle" v-on:click="addTransaction()">
      </div>
    </div>
  </div>
  <div id="snackbar" :class="{ show: snackbarVisible }">{{ snackbarMessage }}</div>
</template>

<script lang="ts">
import TransactionOverview from "@/components/TransactionOverview.vue";
import TransactionEdit from "@/components/TransactionEdit.vue";
import ToolBar from "@/components/ToolBar.vue";
import moment from "moment";
import 'primeicons/primeicons.css'
import TransactionCreate from "@/components/TransactionCreate.vue";
import { usePrivacy, useLoading, useSnackbar, useYearFilter } from '@/composables';
import { useTransactionsStore } from '@/stores/transactions';
import { useAccountsStore } from '@/stores/accounts';
import { useCategoriesStore } from '@/stores/categories';
import { mapState, mapActions } from 'pinia';

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
    edit_return: String,
  },
  setup() {
    // Use composables
    const { privacy, setPrivacy } = usePrivacy();
    const { loading, startLoading, stopLoading } = useLoading();
    const { message: snackbarMessage, isVisible: snackbarVisible, showSnackbar } = useSnackbar();
    const { selectedYear } = useYearFilter();

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
    };
  },
  computed: {
    ...mapState(useTransactionsStore, ['transactions']),
    ...mapState(useAccountsStore, {
      allAccounts: 'accounts'
    }),
    ...mapState(useCategoriesStore, {
      allCategories: 'categories'
    }),
    byDate() {
      return this.transactions.reduce((acc, transaction) => {
        (acc[transaction.booking_date] = acc[transaction.booking_date] || []).push(transaction)
        return acc
      }, {})
    },
    categories() {
      return this.allCategories
        .sort((a, b) => (a.code > b.code) ? 1 : -1)
        .filter(categoryInfo => categoryInfo.discontinued !== 'Y')
        .map(categoryInfo => categoryInfo.code);
    },
    accounts() {
      return [...this.allAccounts].sort((a, b) => (a.description > b.description) ? 1 : -1);
    }
  },
  watch: {
    $route: function () {
      this.loadAllTransactions(this.account_id, this.category_filter);
      this.updateFilterDescription();
    }
  },
  mounted() {
    this.loadAllTransactions(this.account_id, this.category_filter);
    this.updateFilterDescription();
    this.fetchCategories();
    this.fetchAccounts();
  },
  data() {
    return {
      saving: false,
      editDialog: false,
      transaction: undefined,
      filter_description: "All",
      adding: false
    }
  },
  methods: {
    ...mapActions(useTransactionsStore, {
      fetchTransactionsFromStore: 'fetchTransactions',
      updateTransactionFromStore: 'updateTransaction',
      updateTransactionAccountToFromStore: 'updateTransactionAccountTo',
      createTransactionFromStore: 'createTransaction'
    }),
    ...mapActions(useAccountsStore, {
      fetchAccounts: 'fetchAccounts',
      fetchAccount: 'fetchAccount'
    }),
    ...mapActions(useCategoriesStore, ['fetchCategories']),
    moment: moment,
    addTransaction() {
      this.adding = true
    },
    onPrivacyChange(newPrivacy) {
      this.setPrivacy(newPrivacy);
    },
    async loadAllTransactions(account_id, category) {
      this.startLoading();

      if (!category) category = ""
      if (!account_id) account_id = ""

      await this.fetchTransactionsFromStore(account_id, category, this.selectedYear);
      this.stopLoading();

      if (this.edit_id) {
        this.transaction = this.transactions.find(item => {
          return item.id.toString() === this.edit_id.toString();
        });
        this.editDialog = true;
      }
    },
    async updateFilterDescription() {
      if (this.account_id) {
        const account = await this.fetchAccount(this.account_id);
        this.filter_description = account.description;
        return;
      }

      if (this.category_filter) {
        this.filter_description = this.category_filter;
        return;
      }

      this.filter_description = "All"
    },
    onTransactionClick(transaction) {
      this.transaction = transaction;
      this.editDialog = true;
    },
    async onCategoryChange(newCategory, newAccountTo, isTransfer, newDescription) {
      this.editDialog = false;
      this.saving = true;
      if (!isTransfer) {
        let type = (this.transaction.amount_cents <= 0) ? "EXPENSE" : "INCOME";
        await this.updateTransactionFromStore(this.transaction.id, newCategory, type, newDescription);
        this.saving = false;
      } else {
        await this.updateTransactionAccountToFromStore(this.transaction.id, newAccountTo);
        this.saving = false;
      }
    },
    async onNewTransaction(newTransaction) {
      const response = await this.createTransactionFromStore(newTransaction);
      if (response !== "ERROR") {
        this.showSnackbar("Created");
      } else {
        this.showSnackbar("Error");
      }
    },
    followEditReturn() {
      if (this.edit_return) {
        this.$router.push({
          path: this.edit_return,
        });
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