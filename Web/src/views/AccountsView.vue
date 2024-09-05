<template>
  <div class="accounts-table">
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      <template v-for="account in accounts" v-bind:key="account.id">
        <AccountOverview :account=account>
        </AccountOverview>
      </template>
    </div>
  </div>
</template>

<script>
import AccountOverview from "@/components/AccountOverview.vue";
import TransactionApi from "@/TransactionsApi.ts";

export default {
  components: {
    AccountOverview: AccountOverview,
  },
  mounted() {
    this.loadAllAccounts()
  },
  data() {
    return {
      loaded: false,
      currentCategoryFilter: "",
      accounts: [],
      chartOptions: {
        responsive: true,
        maintainAspectRatio: false
      },
      chartData: {
        labels: [],
        datasets: [{
          backgroundColor: [],
          data: []
        }]
      }
    }
  },
  methods: {
    loadAllAccounts() {
      TransactionApi.loadAllAccounts().then(accounts => {
        this.accounts = accounts
        this.loaded = true
      });
    },
  },
}
</script>

<style scoped>
.accounts-table {
  border-radius: 10px;
  background-color: white;
}

.transactionsHeader {
  border-radius: 10px 10px 0 0;
  display: grid;
  grid-template: 10px / 0.6fr 1.4fr 1fr;
  text-align: left;
  padding: 15px;
  background-color: lightsteelblue;
  text-transform: uppercase;
}
</style>