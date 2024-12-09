<template>
  <div class="account-details">
    <div class="description pf-text-medium">
      {{ account.description }}
      <span :class="(account.status == 'OK') ? 'pi' : 'pi pi-exclamation-triangle'"
            :style="{color:'red'}"></span>
    </div>
    <div v-if="!refCurrencyActive" class="balance pf-text-medium">
        <span v-if="privacy">---</span>
        <span v-else>{{ account.balance_cents / 100.0 }} {{ account.currency }}</span>
    </div>
    <div v-else class="balance pf-text-medium">
        <span v-if="privacy">---</span>
        <span v-else>{{ account.balance_cents_in_ref_currency / 100.0 }} {{ account.ref_currency }}</span>
    </div>
  </div>
</template>

<script>
import 'primeicons/primeicons.css'

export default {
  props: {
    account: Object,
    privacy: Boolean,
    refCurrencyActive: Boolean
  },
  data() {
    return {
      $router: this.$router
    }
  },
  methods: {
    showAccountTransactions(account_id) {
      this.$router.push({
        path: '/transactions',
        query: {
          account_id: account_id
        }
      });
    }
  }
}
</script>

<style scoped>
.account-details {
  display: grid;
  grid-template: 'description balance';
  padding: 0px;
  margin-bottom: 2px;
  background-color: var(--color-background)
}

.description {
  grid-area: description;
  text-align: left;
}

.balance {
  grid-area: balance;
  vertical-align: middle;
  text-align: right;
  color: var(--color-text);
}

</style>