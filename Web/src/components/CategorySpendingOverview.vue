<template>
  <div>
    <div class="category-details">
      <div class="category" @click="$emit('categoryClick', entry.category)">
        {{ entry.category }}
      </div>
      <div class="total">
        <span v-if="privacy">--- &nbsp;{{ entry.currency }}</span>
        <span v-else>
            {{ Math.abs(entry.total_cents / 100.0) }} {{ entry.currency }}
        </span>

      </div>
      <div class="transactions-n" @click="$emit('transactionsClick', entry.category)">
        Transactions: {{ entry.transactions_count }}
      </div>
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
.category-details {
  display: grid;
  grid-template: 'category total'
                 'transactions-n transactions-n';
  padding: 5px;
  margin-bottom: 5px;
  background-color: #E9B87222;
}

.category {
  grid-area: category;
  text-align: left;
  font-size: medium;
  font-weight: bold;
}

.total {
  grid-area: total;
  text-align: right;
  font-size: medium;
  font-weight: bold;
  color: #90A959;
}

.transactions-n {
  grid-area: transactions-n;
  text-align: left;
  font-size: medium;
}

</style>
