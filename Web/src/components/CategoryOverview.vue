<template>
  <div class="cso-category-details pf-text-medium">
    <div class="cso-category" @click="$emit('categoryClick', entry.category)">
      {{ entry.category }}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    </div>
    <div class="cso-total">
      <div v-if="privacy">
        ---&nbsp;{{ entry.currency }}
      </div>
      <div v-else>
        {{ Math.floor(Math.abs(entry.total_cents / 100.0)) }}&nbsp;{{ entry.currency }}
      </div>
    </div>
    <div class="cso-transactions-n" @click="$emit('transactionsClick', entry.category)">
      <a>Transactions: {{ entry.transactions_count }}</a>
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
  background-color: var(--color-background);
  color: var(--color-text);
}

.cso-category {
  grid-area: category;
  text-align: left;
}

.cso-total {
  grid-area: total;
  text-align: right;
}

.cso-transactions-n {
  grid-area: transactions-n;
  text-align: left;
  padding-top: 5px;
}
</style>
