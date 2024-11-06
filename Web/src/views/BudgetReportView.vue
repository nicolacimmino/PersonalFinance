<template>
  <ToolBar
      @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
      :eye-enabled="true"
  />
  <div class="budgets-table">
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      Active
      <template v-for="budget in activeBudgets" v-bind:key="budget.id">
        <BudgetOverview :budget=budget :privacy=privacy></BudgetOverview>
      </template>
      Past
      <template v-for="budget in pastBudgets" v-bind:key="budget.id">
        <BudgetOverview :budget=budget :privacy=privacy></BudgetOverview>
      </template>
    </div>
  </div>
</template>

<script>
import TransactionApi from "@/TransactionsApi.ts";
import BudgetOverview from "@/components/BudgetOverview.vue";
import moment from "moment";
import ToolBar from "@/components/ToolBar.vue";

export default {
  mounted() {
    this.privacy = (localStorage.getItem("privacy") === "true");
    this.loadBudgets()
  },
  components: {
    ToolBar,
    BudgetOverview,
  },
  data() {
    return {
      loaded: false,
      activeBudgets: [],
      pastBudgets: [],
      privacy: Boolean,
      upEnabled: Boolean
    }
  },
  methods: {
    onPrivacyChange(newPrivacy) {
      this.privacy = newPrivacy;
    },
    loadBudgets() {
      TransactionApi.loadBudgets().then(fetchedBudgets => {
        this.activeBudgets = fetchedBudgets.filter(item => {
          return item.active === true
        });
        this.pastBudgets = fetchedBudgets.filter(item => {
          return item.active === false
        }).sort((a, b) => (moment(a.start_date).isAfter(moment(b.start_date))) ? 1 : (a.start_date === b.start_date) ? 0 : -1);
        this.loaded = true;
      });
    },
  },
}
</script>

<style scoped>
.budgets-table {
  height: 20px;
}
</style>