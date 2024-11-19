<template>
  <div class="transaction-details">
    <div v-if="transaction.type === 'TRANSFER'" class="transaction-category pf-text-medium">
      <span :class="(transaction.amount_cents < 0) ? 'pi pi-arrow-right' : 'pi pi-arrow-left'"></span>
      {{
        (transaction.account_to) ? accounts.find((account) => account.id === transaction.account_to).description : "-"
      }}
    </div>
    <div v-else class="transaction-category pf-text-medium">
      {{ (transaction.category) ? transaction.category : "-" }}
    </div>
    <div class="pf-text-medium" :class="(transaction.amount_cents < 0) ? negativePriceClass : nonNegativePriceClass">
      <span v-if="privacy">---&nbsp;{{ transaction.currency }}</span>
      <span v-else>
            {{ transaction.amount_cents / 100.0 }} {{ transaction.currency }}
        </span>
    </div>
    <div class="transaction-account-name pf-text-small">
      {{ transaction.account_name }}
    </div>
    <div class="transaction-amount-in-ref-currency pf-text-small">
      <span v-if="!privacy && (transaction.ref_currency !== transaction.currency)">
      {{ transaction.amount_cents_in_ref_currency / 100.0 }} {{ transaction.ref_currency }}
      </span>
    </div>
    <div class="transaction-description pf-text-medium">
      {{ (transaction.creditor_name) ? transaction.creditor_name : transaction.description }}
    </div>
  </div>
</template>

<script>
import {ref} from 'vue'

const negativePriceClass = ref("transaction-negative-price");
const nonNegativePriceClass = ref('transaction-non-negative-price');

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
  padding: 5px;
  margin-bottom: 2px;
  background-color: var(--color-second-background);
}

.transaction-description {
  grid-area: description;
}

.transaction-account-name {
  grid-area: account-name;
  text-align: left;
  overflow: clip;
  color: grey;
}

.transaction-amount-in-ref-currency {
  grid-area: amount-ref;
  text-align: right;
  color: grey;
}

.transaction-negative-price {
  grid-area: amount;
  text-align: right;
  vertical-align: bottom;
  font-weight: bold;
  color: #A63D40;
}

.transaction-non-negative-price {
  grid-area: amount;
  text-align: right;
  font-weight: bold;
  color: #90A959;
}

.transaction-category {
  grid-area: category;
  text-align: left;
  font-weight: bold;
}
</style>