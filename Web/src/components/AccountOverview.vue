<template>
  <div class="account-details" @click="this.showAccountTransactions(account.id)">
    <div class="description pf-text-medium">
      {{ account.description }}
    </div>
    <div class="code pf-text-small">
      {{ account.code }}
    </div>
    <div class="balance pf-text-medium">
      <span v-if="privacy">--- {{ account.currency }}</span>
      <span v-else>{{ formatMoney(account.balanceCents) }} {{ account.currency }}</span>
    </div>
    <div
      v-if="!privacy && account.currency !== 'EUR'"
      class="balance-in-ref-currency pf-text-small"
    >
      {{ formatMoney(account.balanceRefCurrencyCents) }} EUR
    </div>
    <div class="iban pf-text-small">IBAN: {{ account.iban !== '' ? account.iban : '-' }}</div>
    <div class="status pf-text-medium">
      Status: {{ account.status }}
      <span
        :class="account.status === 'OK' ? 'pi' : 'pi pi-exclamation-triangle'"
        :style="{ color: 'red' }"
      ></span
      >&nbsp;&nbsp;
    </div>
  </div>
</template>

<script>
import 'primeicons/primeicons.css'
import { formatMoney } from '@/utils/format'

export default {
  props: {
    account: Object,
    privacy: Boolean,
    compact: Boolean
  },
  data() {
    return {
      $router: this.$router
    }
  },
  methods: {
    formatMoney,
    showAccountTransactions(account_id) {
      this.$router.push({
        path: '/transactions',
        query: {
          account_id: account_id
        }
      })
    }
  }
}
</script>

<style scoped>
.account-details {
  display: grid;
  grid-template:
    'description balance'
    'code balance-ref'
    'iban iban'
    'status status';
  padding: 5px;
  margin-bottom: 5px;
  background-color: var(--color-background);
  color: var(--color-text);
  row-gap: 3px;
}

.description {
  grid-area: description;
  font-weight: bold;
}

.status {
  grid-area: status;
  text-align: left;
}

.code {
  grid-area: code;
  text-align: left;
  color: var(--color-secondary-text);
}

.balance-in-ref-currency {
  grid-area: balance-ref;
  text-align: right;
  color: var(--color-secondary-text);
}

.balance {
  grid-area: balance;
  text-align: right;
  font-weight: bold;
}

.iban {
  grid-area: iban;
  text-align: left;
  overflow: clip;
}
</style>