<template>
  <div class="account-details"
       @click="this.showAccountTransactions(account.id)">
    <div class="description">
      {{ account.description }}
    </div>
    <div class="code">
      {{ account.code }}
    </div>
    <div :class="['balance', (account.balance_cents < 0) ? 'negative-balance' : 'non-negative-balance']">
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
  font-size: 20px;
}

.status {
  grid-area: status;
  font-size: 15px;
  text-align: right;
}

.code {
  grid-area: code;
  text-align: left;
  font-size: 15px;
  color: grey;
}

.balance-in-ref-currency {
  grid-area: balance-ref;
  text-align: right;
  font-size: 15px;
  color: grey;
}

.balance {
  grid-area: balance;
  text-align: right;
  font-size: 20px;
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
  font-size: 15px;
  overflow: clip;
}

</style>