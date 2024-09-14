<template>
  <div class="toolbar">
    <div class="toolbar-eye">
      <span :class="(privacy) ? 'pi pi-eye' : 'pi pi-eye-slash'"
            @click="togglePrivacy()">
      </span>
    </div>
  </div>
  <div class="accounts-table">
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      <template v-for="account in accounts" v-bind:key="account.id">
        <AccountOverview :account=account :privacy=privacy>
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
      privacy: true,
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

.toolbar {
  display: grid;
  grid-template: "none none none none none none none none none none none eye";
  padding: 0px;
  margin-bottom: 10px;
  margin-top: 10px;
  background-color: white;
}

.toolbar-arrow {
  grid-area: arrow;
}

.toolbar-eye {
  grid-area: eye;
}

</style>