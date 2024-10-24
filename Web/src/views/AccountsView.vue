<template>
  <ToolBar
      @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
      @compact="(newCompact) => onCompactChange(newCompact)"
      @ref-currency="(newRefCurrency) => onRefCurrencyChange(newRefCurrency)"
      compact-enabled="true"
      v-bind:ref-currency-enabled="this.compact && !this.privacy"
  />
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
                                :ref-currency-active=refCurrency
        ></CompactAccountOverview>
      </template>
    </div>
  </div>
</template>

<script>
import AccountOverview from "@/components/AccountOverview.vue";
import TransactionApi from "@/TransactionsApi.ts";
import CompactAccountOverview from "@/components/CompactAccountOverview.vue";
import ToolBar from "@/components/ToolBar.vue";

export default {
  components: {
    ToolBar,
    CompactAccountOverview,
    AccountOverview: AccountOverview,
  },
  mounted() {
    this.loadAllAccounts()
    this.privacy = (localStorage.getItem("privacy") === "true")
    this.compact = (localStorage.getItem("compact") === "true")
    this.refCurrency = (localStorage.getItem("refCurrency") === "true")
  },
  data() {
    return {
      loaded: false,
      privacy: true,
      compact: true,
      refCurrency: false,
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
    onPrivacyChange(newPrivacy) {
      this.privacy = newPrivacy;
    },
    onCompactChange(newCompact) {
      this.compact = newCompact;
    },
    onRefCurrencyChange(newRefCurrency) {
      this.refCurrency = newRefCurrency;
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
  height: 20px;
}
</style>