<template>
  <div class="transaction-details">
    <div v-if="transaction.type === 'TRANSFER'" class="transaction-category pf-text-medium">
      <span :class="(transaction.amountCents < 0) ? 'pi pi-arrow-right' : 'pi pi-arrow-left'"></span>
      &nbsp;
      {{
        (transaction.accountToName) ? transaction.accountToName : "-"
      }}
    </div>
    <div v-else class="transaction-category pf-text-medium">
      {{ (transaction.category) ? transaction.category : "-" }}
    </div>
    <div class="pf-text-medium transaction-amount">
      <span v-if="privacy">---&nbsp;{{ transaction.currency }}</span>
      <span v-else>
            {{ transaction.amountCents / 100.0 }} {{ transaction.currency }}
        </span>
    </div>
    <div class="transaction-account-name pf-text-small">
      {{ transaction.accountName }}
    </div>
    <div class="transaction-amount-in-ref-currency pf-text-small">
      <span v-if="!privacy && (transaction.refCurrency !== transaction.currency)">
      {{ transaction.amountCentsInRefCurrency / 100.0 }} {{ transaction.refCurrency }}
      </span>
    </div>
    <div class="transaction-creditor pf-text-medium">
      {{ (transaction.creditorName) ? transaction.creditorName : "---" }}
    </div>
    <div class="transaction-description pf-text-small">
      {{ transaction.description }}
    </div>
  </div>
</template>

<script lang="ts">
export default {
  props: {
    transaction: Object,
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
  background-color: var(--color-background);
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
  color: var(--color-muted-text);
}

.transaction-amount-in-ref-currency {
  grid-area: amount-ref;
  text-align: right;
  color: var(--color-muted-text);
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