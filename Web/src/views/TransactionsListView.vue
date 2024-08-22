<template>
  <div class="transactions-table">
    <div v-if="loading">
      Loading...
    </div>
    <div v-else>
      <template v-for="transactions, booking_date in byDate" v-bind:key="booking_date">
        <div class="transactions-date-header">
          {{ moment(booking_date).format("DD/MM/YYYY") }}
        </div>
        <div v-for="transaction in transactions" :key="transaction.id">
          <TransactionOverview :transaction=transaction
                               v-on:click="onTransactionClick(transaction.id)">
          </TransactionOverview>
        </div>
      </template>
    </div>
  </div>
</template>

<script>
import TransactionOverview from "@/components/TransactionOverview.vue";
import moment from "moment";
import TransactionApi from "@/TransactionsApi.ts";

export default {
  components: {
    TransactionOverview: TransactionOverview,
  },
  mounted() {
    this.loadAllTransactions()
  },
  data() {
    return {
      loading: false,
      transactions: []
    }
  },
  methods: {
    moment: moment,
    loadAllTransactions() {
      this.loading = true;
      TransactionApi.getAllTransactions().then(fetchedTransactions => {
        this.transactions = fetchedTransactions

        //     .filter(item => {
        //   return item.type === "EXPENSE"
        // });

        this.loading = false;
      });
    },
    onTransactionClick(id) {
      this.$router.push({name: "transaction_details", params: {id: id}})
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
  padding: 0px;
  background-color: lightsteelblue;
}
</style>