<template>
  <div class="account-details">
    <div class="description pf-text-medium">
      {{ account.description }}
      <span :class="(account.status == 'OK') ? 'pi' : 'pi pi-exclamation-triangle'"
            :style="{color:'red'}"></span>
    </div>
    <div v-if="!refCurrencyActive" class="balance pf-text-medium">
      <div :class="classOfAmount(account.balance_cents)">
        <span v-if="privacy">---</span>
        <span v-else>{{ account.balance_cents / 100.0 }} {{ account.currency }}</span>
      </div>
    </div>
    <div v-else class="balance pf-text-medium">
      <div :class="classOfAmount(account.balance_cents)">
        <span v-if="privacy">---</span>
        <span v-else>{{ account.balance_cents_in_ref_currency / 100.0 }} {{ account.ref_currency }}</span>
      </div>
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
    classOfAmount(amount) {
      return (amount < 0) ? 'negative-balance' : 'non-negative-balance'
    },
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
  background-color: #E9B87222;
}

.description {
  grid-area: description;
  text-align: left;
}

.balance {
  grid-area: balance;
  vertical-align: middle;
  text-align: right;
}

.negative-balance {
  color: #A63D40;
}

.non-negative-balance {
  color: #90A959;
}

</style>