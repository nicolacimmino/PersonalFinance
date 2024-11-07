<template>
  <ToolBar
      @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
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
                           @save="(newCategory, newAccountTo, isTransfer) => {onCategoryChange(newCategory, newAccountTo, isTransfer); this.followEditReturn();}">
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
    category_filter: String,
    edit_id: String,
    edit_return: String,
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

        if (this.edit_id) {
          this.transaction = this.transactions.filter(item => {
            return item.id == this.edit_id;
          })[0];
          this.editDialog = true;
        }
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
    },
    followEditReturn() {
      if (this.edit_return) {
        this.$router.push({
          path: this.edit_return,
        });
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
  font-size: 12px;
}

.transactions-date-header {
  text-align: left;
  padding: 3px;
  margin-top: 12px;
  background-color: #6494AA;
  color: white;
  font-size: 12px;
  font-family: monospace;
}
</style>