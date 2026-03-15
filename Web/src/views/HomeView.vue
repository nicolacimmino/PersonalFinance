<template>
  <ToolBar
  />
  <div>
    <div v-if="existingApiKey === ''">
      Please enter your API key.
      <input type="text" v-model="newApiKey"> <input type="button" value="Save" v-on:click="saveApiKey()">
    </div>
    <div v-else>
      <div v-if="this.alerts.length === 0">
        No Alerts.
      </div>
      <template v-for="(alerts, type) in alertsByItemType" v-bind:key="type">
        <div class="alert-group-header pf-text-large">
          {{ typeToTypeDescription(type) }} ({{ alerts.length }})
        </div>
        <template v-for="alert in alerts" v-bind:key="alert.message">
          <AlertOverview :alert=alert></AlertOverview>
        </template>
      </template>
    </div>
  </div>
</template>


<script lang="ts">

import ToolBar from "@/components/ToolBar.vue";
import AlertOverview from "@/components/AlertOverview.vue";
import { useAlerts, useSettings } from '@/composables';

export default {
  name: 'HomeView',
  components: {AlertOverview, ToolBar},
  setup() {
    const { alerts, alertsByItemType } = useAlerts()
    const settings = useSettings()
    return { alerts, alertsByItemType, settings }
  },
  computed: {
    existingApiKey() {
      return this.settings.apiKey
    },
  },
  methods: {
    saveApiKey() {
      this.settings.setApiKey(this.newApiKey);
      this.newApiKey = "";
    },
    typeToTypeDescription(type: string) {
      switch (type) {
        case 'ACCOUNTS':
          return "Accounts"
        case 'BUDGETS':
          return "Budgets"
        case 'TRANSACTIONS':
          return "Transactions"
        default:
          return "Other"
      }
    }
  },
  data() {
    return {
      newApiKey: "",
    }
  },
}
</script>
<style>

.alert-group-header {
  text-align: left;
  padding: 5px;
  margin-top: 15px;
  background-color: var(--color-negative-background);
  color: var(--color-negative-text);
}
</style>