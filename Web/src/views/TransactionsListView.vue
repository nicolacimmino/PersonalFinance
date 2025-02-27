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
  <div id="snackbar">message</div>
</template>

<script>
import TransactionOverview from "@/components/TransactionOverview.vue";
import TransactionEdit from "@/components/TransactionEdit.vue";
import ToolBar from "@/components/ToolBar.vue";
import moment from "moment";
import TransactionApi from "@/TransactionsApi.ts";
import 'primeicons/primeicons.css'
import TransactionCreate from "@/components/TransactionCreate.vue";

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
      privacy: Boolean,
      adding: false
    }
  },
  methods: {
    moment: moment,
    addTransaction() {
      this.adding = true
    },
    onPrivacyChange(newPrivacy) {
      this.privacy = newPrivacy;
    },
    loadAllTransactions(account_id, category) {
      this.loading++;

      if (!category) category = ""
      if (!account_id) account_id = ""

      TransactionApi.getTransactions(account_id, category, localStorage.getItem("year")).then(fetchedTransactions => {
        this.transactions = fetchedTransactions
        this.loading--;

        if (this.edit_id) {
          this.transaction = this.transactions.filter(item => {
            return item.id.toString() === this.edit_id.toString();
          })[0];
          this.editDialog = true;
        }
      });
    },
    loadAllCategories() {
      this.loading++;
      TransactionApi.getCategories().then(fetchedCategories => {
        this.categories = fetchedCategories
            .sort((a, b) => (a.code > b.code) ? 1 : -1)
            .filter(categoryInfo => {
              return categoryInfo.discontinued !== 'Y'
            })
            .map(
                categoryInfo => {
                  return categoryInfo.code
                }
            );
        this.loading--;
      });
    },
    loadAllAccounts() {
      this.loading++;
      TransactionApi.getAccounts().then(accounts => {
        this.accounts = accounts.sort((a, b) => (a.description > b.description) ? 1 : -1);
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
    onCategoryChange(newCategory, newAccountTo, isTransfer, newDescription) {
      this.editDialog = false;
      this.saving = true;
      if (!isTransfer) {
        let type = (this.transaction.amount_cents <= 0) ? "EXPENSE" : "INCOME";

        TransactionApi.updateTransactionCategory(this.transaction.id, newCategory, type, newDescription).then(updatedTransaction => {
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
    onNewTransaction(newTransaction) {
      TransactionApi.createTransaction(newTransaction).then(response => {
        let message = "Error"
        if (response !== "ERROR") {
          this.transactions[this.transactions.length] = response;
          message = "Created"
        }

        let snackbar = document.getElementById("snackbar");
        snackbar.textContent = message
        snackbar.className = "show";

        setTimeout(function () {
          snackbar.className = snackbar.className.replace("show", "");
        }, 3000);
      })
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