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
      <template v-for="group in groupedIndicators" v-bind:key="group.name">
        <div class="indicator-header">{{ group.name }}</div>
        <div v-for="indicator in group.items" v-bind:key="indicator.label" class="indicator-entry" :class="{ 'indicator-bold': indicator.bold }">
          <div class="indicator-label">
            {{ indicator.label }}
          </div>
          <div class="indicator-description">
            {{ indicator.description }}
          </div>
          <div v-if="!privacy" class="indicator-value" :class="indicator.totalCents < 0 ? 'value-negative' : 'value-positive'">
            {{ formatMoney(indicator.totalCents) }}
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
          CASH/ESS (Months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ (valueOfIndicator('CASH') / ESS_MONTHLY_COST_CENTS).toFixed(2) }}
        </div>
        <div v-else class="indicator-value">
          ---
        </div>
      </div>

      <div class="indicator-derived-entry" :class="fiClass(tonwMonths(0.03, 1.2 * ESS_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          TONW 3%/ESS+20% (Months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ tonwMonths(0.03, 1.2 * ESS_MONTHLY_COST_CENTS).toFixed(2) }}
        </div>
        <div v-else class="indicator-value">---</div>
      </div>
      <div class="indicator-derived-entry" :class="fiClass(tonwMonths(0.04, 1.2 * ESS_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          TONW 4%/ESS+20% (Months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ tonwMonths(0.04, 1.2 * ESS_MONTHLY_COST_CENTS).toFixed(2) }}
        </div>
        <div v-else class="indicator-value">---</div>
      </div>
      <div class="indicator-derived-entry" :class="fiClass(tonwMonths(0.05, 1.2 * ESS_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          TONW 5%/ESS+20% (Months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ tonwMonths(0.05, 1.2 * ESS_MONTHLY_COST_CENTS).toFixed(2) }}
        </div>
        <div v-else class="indicator-value">---</div>
      </div>

      <div class="indicator-derived-entry" :class="fiClass(tonwMonths(0.03, 1.2 * TOTAL_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          TONW 3%/ESS+DST+20% (Months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ tonwMonths(0.03, 1.2 * TOTAL_MONTHLY_COST_CENTS).toFixed(2) }}
        </div>
        <div v-else class="indicator-value">---</div>
      </div>
      <div class="indicator-derived-entry" :class="fiClass(tonwMonths(0.04, 1.2 * TOTAL_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          TONW 4%/ESS+DST+20% (Months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ tonwMonths(0.04, 1.2 * TOTAL_MONTHLY_COST_CENTS).toFixed(2) }}
        </div>
        <div v-else class="indicator-value">---</div>
      </div>
      <div class="indicator-derived-entry" :class="fiClass(tonwMonths(0.05, 1.2 * TOTAL_MONTHLY_COST_CENTS))">
        <div class="indicator-description">
          TONW 5%/ESS+DST+20% (Months)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ tonwMonths(0.05, 1.2 * TOTAL_MONTHLY_COST_CENTS).toFixed(2) }}
        </div>
        <div v-else class="indicator-value">---</div>
      </div>

      <div class="indicator-derived-entry">
        <div class="indicator-description">
          Cash (% of TWO)
        </div>
        <div v-if="!privacy" class="indicator-value">
          {{ (Math.abs(100 * valueOfIndicator('CASH') / valueOfIndicator('TONW'))).toFixed(2) }}
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
            (Math.abs(100 * valueOfIndicator('INAT') / (valueOfIndicator('INAT') + (valueOfIndicator('INPS'))))).toFixed(2)
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
import { ESS_MONTHLY_COST_CENTS, TOTAL_MONTHLY_COST_CENTS, INVESTMENT_RETURN_RATE, INDICATORS_INFO } from "@/constants";
import { formatMoney, formatCount } from '@/utils/format'
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
    groupedIndicators() {
      const byLabel = Object.fromEntries(this.indicators.map(ind => [ind.label, ind]));
      const seen = new Set<string>();
      const groups = INDICATORS_INFO.map(({ group, indicators }) => {
        const items = indicators.flatMap(({ indicator, bold, description }) => {
          seen.add(indicator);
          return byLabel[indicator] ? [{ ...byLabel[indicator], bold: bold ?? false, description: description ?? '' }] : [];
        });
        return { name: group, items };
      });
      const remaining = this.indicators.filter(ind => !seen.has(ind.label));
      if (remaining.length) groups.push({ name: '', items: remaining.map(ind => ({ ...ind, bold: false, description: '' })) });
      return groups;
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
    formatMoney,
    formatCount,
    tonwMonths(rate: number, monthlyBaseCents: number): number {
      return Math.floor(100 * (rate * this.valueOfIndicator('TONW')) / monthlyBaseCents) / 100;
    },
    fiClass(value: number): string {
      return (!this.privacy && value >= 12) ? 'indicator-fi-achieved' : '';
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

.indicator-subheader {
  width: 90%;
  margin-left: auto;
  margin-right: auto;
  margin-top: 20px;
  padding-left: 10px;
  padding-right: 10px;
  font-size: var(--pf-text-font-size);
  font-weight: bold;
  color: var(--color-negative-background);
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

.indicator-bold {
  font-weight: bold;
}

.value-positive {
  color: #90A959;
}

.value-negative {
  color: #FF7070;
}
</style>