<template>
  <div class="transaction-details">
    <div v-if="transaction.type === 'TRANSFER'" class="category">
      <span :class="(transaction.amount_cents < 0) ? 'pi pi-arrow-right' : 'pi pi-arrow-left'" style="font-size: 0.5rem"></span>
      {{ (transaction.account_to) ? accounts.find((account) => account.id === transaction.account_to).description : "-" }}
    </div>
    <div v-else class="category">
      {{ (transaction.category) ? transaction.category : "-" }}
    </div>
    <div :class="(transaction.amount_cents < 0) ? negativePriceClass : nonNegativePriceClass">
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
      {{ (transaction.creditor_name) ? transaction.creditor_name : transaction.description }}
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'

const negativePriceClass = ref("negative-price");
const nonNegativePriceClass = ref('non-negative-price');

export default {
  props: {
    transaction: Object,
    accounts: Array,
    privacy: Boolean
  },
  data() {
    return {
      negativePriceClass,
      nonNegativePriceClass
    }
  }
}
</script>

<style scoped>
.transaction-details {
  display: grid;
  grid-template:
    'category amount amount'
    'account-name account-name amount-ref'
    'description description description';
  padding: 0px;
  margin-bottom: 2px;
  background-color: #E9B87222;
}

.description {
  grid-area: description;
  font-size: xx-small;
}

.account-name {
  grid-area: account-name;
  text-align: left;
  font-size: xx-small;
  overflow: clip;
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
  font-size: small;
  font-weight: bold;
  color: #A63D40;
}

.non-negative-price {
  grid-area: amount;
  text-align: right;
  font-size: small;
  font-weight: bold;
  color: #90A959;
}

.category {
  grid-area: category;
  text-align: left;
  font-size: small;
  font-weight: bold;
}

</style>