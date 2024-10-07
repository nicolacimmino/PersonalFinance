<template>
  <div>
    <div class="category-details">
      <div class="description">
        {{ budget.description }}
      </div>
      <div class="size">
        <span>
            {{ Math.abs(budget.amount_cents / 100.0) }} {{ budget.currency }}
        </span>
      </div>
      <div class="spent">
        <span>
            {{ Math.abs(budget.spent_cents / 100.0) }} {{ budget.currency }}
        </span>

      </div>
    </div>
  </div>

</template>

<script>
import 'primeicons/primeicons.css'

export default {
  props: {
    budget: Object,
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
