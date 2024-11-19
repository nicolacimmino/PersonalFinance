<template>
  <ToolBar
      @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
      @compact="(newCompact) => onCompactChange(newCompact)"
      @ref-currency="(newRefCurrency) => onRefCurrencyChange(newRefCurrency)"
      :compact-enabled="true"
      :eye-enabled="true"
      v-bind:ref-currency-enabled="this.compact && !this.privacy"
  />
  <div class="accounts-table">
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      <template v-for="(accounts, type) in byAccountType" v-bind:key="type">
        <div class="account-group-header">
          {{ typeToTypeDescription(type) }} ({{ accounts.length }})
        </div>
        <template v-for="account in accounts.sort((a,b) => (a.description > b.description) ? 1 : -1) "
                  v-bind:key="account.id">
          <AccountOverview v-if="!compact" :account=account :privacy=privacy :compact=compact></AccountOverview>
          <CompactAccountOverview v-if="compact"
                                  :account=account
                                  :privacy=privacy
                                  :compact=compact
                                  :ref-currency-active=refCurrency
          ></CompactAccountOverview>
        </template>
      </template>
    </div>
  </div>
</template>

<script>
import AccountOverview from "@/components/AccountOverview.vue";
import TransactionApi from "@/TransactionsApi.ts";
import CompactAccountOverview from "@/components/AccountOverviewCompact.vue";
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
      accounts: []
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
    typeToTypeDescription(type) {
      switch (type) {
        case 'CASH':
          return "Cash"
        case 'BANK_CURRENT':
          return "Bank Current"
        case 'BANK_SAVINGS':
          return "Bank Savings"
        case 'BANK_CREDIT':
          return "Credit Cards"
        case 'INV':
          return "Investments"
        case 'ACC':
          return "Accounting"
        default:
          return "Other"
      }
    }
  },
  computed: {
    byAccountType() {
      return this.accounts.sort((a, b) => (a.type > b.type) ? 1 : -1)
          .reduce((acc, account) => {
            (acc[account.type] = acc[account.type] || []).push(account)
            return acc
          }, {})
    }
  }
}
</script>

<style scoped>
.accounts-table {
  /*height: 20px;*/
}

.account-group-header {
  text-align: left;
  padding: 5px;
  margin-top: 15px;
  background-color: #6494AA;
  color: white;
}
</style>