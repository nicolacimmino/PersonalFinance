<template>
  <div class="account-details"
       @click="this.showAccountTransactions(account.id)">
    <div class="description">
      {{ account.description }}
    </div>
    <div class="code">
      {{ account.code }}
    </div>
    <div :class="['balance', 'balance-style', (account.balance_cents < 0) ? 'negative-balance' : 'non-negative-balance']">
      <span v-if="privacy">---  {{ account.currency }}</span>
      <span v-else>{{ account.balance_cents / 100.0 }} {{ account.currency }}</span>
    </div>
    <div v-if="!privacy" class="balance-in-ref-currency">
      {{ account.balance_cents_in_ref_currency / 100.0 }} {{ account.ref_currency }}
    </div>
    <div class="iban">
      IBAN: {{ (account.iban != "") ? account.iban : "-" }}
    </div>
    <div class="status">
       <span :class="(account.status == 'OK') ? 'pi' : 'pi pi-exclamation-triangle'"
             :style="{color:'red'}"></span>
      Status: {{ account.status }}
    </div>
  </div>
</template>

<script>
import 'primeicons/primeicons.css'

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
  grid-template:
    'description balance'
    'code balance-ref'
    'iban iban'
    'status status';
  padding: 0px;
  margin-bottom: 2px;
  background-color: #E9B87222;
}

.description {
  grid-area: description;
  font-size: smaller;
}

.status {
  grid-area: status;
  font-size: smaller;
  text-align: right;
}

.code {
  grid-area: code;
  text-align: left;
  font-size: xx-small;
  color: grey;
}

.balance-in-ref-currency {
  grid-area: balance-ref;
  text-align: right;
  font-size: xx-small;
  color: grey;
}

.balance {
  grid-area: balance;
  text-align: right;
}

.balance-ref {
  grid-area: balance-ref;
}

.balance-style {
  text-align: right;
  vertical-align: middle;
  font-size: medium;
  font-weight: bold;
}
.negative-balance {
  color: #A63D40;
}

.non-negative-balance {
  color: #90A959;
}

.iban {
  grid-area: iban;
  text-align: left;
  font-size: small;
}

</style>