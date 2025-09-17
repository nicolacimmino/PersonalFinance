<template>
  <ToolBar
      @privacy="(newPrivacy) => onPrivacyChange(newPrivacy)"
      @changeYear="loadAllIndicators()"
      :eye-enabled="true"
  />
  <div>
    <div v-if="!loaded">
      Loading...
    </div>
    <div v-else>
      <div class="indicator-header">
        <div class="indicator-label">
          Indicator
        </div>
        <div class="indicator-description">

        </div>
        <div class="indicator-value">

        </div>
      </div>
      <template v-for="indicator in indicators" v-bind:key="indicator.label">
        <div class="indicator-entry">
          <div class="indicator-label">
            {{ indicator.label }}
          </div>
          <div class="indicator-description">
            {{ labelToDescription(indicator.label) }}
          </div>
          <div v-if="!privacy" class="indicator-value">
            {{ Math.floor(indicator.total_cents / 100.0) }}
          </div>
          <div v-else class="indicator-value">
            ---
          </div>
        </div>
      </template>

      <div class="indicator-header">
        Derived Indicators
      </div>
      <div class="indicator-derived-entry">
        <div class="indicator-description">
          Cash/ESS (months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ Math.floor(valueOfIndicator('CASH') / 164000) }}
        </div>
        <div v-else class="indicator-value">
          ---
        </div>
      </div>
      <div class="indicator-derived-entry">
        <div class="indicator-description">
          INPS/ESS (months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ Math.floor(valueOfIndicator('INPS') / 164000) }}
        </div>
        <div v-else class="indicator-value">
          ---
        </div>
      </div>
      <div class="indicator-derived-entry">
        <div class="indicator-description">
          INPS/ESS+DST (months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ Math.floor(valueOfIndicator('INPS') / (164000 + 125000)) }}
        </div>
        <div v-else class="indicator-value">
          ---
        </div>
      </div>
      <div class="indicator-derived-entry">
        <div class="indicator-description">
          INVT 6%/ESS+DST
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ Math.floor((0.06 * valueOfIndicator('INVT')) / (164000 + 125000)) }}
        </div>
        <div v-else class="indicator-value">
          ---
        </div>
      </div>
      <div class="indicator-derived-entry">
        <div class="indicator-description">
          Cash (% of TWO)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ (Math.abs(100 * valueOfIndicator('CASH') / valueOfIndicator('TONW'))).toFixed(1) }}
        </div>
        <div v-else class="indicator-value">
          ---
        </div>
      </div>
      <div class="indicator-derived-entry">
        <div class="indicator-description">
          Income Active (% of total)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{
            (Math.abs(100 * valueOfIndicator('INAT') / (valueOfIndicator('INAT') + (valueOfIndicator('INPS'))))).toFixed(1)
          }}
        </div>
        <div v-else class="indicator-value">
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
    this.loadAllIndicators()
    this.privacy = (localStorage.getItem("privacy") === "true")
  },
  data() {
    return {
      loaded: false,
      privacy: true,
      indicators: []
    }
  },
  methods: {
    onPrivacyChange(newPrivacy) {
      this.privacy = newPrivacy;
    },
    loadAllIndicators() {
      TransactionApi.loadIndicators(localStorage.getItem("year")).then(response => {
        this.indicators = response.indicators;
        this.indicators = response.indicators.sort((a, b) => (this.labelToPosition(a.label) > this.labelToPosition(b.label)) ? 1 : -1)
        this.loaded = true
      });
    },
    valueOfIndicator(label) {
      let res = this.indicators.filter(item => {
        return item.label === label;
      })[0]
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
        case 'EXPE':
          return "Expenses"
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
          return 3
        case 'INAT':
          return 5
        case 'INPS':
          return 6
        case 'EXPE':
          return 7
        case 'CFAT':
          return 8
        case 'CFOA':
          return 9
        default:
          if (type.substring(0, 1) === "C") {
            return 10000000 - (this.valueOfIndicator(type) / 100)
          }
          return 9
      }
    }
  }
}
</script>

<style scoped>
.indicator-header {
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

.indicator-entry {
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

.indicator-label {
  grid-area: label;
}

.indicator-value {
  grid-area: value;
  text-align: right;
}

.indicator-description {
  grid-area: description;
}

.indicator-derived-entry {
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