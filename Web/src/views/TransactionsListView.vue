<template>
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
          <TransactionOverview :transaction=transaction :id="transaction.id"
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
  mounted() {
    this.loadAllTransactions();
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
    }
  },
  methods: {
    moment: moment,
    loadAllTransactions() {
      this.loading = true;
      TransactionApi.getAllTransactions().then(fetchedTransactions => {
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
            //document.getElementById(updatedTransaction.id).transaction = updatedTransaction;
            //this.loadAllTransactions();
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
</style>