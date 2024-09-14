<template>
  <div class="toolbar">
    <div class="toolbar-filter">
      {{ filter_description }}
    </div>
    <div class="toolbar-eye">
        <span :class="(privacy) ? 'pi pi-eye' : 'pi pi-eye-slash'"
              @click="togglePrivacy()">
      </span>
    </div>
  </div>
  <div class="transactions-table">
    <div v-if="editDialog">
      <transition name="modal">
        <TransactionEdit :transaction="transaction"
                         :categories="categories"
                         @cancel="editDialog = false"
                         @save="(newCategory) => onCategoryChange(newCategory)">
        </TransactionEdit>
      </transition>
    </div>

    <div v-if="loading">
      Loading...
    </div>
    <div v-else>
      <template v-for="(transactions, booking_date) in byDate" v-bind:key="booking_date">
        <div class="transactions-date-header">
          {{ moment(booking_date).format("DD-MM-YYYY") }}
        </div>
        <div v-for="transaction in transactions" :key="transaction.id">
          <TransactionOverview :transaction=transaction
                               :id=transaction.id
                               :privacy="privacy"
                               v-on:click="onTransactionClick(transaction)">
          </TransactionOverview>
        </div>
      </template>
    </div>
  </div>
</template>

<script>
import TransactionOverview from "@/components/TransactionOverview.vue";
import TransactionEdit from "@/components/TransactionDetailsEdit.vue";
import moment from "moment";
import TransactionApi from "@/TransactionsApi.ts";

export default {
  components: {
    TransactionOverview: TransactionOverview, TransactionEdit
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
    this.loadAllTransactions(this.account_id, this.category_filter);
    this.updateFilterDescription();
    this.loadAllCategories();
  },
  data() {
    return {
      loading: false,
      saving: false,
      editDialog: false,
      transactions: [],
      categories: [],
      transaction: undefined,
      filter_description: "All",
      privacy: Boolean
    }
  },
  methods: {
    moment: moment,
    togglePrivacy() {
      this.privacy = !this.privacy;
    },
    loadAllTransactions(account_id, category) {
      this.loading = true;

      if (!category) category = ""
      if (!account_id) account_id = ""

      TransactionApi.getAllTransactions(account_id, category).then(fetchedTransactions => {
        this.transactions = fetchedTransactions
        this.loading = false;
      });
    },
    loadAllCategories() {
      this.loading = true;
      TransactionApi.getCategories().then(fetchedCategories => {
        this.categories = fetchedCategories.map(
            categoryInfo => {
              return categoryInfo.code
            }
        );
        this.loading = false;
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
    onCategoryChange(newCategory) {
      this.editDialog = false;
      this.saving = true;
      TransactionApi.updateTransactionCategory(this.transaction.id, newCategory).then(updatedTransaction => {
            this.saving = false;
            this.transactions[this.transactions.findIndex(
                transaction => transaction.id === updatedTransaction.id)] = updatedTransaction;
          }
      )
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
  /*font-family: Roboto, Arial, sans-serif;*/
  font-family: monospace;
  font-size: small;
}

.transactions-date-header {
  text-align: left;
  padding: 0 0 0 5px;
  background-color: #6494AA;
  color: white;
}

.toolbar {
  display: grid;
  grid-template: "filter filter filter filter filter none none none none none none eye";
  padding: 0px;
  margin-bottom: 10px;
  margin-top: 10px;
  background-color: white;
}

.toolbar-eye {
  grid-area: eye;
}

.toolbar-filter {
  padding-left: 5px;
  font-size: small;
}
</style>