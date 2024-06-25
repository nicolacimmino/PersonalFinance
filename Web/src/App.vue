<template>
  <div class="mainContainer">
    <TransactionsList :transactions=transactions :loading=loading></TransactionsList>
  </div>
</template>

<script>
import TransactionsList from "@/components/TransactionsList.vue";
import TransactionApi from "./TransactionsApi.ts";

export default {
  name: 'App',
  components: {
    TransactionsList,
  },
  async created() {
    await this.$router.isReady();
    this.getQueryParams()
    this.loadAllTransactions()
  },
  data() {
    return {
      transactions: [],
      selectedExpense: undefined,
      loading: false,
      apiKey: null
    }
  },
  methods: {
    loadAllTransactions() {
      this.loading = true;
      TransactionApi.getAll(this.apiKey).then(transactions => {
        this.transactions = transactions;
        this.loading = false;
      });
    },
    getQueryParams() {
      this.apiKey = this.$route.query.apikey
    }
  }
}
</script>

<style>
.mainContainer {
  background-color: whitesmoke;
  padding: 20px;
  width: 80%;
}
</style>