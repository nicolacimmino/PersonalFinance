<template>
  <div class="cso-category-details">
    <div class="cso-category pf-text-large" @click="$emit('categoryClick', entry.category)">
      <!--  Ugly workaround. On mobile chrome only font size gets smaller unless at least one
          item is long enough!-->
      {{ entry.category }}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    </div>
    <div class="cso-transactions-n pf-text-medium" @click="$emit('transactionsClick', entry.category)">
      <a>Trans.: {{ entry.transactions_count }}</a>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    </div>
    <div v-if="privacy" class="cso-total pf-text-medium">
      ---&nbsp;{{ entry.currency }}
    </div>
    <div v-else class="cso-total pf-text-medium">
      {{ Math.floor(Math.abs(entry.total_cents / 100.0)) }}&nbsp;{{ entry.currency }}
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
  grid-template: 'category category'
                 'transactions-n total';
  padding: 5px;
  margin-bottom: 5px;
  background-color: #E9B87222;
}

.cso-category {
  grid-area: category;
  text-align: left;
}

.cso-total {
  grid-area: total;
  text-align: right;
  color: #90A959;
  padding-top: 10px;
}

.cso-transactions-n {
  grid-area: transactions-n;
  text-align: left;
  padding-top: 10px;
}

</style>
