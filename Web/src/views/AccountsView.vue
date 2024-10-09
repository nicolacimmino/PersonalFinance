<template>
  <div class="toolbar">
    <div class="toolbar-eye">
      <span :class="(privacy) ? 'pi pi-eye' : 'pi pi-eye-slash'"
            @click="togglePrivacy()">
      </span>
    </div>
    <div class="toolbar-compact">
      <span :class="(compact) ? 'pi pi-window-maximize' : 'pi pi-window-minimize'"
            @click="toggleCompact()">
      </span>
    </div>
    <div class="toolbar-currency">
      <span :class="(refCurrencyActive) ? 'pi pi-money-bill' : 'pi pi-euro'"
            :style="(!compact) ? {color:'#AAAAAA'}: {color:'#000000'}"
            @click="toggleRefCurrencyActive()">
      </span>
    </div>
  </div>
  <div class="accounts-table">
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      <template v-for="account in accounts" v-bind:key="account.id">
        <AccountOverview v-if="!compact" :account=account :privacy=privacy :compact=compact></AccountOverview>
        <CompactAccountOverview v-if="compact"
                                :account=account
                                :privacy=privacy
                                :compact=compact
                                :ref-currency-active=refCurrencyActive
        ></CompactAccountOverview>
      </template>
    </div>
  </div>
</template>

<script>
import AccountOverview from "@/components/AccountOverview.vue";
import TransactionApi from "@/TransactionsApi.ts";
import CompactAccountOverview from "@/components/CompactAccountOverview.vue";

export default {
  components: {
    CompactAccountOverview,
    AccountOverview: AccountOverview,
  },
  mounted() {
    this.loadAllAccounts()
  },
  data() {
    return {
      loaded: false,
      privacy: true,
      compact: true,
      refCurrencyActive: false,
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
    togglePrivacy() {
      this.privacy = !this.privacy;
    },
    toggleCompact() {
      this.compact = !this.compact;
    },
    toggleRefCurrencyActive() {
      if (this.compact) {
        this.refCurrencyActive = !this.refCurrencyActive
      }
    },
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

.toolbar {
  display: grid;
  grid-template: "none currency compact eye";
  grid-template-columns: [none] 7fr [currency] 1fr [compact] 1fr [compact] 1fr;
  padding: 0px;
  margin-bottom: 10px;
  margin-top: 10px;
  background-color: white;
  text-align: right;
}

.toolbar-compact {
  grid-area: compact;
}

.toolbar-currency {
  grid-area: currency;
}

.toolbar-eye {
  grid-area: eye;
}

</style>