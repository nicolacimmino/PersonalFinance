<template>
  <ToolBar
  />
  <div>
    <div v-if="existingApiKey === ''">
      Please enter your API key.
      <input type="text" v-model="newApiKey"> <input type="button" value="Save" v-on:click="saveApiKey()">
    </div>
    <div v-else>
      <template v-for="alert in alerts" v-bind:key="alert.message">
        <AlertOverview :alert=alert></AlertOverview>
      </template>
    </div>
  </div>
</template>


<script>

import ToolBar from "@/components/ToolBar.vue";
import AlertOverview from "@/components/AlertOverview.vue";
import TransactionApi from "@/TransactionsApi.ts";

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
      alerts: []
    }
  },
}
</script>
