<template>
  <div class="cso-category-details">
    <div class="cso-category" @click="$emit('categoryClick', entry.category)">
      {{ entry.category }}
    </div>
    <div class="cso-total">
      <i>
        {{ (privacy) ? "---" : Math.abs(entry.total_cents / 100.0) }} {{ entry.currency }}
      </i>
    </div>
    <div class="cso-transactions-n" @click="$emit('transactionsClick', entry.category)">
      Transactions: {{ entry.transactions_count }}
    </div>
  </div>
</template>

<script>
import 'primeicons/primeicons.css'

export default {
  props: {
    entry: Object,
    privacy: Boolean
  }
};
</script>

<style scoped>
.cso-category-details {
  display: grid;
  grid-template: 'category total'
                 'transactions-n transactions-n';
  padding: 5px;
  margin-bottom: 5px;
  background-color: #E9B87222;
}

.cso-category {
  grid-area: category;
  text-align: left;
  font-size: 15px;
  font-weight: bold;
}

.cso-total {
  grid-area: total;
  text-align: right;
  font-size: 15px;
  font-weight: bold;
  color: #90A959;
}

.cso-transactions-n {
  grid-area: transactions-n;
  text-align: left;
  font-size: 15px;
}

</style>
