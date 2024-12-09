<template>
  <div class="transaction-details">
    <div v-if="transaction.type === 'TRANSFER'" class="transaction-category pf-text-medium">
      <span :class="(transaction.amount_cents < 0) ? 'pi pi-arrow-right' : 'pi pi-arrow-left'"></span>
      &nbsp;
      {{
        (transaction.account_to) ? accounts.find((account) => account.id === transaction.account_to).description : "-"
      }}
    </div>
    <div v-else class="transaction-category pf-text-medium">
      {{ (transaction.category) ? transaction.category : "-" }}
    </div>
    <div class="pf-text-medium transaction-amount">
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
    <div class="transaction-creditor pf-text-medium">
      {{ (transaction.creditor_name) ? transaction.creditor_name : "---" }}
    </div>
    <div class="transaction-description pf-text-small">
      {{ transaction.description }}
    </div>
  </div>
</template>

<script>
export default {
  props: {
    transaction: Object,
    accounts: Array,
    privacy: Boolean
  }
}
</script>

<style scoped>
.transaction-details {
  display: grid;
  grid-template:
    'category category amount amount'
    'account-name account-name account-name amount-ref'
    'creditor creditor creditor creditor'
    'description description description description';
  padding: 5px;
  margin-bottom: 2px;
  background-color: var(--color-second-background);
}

.transaction-description {
  grid-area: description;
}

.transaction-creditor {
  grid-area: creditor;
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

.transaction-amount {
  grid-area: amount;
  text-align: right;
  vertical-align: bottom;
  font-weight: bold;
}

.transaction-category {
  grid-area: category;
  text-align: left;
  font-weight: bold;
}
</style>