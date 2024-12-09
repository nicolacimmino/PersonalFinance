<template>
  <ToolBar
      @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
      :eye-enabled="true"
  />
  <div>
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

      <div class="kpi-header">
        Derived Indicators
      </div>
      <div class="kpi-derived-entry">
        <div class="kpi-description">
          Cash/ESS (months)
        </div>
        <div v-if="!privacy" class="kpi-value">
          {{ Math.floor(valueOfKpi('CASH') / 164000) }}
        </div>
        <div v-else class="kpi-value">
          ---
        </div>
      </div>
      <div class="kpi-derived-entry">
        <div class="kpi-description">
          INPS/ESS (months)
        </div>
        <div v-if="!privacy" class="kpi-value">
          {{ Math.floor(valueOfKpi('INPS') / 164000) }}
        </div>
        <div v-else class="kpi-value">
          ---
        </div>
      </div>
      <div class="kpi-derived-entry">
        <div class="kpi-description">
          INPS/ESS+DST (months)
        </div>
        <div v-if="!privacy" class="kpi-value">
          {{ Math.floor(valueOfKpi('INPS') / (164000 + 125000)) }}
        </div>
        <div v-else class="kpi-value">
          ---
        </div>
      </div>
      <div class="kpi-derived-entry">
        <div class="kpi-description">
          INVT 6%/ESS+DST
        </div>
        <div v-if="!privacy" class="kpi-value">
          {{ Math.floor((0.06 * valueOfKpi('INVT')) / (164000 + 125000)) }}
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
          {{ (Math.abs(100 * valueOfKpi('CASH') / valueOfKpi('TONW'))).toFixed(1) }}
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
          {{ (Math.abs(100 * valueOfKpi('INAT') / (valueOfKpi('INAT') + (valueOfKpi('INPS'))))).toFixed(1) }}
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
        this.kpis = response.kpis;
        this.kpis = response.kpis.sort((a, b) => (this.labelToPosition(a.label) > this.labelToPosition(b.label)) ? 1 : -1)
        this.loaded = true
      });
    },
    valueOfKpi(label) {
      console.log(label)
      let res = this.kpis.filter(item => {
        return item.label === label;
      })[0]
      console.log(res)
      console.log(this.kpis)
      return (res) ? res.total_cents : 20;
    },
    labelToDescription(type) {
      switch (type.substring(0, 4)) {
        case 'CASH':
          return "Cash"
        case 'TONW':
          return "Total Net Worth"
        case 'CFAT':
          return "Cash Flow Active"
        case 'CFOA':
          return "Cash Flow"
        case 'INVT':
          return "Investments"
        case 'INAT':
          return "Income Active"
        case 'INPS':
          return "Income Passive"
        default:
          if (type.substring(0, 1) == "C") {
            return type.substring(1, 4) + " in EUR"
          }
          return "Other"
      }
    },
    labelToPosition(type) {
      switch (type.substring(0, 4)) {
        case 'CASH':
          return 0
        case 'INVT':
          return 1
        case 'TONW':
          return 2
        case 'INAT':
          return 3
        case 'INPS':
          return 4
        case 'CFAT':
          return 5
        case 'CFOA':
          return 6
        default:
          if (type.substring(0, 1) === "C") {
            return 10000000 - (this.valueOfKpi(type) / 100)
          }
          return 8
      }
    }
  }
}
</script>

<style scoped>
.kpi-header {
  width: 90%;
  margin-left: auto;
  margin-right: auto;
  margin-top: 20px;
  padding-left: 10px;
  padding-right: 10px;
  font-size: var(--pf-text-large-font-size);
  font-weight: bold;
  background-color: var(--color-negative-background);
  color: var(--color-negative-text);
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
  font-size: var(--pf-text-small-font-size);
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

.kpi-derived-entry {
  display: grid;
  grid-template: 'description value';
  grid-template-columns: 6fr 4fr;
  width: 90%;
  margin: auto auto;
  padding-top: 5px;
  padding-left: 10px;
  padding-right: 10px;
  font-size: var(--pf-text-small-font-size);
}
</style>