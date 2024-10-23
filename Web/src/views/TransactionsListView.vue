<template>
  <div v-if="loading > 0">
    Loading...
  </div>
  <div v-else>
    <ToolBar
        @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
    />
    <div class="transactions-table">
      <div v-if="editDialog">
        <transition name="modal">
          <TransactionEdit :transaction="transaction"
                           :accounts="accounts"
                           :categories="categories"
                           @cancel="editDialog = false"
                           @save="(newCategory, newAccountTo, isTransfer) => onCategoryChange(newCategory, newAccountTo, isTransfer)">
          </TransactionEdit>
        </transition>
      </div>

      <template v-for="(transactions, booking_date) in byDate" v-bind:key="booking_date">
        <div class="transactions-date-header">
          {{ moment(booking_date).format("DD-MM-YYYY") }}
        </div>
        <div v-for="transaction in transactions" :key="transaction.id">
          <TransactionOverview :transaction=transaction
                               :accounts=accounts
                               :id=transaction.id
                               :privacy=privacy
                               v-on:click="onTransactionClick(transaction)">
          </TransactionOverview>
        </div>
      </template>
    </div>
  </div>
</template>

<script>
import TransactionOverview from "@/components/TransactionOverview.vue";
import TransactionEdit from "@/components/TransactionEdit.vue";
import ToolBar from "@/components/ToolBar.vue";
import moment from "moment";
import TransactionApi from "@/TransactionsApi.ts";
import 'primeicons/primeicons.css'

export default {
  components: {
    ToolBar,
    TransactionOverview,
    TransactionEdit
  },
  props: {
    account_id: String,
    category_filter: String
  },
  watch: {
    $route: function () {
      this.loadAllTransactions(this.account_id, this.category_filter);
      this.updateFilterDescription();
    }
  },
  mounted() {
    this.privacy = (localStorage.getItem("privacy") === "true")
    this.loadAllTransactions(this.account_id, this.category_filter);
    this.updateFilterDescription();
    this.loadAllCategories();
    this.loadAllAccounts();
  },
  data() {
    return {
      loading: 0,
      saving: false,
      editDialog: false,
      transactions: [],
      categories: [],
      accounts: [],
      transaction: undefined,
      filter_description: "All",
      privacy: Boolean
    }
  },
  methods: {
    moment: moment,
    onPrivacyChange(newPrivacy) {
      this.privacy = newPrivacy;
    },
    loadAllTransactions(account_id, category) {
      this.loading++;

      if (!category) category = ""
      if (!account_id) account_id = ""

      TransactionApi.getAllTransactions(account_id, category).then(fetchedTransactions => {
        this.transactions = fetchedTransactions
        this.loading--;
      });
    },
    loadAllCategories() {
      this.loading++;
      TransactionApi.getCategories().then(fetchedCategories => {
        this.categories = fetchedCategories.map(
            categoryInfo => {
              return categoryInfo.code
            }
        );
        this.loading--;
      });
    },
    loadAllAccounts() {
      this.loading++;
      TransactionApi.getAccounts().then(fetchedAccounts => {
        this.accounts = fetchedAccounts;
        this.loading--;
      });
    },
    updateFilterDescription() {
      if (this.account_id) {
        TransactionApi.getAccount(this.account_id).then(account => {
          this.filter_description = account.description;
        });
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
    onCategoryChange(newCategory, newAccountTo, isTransfer) {
      this.editDialog = false;
      this.saving = true;
      if (!isTransfer) {
        let type = (this.transaction.amount_cents <= 0) ? "EXPENSE" : "INCOME";

        TransactionApi.updateTransactionCategory(this.transaction.id, newCategory, type).then(updatedTransaction => {
              this.saving = false;
              this.transactions[this.transactions.findIndex(
                  transaction => transaction.id === updatedTransaction.id)] = updatedTransaction;
            }
        )
      } else {
        TransactionApi.updateTransactionAccountTo(this.transaction.id, newAccountTo).then(updatedTransaction => {
              this.saving = false;
              this.transactions[this.transactions.findIndex(
                  transaction => transaction.id === updatedTransaction.id)] = updatedTransaction;
            }
        )
      }
    }
  },
  computed: {
    byDate() {
      return this.transactions.reduce((acc, transaction) => {
        (acc[transaction.booking_date] = acc[transaction.booking_date] || []).push(transaction)
        return acc
      }, {})
    }
  }
}
</script>

<style scoped>

.transactions-table {
  background-color: white;
  font-family: monospace;
  font-size: small;
}

.transactions-date-header {
  text-align: left;
  padding: 0 0 0 5px;
  background-color: #6494AA;
  color: white;
  font-size: small;
}

.tlw-toolbar {
  display: grid;
  grid-template: "none eye";
  grid-template-columns: [none] 9fr [eye] 1fr;
  padding: 0px;
  margin-bottom: 10px;
  margin-top: 10px;
  background-color: white;
  font-size: 1em;
}

.tlw-toolbar-eye {
  grid-area: eye;
}
</style>