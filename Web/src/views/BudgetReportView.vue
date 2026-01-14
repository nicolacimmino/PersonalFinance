<template>
  <ToolBar
      @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
      :eye-enabled="true"
  />
  <div class="pf-tabs">
    <div class="pf-tab" :class="(this.type==='ACTIVE') ? 'pf-tab-selected' : 'pf-tab-inactive'"
         @click="this.$router.push({ path: '/budgets', query: {type:'ACTIVE'}})"
    >
      <p class=" pf-text-large">Active</p>
    </div>
    <div class="pf-tab" :class="(this.type==='PAST') ? 'pf-tab-selected' : 'pf-tab-inactive'"
         @click="this.$router.push({ path: '/budgets', query: {type:'PAST'}})"
    >
      <p class=" pf-text-large">Past</p>
    </div>
  </div>
  <div class="budgets-table">
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      <div v-if="this.type==='ACTIVE'">
        <template v-for="budget in activeBudgets" v-bind:key="budget.id">
          <BudgetOverview :budget=budget :privacy=privacy></BudgetOverview>
        </template>
      </div>
      <div v-else>
        <template v-for="budget in pastBudgets" v-bind:key="budget.id">
          <BudgetOverview :budget=budget :privacy=privacy></BudgetOverview>
        </template>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import BudgetOverview from "@/components/BudgetOverview.vue";
import moment from "moment";
import ToolBar from "@/components/ToolBar.vue";
import { useBudgetsStore } from '@/stores/budgets';
import { useSettingsStore } from '@/stores/settings';
import { mapState, mapActions } from 'pinia';

export default {
  components: {
    ToolBar,
    BudgetOverview,
  },
  props: {
    type: {
      type: String,
      default: "ACTIVE"
    }
  },
  computed: {
    ...mapState(useSettingsStore, ['privacy']),
    ...mapState(useBudgetsStore, ['budgets', 'loading']),
    loaded() {
      return !this.loading;
    },
    activeBudgets() {
      return this.budgets
        .filter(item => moment(item.from_date).isBefore(moment.now()))
        .filter(item => item.active === true);
    },
    pastBudgets() {
      return this.budgets
        .filter(item => moment(item.from_date).isBefore(moment.now()))
        .filter(item => item.active === false)
        .sort((a, b) => (moment(a.start_date).isAfter(moment(b.start_date))) ? 1 : (a.start_date === b.start_date) ? 0 : -1);
    }
  },
  watch: {
    $route: function () {
      this.fetchBudgets();
    }
  },
  mounted() {
    this.fetchBudgets();
  },
  methods: {
    ...mapActions(useSettingsStore, {
      onPrivacyChange: 'setPrivacy'
    }),
    ...mapActions(useBudgetsStore, ['fetchBudgets'])
  },
}
</script>

<style scoped>
.budgets-table {
  padding: 5px;
  background-color: var(--color-background);
}
</style>