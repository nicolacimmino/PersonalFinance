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
import TransactionApi from "@/TransactionsApi.ts";
import Alert from "@/models/alert.ts";

export default {
  name: 'HomeView',
  components: {AlertOverview, ToolBar},
  methods: {
    saveApiKey() {
      localStorage.setItem("pfinanceApiKey", this.newApiKey)
      this.existingApiKey = this.newApiKey;
    },
    loadAllAlerts() {
      TransactionApi.loadAllAlerts().then(alerts => {
        this.alerts = alerts
      });

    },
    typeToTypeDescription(type) {
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
  mounted() {
    this.existingApiKey = localStorage.getItem("pfinanceApiKey") || ""
    if (this.existingApiKey !== "") {
      this.loadAllAlerts()
    }
  },
  data() {
    return {
      newApiKey: "",
      existingApiKey: "",
      alerts: Array as Alert[]
    }
  },
  computed: {
    alertsByItemType() {
      return this.alerts.reduce((acc, alert) => {
        (acc[alert.item] = acc[alert.item] || []).push(alert)
        return acc
      }, {})
    }
  }
}
</script>
<style>

.alert-group-header {
  text-align: left;
  padding: 5px;
  margin-top: 15px;
  background-color: #6494AA;
  color: white;
}
</style>