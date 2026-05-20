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
      <template v-for="indicator in orderedIndicators" v-bind:key="indicator.label">
        <div class="indicator-entry">
          <div class="indicator-label">
            {{ indicator.label }}
          </div>
          <div class="indicator-description">
            {{ labelToDescription(indicator.label) }}
          </div>
          <div v-if="!privacy" class="indicator-value">
            {{ Math.floor(indicator.totalCents / 100.0) }}
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
          CASH/ESS (months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ Math.floor(valueOfIndicator('CASH') / ESS_MONTHLY_COST_CENTS) }}
        </div>
        <div v-else class="indicator-value">
          ---
        </div>
      </div>
      <div class="indicator-derived-entry">
        <div class="indicator-description">
          IP12/ESS (months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ Math.floor(valueOfIndicator('IP12') / ESS_MONTHLY_COST_CENTS) }}
        </div>
        <div v-else class="indicator-value">
          ---
        </div>
      </div>
      <div class="indicator-derived-entry">
        <div class="indicator-description">
          IP12/ESS+DST (months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ Math.floor(valueOfIndicator('IP12') / TOTAL_MONTHLY_COST_CENTS) }}
        </div>
        <div v-else class="indicator-value">
          ---
        </div>
      </div>

      <div class="indicator-derived-entry" :class="fiClass(invtMonths(0.03, 1.2 * ESS_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          INVT 3%/ESS+20%
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ invtMonths(0.03, 1.2 * ESS_MONTHLY_COST_CENTS) }}
        </div>
        <div v-else class="indicator-value">---</div>
      </div>
      <div class="indicator-derived-entry" :class="fiClass(invtMonths(0.04, 1.2 * ESS_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          INVT 4%/ESS+20%
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ invtMonths(0.04, 1.2 * ESS_MONTHLY_COST_CENTS) }}
        </div>
        <div v-else class="indicator-value">---</div>
      </div>
      <div class="indicator-derived-entry" :class="fiClass(invtMonths(0.05, 1.2 * ESS_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          INVT 5%/ESS+20%
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ invtMonths(0.05, 1.2 * ESS_MONTHLY_COST_CENTS) }}
        </div>
        <div v-else class="indicator-value">---</div>
      </div>

      <div class="indicator-derived-entry" :class="fiClass(invtMonths(0.03, 1.2 * TOTAL_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          INVT 3%/ESS+DST+20%
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ invtMonths(0.03, 1.2 * TOTAL_MONTHLY_COST_CENTS) }}
        </div>
        <div v-else class="indicator-value">---</div>
      </div>
      <div class="indicator-derived-entry" :class="fiClass(invtMonths(0.04, 1.2 * TOTAL_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          INVT 4%/ESS+DST+20%
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ invtMonths(0.04, 1.2 * TOTAL_MONTHLY_COST_CENTS) }}
        </div>
        <div v-else class="indicator-value">---</div>
      </div>
      <div class="indicator-derived-entry" :class="fiClass(invtMonths(0.05, 1.2 * TOTAL_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          INVT 5%/ESS+DST+20%
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ invtMonths(0.05, 1.2 * TOTAL_MONTHLY_COST_CENTS) }}
        </div>
        <div v-else class="indicator-value">---</div>
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

<script lang="ts">
import ToolBar from "@/components/ToolBar.vue";
import { ESS_MONTHLY_COST_CENTS, TOTAL_MONTHLY_COST_CENTS, INVESTMENT_RETURN_RATE, INDICATOR_ORDER } from "@/constants";
import { useIndicators, useYearFilter, useSettings } from '@/composables';
import { toRef } from 'vue';

export default {
  components: {
    ToolBar,
  },
  setup() {
    const { selectedYear } = useYearFilter()
    const { indicators, getIndicatorValue, isLoading } = useIndicators(toRef(selectedYear))
    const { privacy, setPrivacy } = useSettings()
    return { indicators, getIndicatorValue, isLoading, privacy, setPrivacy }
  },
  computed: {
    loaded() {
      return !this.isLoading;
    },
    orderedIndicators() {
      const indexed = Object.fromEntries(
        INDICATOR_ORDER.map((label, i) => [label, i])
      );
      return [...this.indicators].sort((a, b) => {
        const ai = indexed[a.label] ?? Infinity;
        const bi = indexed[b.label] ?? Infinity;
        return ai - bi;
      });
    }
  },
  data() {
    return {
      // Expose constants to template
      ESS_MONTHLY_COST_CENTS,
      TOTAL_MONTHLY_COST_CENTS,
      INVESTMENT_RETURN_RATE
    }
  },
  methods: {
    onPrivacyChange(value) {
      this.setPrivacy(value)
    },
    valueOfIndicator(label: string) {
      return this.getIndicatorValue(label);
    },
    invtMonths(rate: number, monthlyBaseCents: number): number {
      return Math.floor((rate * this.valueOfIndicator('INVT')) / monthlyBaseCents);
    },
    fiClass(value: number): string {
      return (!this.privacy && value >= 12) ? 'indicator-fi-achieved' : '';
    },
    labelToDescription(type: string) {
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
        case 'IP12':
          return "Income Passive Last 12 Months"
        case 'IA12':
          return "Income Active Last 12 Months"
        default:
          if (type.substring(0, 1) == "C") {
            return type.substring(1, 4) + " in EUR"
          }
          return "Other"
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
  font-size: var(--pf-text-large-font-size);
}

.indicator-entry:nth-child(even) {
  background-color: var(--color-row-alt);
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
  font-size: var(--pf-text-large-font-size);
}

.indicator-derived-entry:nth-child(even) {
  background-color: var(--color-row-alt);
}

.indicator-fi-achieved {
  color: #90A959;
}
</style>