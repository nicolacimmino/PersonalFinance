<template>
  <div class="transaction-details">
    <div v-if="transaction.type === 'TRANSFER'" class="category">
      Transfer
    </div>
    <div v-else class="category">
      {{ (transaction.category) ? transaction.category : "-" }}
    </div>
    <div :class="(transaction.amount_cents < 0) ? 'negative-price' : 'non-negative-price'">
      <span v-if="privacy">--- &nbsp;{{ transaction.currency }}</span>
      <span v-else>
            {{ transaction.amount_cents / 100.0 }} {{ transaction.currency }}
        </span>
    </div>
    <div class="account-name">
      {{ transaction.account_name }}
    </div>
    <div class="amount-in-ref-currency">
      {{ transaction.amount_cents_in_ref_currency / 100.0 }} {{ transaction.ref_currency }}
    </div>
    <div class="description">
      {{ (transaction.description) ? transaction.description : "-" }}
    </div>
  </div>
</template>

<script>

export default {
  props: {
    transaction: Object,
    privacy: Boolean
  },
}
</script>

<style scoped>
.transaction-details {
  display: grid;
  grid-template:
    'category amount'
    'account-name amount-ref'
    'description description';
  padding: 0px;
  margin-bottom: 2px;
  background-color: #E9B87222;
}

.description {
  grid-area: description;
  font-size: smaller;
}

.account-name {
  grid-area: account-name;
  text-align: left;
  font-size: xx-small;
  color: grey;
}

.amount-in-ref-currency {
  grid-area: amount-ref;
  text-align: right;
  font-size: xx-small;
  color: grey;
}

.negative-price {
  grid-area: amount;
  text-align: right;
  vertical-align: bottom;
  font-size: medium;
  font-weight: bold;
  color: #A63D40;
}

.non-negative-price {
  grid-area: amount;
  text-align: right;
  font-size: medium;
  font-weight: bold;
  color: #90A959;
}

.category {
  grid-area: category;
  text-align: left;
  font-size: medium;
  font-weight: bold;
}

</style>