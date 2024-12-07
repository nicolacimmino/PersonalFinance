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
          Indicator
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
          Derived Indicators
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
          INP/ESS (months)
        </div>
        <div v-if="!privacy" class="kpi-value">
          {{ Math.floor(valueOfKpi('INP') / 164000) }}
        </div>
        <div v-else class="kpi-value">
          ---
        </div>
      </div>
      <div class="kpi-derived-entry">
        <div class="kpi-description">
          INP/(ESS+DST) (months)
        </div>
        <div v-if="!privacy" class="kpi-value">
          {{ Math.floor(valueOfKpi('INP') / (164000 + 125000)) }}
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
        <div v-else class="kpi-value">
          ---
        </div>
      </div>
      <div class="kpi-derived-entry">
        <div class="kpi-description">
          Income Active (% of total)
        </div>
        <div v-if="!privacy" class="kpi-value">
          {{ (Math.abs(100 * valueOfKpi('INA') / (valueOfKpi('INA') + (valueOfKpi('INP'))))).toFixed(1) }}
        </div>
        <div v-else class="kpi-value">
          ---
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
        this.kpis = response.kpis.sort((a, b) => (this.labelToPosition(a.label) > this.labelToPosition(b.label)) ? 1 : -1)
        this.loaded = true
      });
    },
    valueOfKpi(label) {
      let res = this.kpis.filter(item => {
        return item.label === label;
      })[0]
      return (res) ? res.total_cents : 0;
    },
    labelToDescription(type) {
      switch (type.substring(0, 3)) {
        case 'CSH':
          return "Cash"
        case 'TWO':
          return "Total Worth"
        case 'CFA':
          return "Cash Flow Active"
        case 'CFO':
          return "Cash Flow"
        case 'INV':
          return "Investments"
        case 'INA':
          return "Income Active"
        case 'INP':
          return "Income Passive"
        case 'CUR':
          return type.substring(4, 7) + " in EUR"
        default:
          return "Other"
      }
    },
    labelToPosition(type) {
      switch (type.substring(0, 3)) {
        case 'CSH':
          return 0
        case 'INV':
          return 1
        case 'TWO':
          return 2
        case 'INA':
          return 3
        case 'INP':
          return 4
        case 'CFA':
          return 5
        case 'CFO':
          return 6
        case 'CUR':
          return 10000000 - (this.valueOfKpi(type)/100)
        default:
          return 8
      }
    }
  }
}
</script>

<style scoped>
.kpi-header {
  display: grid;
  grid-template: 'label description value';
  grid-template-columns: 4fr 6fr 4fr;
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
  grid-template-columns: 4fr 6fr 4fr;
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