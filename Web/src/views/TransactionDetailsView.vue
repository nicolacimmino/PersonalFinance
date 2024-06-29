<template>
  <div v-if="loading">
    Loading...
  </div>
  <div v-else-if="saving">
    Saving...
  </div>
  <div v-else>
    <TransactionDetails :transaction=transaction
                        @updateCategory="(newCategory) => updateTransactionCategory(newCategory)"></TransactionDetails>
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
      loading: true,
      saving: false
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
    updateTransactionCategory(newCategory) {
      this.saving = true;
      TransactionApi.updateTransactionCategory(this.id, newCategory).then(() => {
            this.saving = false;
            this.loadTransaction();
          }
      )
    }
  }
}
</script>

