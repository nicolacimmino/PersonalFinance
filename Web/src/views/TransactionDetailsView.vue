<template>
  <div v-if="loading">
    Loading...
  </div>
  <div v-else>
    <TransactionDetails :transaction=transaction></TransactionDetails>
  </div>
</template>

<script>
import TransactionDetails from "@/components/TransactionDetails.vue";
import TransactionApi from "@/TransactionsApi";

export default {
  components: {
    TransactionDetails
  },
  props: {
    id: undefined
  },
  mounted() {
    this.loadTransaction()
  },
  data() {
    return {
      transaction: Object,
      loading: true
    }
  },
  methods: {
    loadTransaction() {
      this.loading = true;
      TransactionApi.getTransaction(this.id).then(fetchedTransaction => {
        this.transaction = fetchedTransaction;
        this.loading = false;
      });
    },
  }
}
</script>

