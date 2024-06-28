<template>
  <div class="transactionsTable">
    <div v-if="loading">
      Loading...
    </div>
    <div v-else>
      <template v-for="transactions, booking_date in byDate">
        <div>
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
        this.transactions = fetchedTransactions;
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
.transactionsTable {
  border-radius: 10px;
  background-color: white;
}

.transactionsHeader {
  border-radius: 10px 10px 0 0;
  display: grid;
  grid-template: 10px / 0.6fr 1.4fr 1fr;
  text-align: left;
  padding: 15px;
  background-color: lightsteelblue;
  text-transform: uppercase;
}
</style>