<template>
  <div class="cso-category-details pf-text-medium">
    <div class="cso-category" @click="$emit('categoryClick', entry.category)">
      {{ entry.category }}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    </div>
    <div class="cso-total">
      <div v-if="privacy">
        ---&nbsp;{{ entry.currency }}
      </div>
      <div v-else-if="entry.type === 'EXPENSE'">
        {{ Math.floor(-(entry.total_cents / 100.0)) }}&nbsp;{{ entry.currency }}
      </div>
      <div v-else>
        {{ Math.floor(entry.total_cents / 100.0) }}&nbsp;{{ entry.currency }}
      </div>
    </div>
    <div class="cso-transactions-n" @click="$emit('transactionsClick', entry.category)">
      <a>Transactions: {{ entry.transactions_count }}</a>
    </div>
    <div class="cso-description-n" @click="$emit('transactionsClick', entry.category)">
      <template v-for="category in entry.subcategories" v-bind:key="category">
        <div class="subcategory-container">

          <div class="subcategory-category">{{ category.category }}</div>
          <div v-if="entry.type === 'EXPENSE'" class="subcategory-amount">
            {{ Math.floor(-(category.total_cents / 100.0)) }}&nbsp;{{ category.currency }}
          </div>
          <div v-else class="subcategory-amount">
            {{ Math.floor((category.total_cents / 100.0)) }}&nbsp;{{ category.currency }}
          </div>

        </div>
      </template>
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
                 'transactions-n transactions-n'
                 'area-n area-n';
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

.cso-description-n {
  grid-area: area-n;
  text-align: left;
  padding-top: 5px;
}

.subcategory-container {
  display: grid;
  grid-template: 'sc-category sc-amount';
  grid-template-columns: 2fr 1fr;
  text-align: left;
  font-size: var(--pf-text-medium-font-size);
  color: var(--color-secondary-text);
}

.subcategory-category {
  grid-area: sc-category;
}

.subcategory-amount {
  grid-area: sc-amount;
  text-align: right;
}

</style>
