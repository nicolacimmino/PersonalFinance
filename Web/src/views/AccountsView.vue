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
        <div class="account-group-header pf-text-medium">
          <div class="account-group-description">
            {{ typeToTypeDescription(type) }} ({{ accounts.length }})
          </div>
          <div v-if="!privacy" class="account-group-balance">
            {{ Math.floor(totalEurCentsForType(type) / 100.0) }} EUR
          </div>
          <div v-else class="account-group-balance">
            ---
          </div>
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

<script lang="ts">
import AccountOverview from "@/components/AccountOverview.vue";
import CompactAccountOverview from "@/components/AccountOverviewCompact.vue";
import ToolBar from "@/components/ToolBar.vue";
import { useAccountsStore } from '@/stores/accounts';
import { useSettingsStore } from '@/stores/settings';
import { mapState, mapActions } from 'pinia';

export default {
  components: {
    ToolBar,
    CompactAccountOverview,
    AccountOverview: AccountOverview,
  },
  computed: {
    ...mapState(useSettingsStore, ['privacy', 'compact', 'refCurrency']),
    ...mapState(useAccountsStore, ['accounts', 'loading']),
    loaded() {
      return !this.loading;
    },
    byAccountType() {
      return this.accounts
          .slice()
          .sort((a, b) => (a.type > b.type) ? 1 : -1)
          .reduce((acc, account) => {
            (acc[account.type] = acc[account.type] || []).push(account)
            return acc
          }, {})
    }
  },
  mounted() {
    this.fetchAccounts();
  },
  methods: {
    ...mapActions(useSettingsStore, {
      onPrivacyChange: 'setPrivacy',
      onCompactChange: 'setCompact',
      onRefCurrencyChange: 'setRefCurrency'
    }),
    ...mapActions(useAccountsStore, ['fetchAccounts']),
    totalEurCentsForType(type) {
      return this.byAccountType[type].reduce((sum, account) => sum + account.balance_cents_in_ref_currency, 0)
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
  }
}
</script>

<style scoped>
.accounts-table {
  /*height: 20px;*/
}

.account-group-header {
  display: grid;
  grid-template: 'description balance';
  text-align: left;
  padding: 5px;
  margin-top: 15px;
  background-color: var(--color-negative-background);
  color: var(--color-negative-text)
}

.account-group-description {
  grid-area: description;
}

.account-group-balance {
  grid-area: balance;
  text-align: right;
}

</style>