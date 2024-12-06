<template>
  <ToolBar
      @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
      :eye-enabled="true"
  />
  <div class="pf-text-small">
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      <div class="kpi-header">
        <div class="kpi-label">
          KPI
        </div>
        <div class="kpi-description">

        </div>
        <div class="kpi-value">
          Value
        </div>
      </div>
      <template v-for="kpi in kpis" v-bind:key="kpi.label">
        <div class="kpi-entry">
          <div class="kpi-label">
            {{ kpi.label }}
          </div>
          <div class="kpi-description">
            {{ labelToDescription(kpi.label) }}
          </div>
          <div v-if="!privacy" class="kpi-value">
            {{ Math.floor(kpi.total_cents / 100.0) }}
          </div>
          <div v-else class="kpi-value">
            ---
          </div>
        </div>
      </template>
      <div class="derived">
        <div class="kpi-description">
          Derived KPIs.
        </div>
      </div>
      <div class="kpi-derived-entry">
        <div class="kpi-description">
          Cash/ESS (months)
        </div>
        <div v-if="!privacy" class="kpi-value">
          {{ Math.floor(valueOfKpi('CSH') / 164000) }}
        </div>
        <div v-else class="kpi-value">
          ---
        </div>
      </div>
      <div class="kpi-derived-entry">
        <div class="kpi-description">
          Cash (% of TWO)
        </div>
        <div v-if="!privacy" class="kpi-value">
          {{ (Math.abs(100 * valueOfKpi('CSH') / valueOfKpi('TWO'))).toFixed(1) }}
        </div>
      </div>
    </div>

  </div>
</template>

<script>
import TransactionApi from "@/TransactionsApi.ts";
import ToolBar from "@/components/ToolBar.vue";

export default {
  components: {
    ToolBar,
  },
  mounted() {
    this.loadAllKpis()
    this.privacy = (localStorage.getItem("privacy") === "true")
  },
  data() {
    return {
      loaded: false,
      privacy: true,
      kpis: []
    }
  },
  methods: {
    onPrivacyChange(newPrivacy) {
      this.privacy = newPrivacy;
    },
    loadAllKpis() {
      TransactionApi.loadKpis().then(response => {
        this.kpis = response.kpis
        this.loaded = true
      });
    },
    valueOfKpi(label) {
      return this.kpis.filter(item => {
        return item.label === label;
      })[0].total_cents;
    },
    labelToDescription(type) {
      switch (type) {
        case 'CSH':
          return "Cash"
        case 'TWO':
          return "Total Worth"
        case 'CFY.A':
          return "Cash Flow (Active)"
        case 'CFY.O':
          return "Cash Flow (Overall)"
        case 'INV':
          return "Investments"
        default:
          return "Other"
      }
    }
  }
}
</script>

<style scoped>
.kpi-header {
  display: grid;
  grid-template: 'label description value';
  grid-template-columns: 2fr 6fr 4fr;
  width: 90%;
  margin: auto auto;
  padding-left: 10px;
  padding-right: 10px;
  font-weight: bold;
  padding-top: 50px;
  border-bottom: solid 1px black;
}

.kpi-entry {
  display: grid;
  grid-template: 'label description value';
  grid-template-columns: 2fr 6fr 4fr;
  width: 90%;
  margin: auto auto;
  padding-top: 5px;
  padding-left: 10px;
  padding-right: 10px;
  /*grid-template-columns: 5fr 1fr 1fr 5fr;*/
}

.kpi-label {
  grid-area: label;
}

.kpi-value {
  grid-area: value;
  text-align: right;
}

.kpi-description {
  grid-area: description;
}

.derived {
  display: grid;
  grid-template: 'description';
  width: 90%;
  margin: auto auto;
  padding-left: 10px;
  padding-right: 10px;
  font-weight: bold;
  padding-top: 50px;
  border-bottom: solid 1px black;
}

.kpi-derived-entry {
  display: grid;
  grid-template: 'description value';
  grid-template-columns: 6fr 4fr;
  width: 90%;
  margin: auto auto;
  padding-top: 5px;
  padding-left: 10px;
  padding-right: 10px;
  /*grid-template-columns: 5fr 1fr 1fr 5fr;*/
}
</style>