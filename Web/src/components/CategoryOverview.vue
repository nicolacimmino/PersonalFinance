<template>
  <div class="cso-category-details pf-text-medium">
    <div class="cso-header" @click="$emit('categoryClick', entry.category)">
      <div class="cso-category">
        {{ entry.category }}
      </div>
      <div class="cso-total">
        <div v-if="privacy">
          ---&nbsp;{{ entry.currency }}
        </div>
        <div v-else-if="entry.type === 'EXPENSE'">
          {{ Math.floor(-(entry.totalCents / 100.0)) }}&nbsp;{{ entry.currency }}
        </div>
        <div v-else>
          {{ Math.floor(entry.totalCents / 100.0) }}&nbsp;{{ entry.currency }}
        </div>
      </div>
    </div>
    <div class="cso-description-n" @click="$emit('transactionsClick', entry.category)">
      <template v-for="category in entry.subcategories" v-bind:key="category">
        <div class="subcategory-container">

          <div class="subcategory-category">{{ category.category }}</div>
          <div v-if="entry.type === 'EXPENSE'" class="subcategory-amount">
            {{ Math.floor(-(category.totalCents / 100.0)) }}&nbsp;{{ category.currency }}
          </div>
          <div v-else class="subcategory-amount">
            {{ Math.floor((category.totalCents / 100.0)) }}&nbsp;{{ category.currency }}
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
  margin-top: 12px;
  margin-bottom: 5px;
}

.cso-header {
  display: grid;
  grid-template-columns: 1fr auto;
  padding: 3px 5px;
  background-color: var(--color-negative-background);
  color: var(--color-negative-text);
  font-weight: bold;
  cursor: pointer;
}

.cso-category {
  text-align: left;
}

.cso-total {
  text-align: right;
}

.cso-description-n {
  text-align: left;
  padding: 4px 5px;
  cursor: pointer;
}

.subcategory-container {
  display: grid;
  grid-template: 'sc-category sc-amount';
  grid-template-columns: 2fr 1fr;
  text-align: left;
  color: var(--color-muted-text);
}

.subcategory-category {
  grid-area: sc-category;
}

.subcategory-amount {
  grid-area: sc-amount;
  text-align: right;
}

</style>
