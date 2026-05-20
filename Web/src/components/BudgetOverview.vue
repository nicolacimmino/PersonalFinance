<template>
  <div class="budget-details">
    <div class="budget-description">
      {{ budget.description }}
    </div>
    <div class="budget-data-container">
      <div class="budget-data">
        <div class="budget-data-label">
          Valid from
        </div>
        <div class="budget-data-value">
          {{ budget.fromDate }}
        </div>
      </div>
      <div class="budget-data">
        <div class="budget-data-label">
          Valid to
        </div>
        <div class="budget-data-value">
          {{ budget.toDate }}
        </div>
      </div>
      <div class="budget-data" v-if="!privacy">
        <div class="budget-data-label">
          Size
        </div>
        <div class="budget-data-value">
          {{ formatMoney(Math.abs(budget.amountCents)) }} {{ budget.currency }}
        </div>
      </div>
      <div class="budget-data" v-if="!privacy">
        <div class="budget-data-label">
          Spent
        </div>
        <div class="budget-data-value">
          {{ formatMoney(Math.abs(budget.spentCents)) }} {{ budget.currency }}
        </div>
      </div>
      <div class="budget-data">
        <div class="budget-data-label">
          Spent %
        </div>
        <div class="budget-data-value">
          {{ Math.floor((100.0 * Math.abs(budget.spentCents)) / budget.amountCents) }}%
        </div>
      </div>
      <div class="budget-data">
        <div class="budget-data-label">
          Transactions
        </div>
        <div class="budget-data-value">
          {{ budget.transactions }}
        </div>
      </div>
    </div>
    <div class="budget-graph-spent-container">
      <div class="budget-graph-spent">
        <Bar id="{{budget.id}}"
             :options="chartOptions()"
             :data="chartData(budget)"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import 'primeicons/primeicons.css'
import {Bar} from 'vue-chartjs'
import 'primeicons/primeicons.css'
import moment from "moment";
import {Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale} from 'chart.js'
import ChartDataLabels from 'chartjs-plugin-datalabels';
import { formatMoney } from '@/utils/format'
import { useSettings } from '@/composables/useSettings'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, ChartDataLabels)

export default {
  components: {
    Bar: Bar
  },
  props: {
    budget: Object,
    privacy: Boolean
  },
  setup() {
    const { isDark } = useSettings()
    return { isDark }
  },
  methods: {
    formatMoney,
    chartOptions() {
      return {
        animation: {
          duration: 0
        },
        responsive: true,
        maintainAspectRatio: false,
        font: {
          size: 15
        },
        plugins: {
          legend: {
            display: false
          },
          datalabels: {
            formatter: (value, ctx) => {
              if (ctx.dataIndex === 0) {
                if(this.privacy) {
                  return "---"
                }
                return value
              }
              return Math.round(value / ctx.dataset.data[0] * 100) + "%"
            },
            color: '#333333',
            backgroundColor: '#DDDDDDAE',
            borderRadius: 10,
            anchor: 'center',
            font: {
              size: 15
            }
          }
        },
        scales: {
          x: {
            stacked: false,
            display: !this.privacy,
            ticks: { color: this.isDark ? '#ffffff' : '#333333' },
            grid: { color: this.isDark ? '#ffffff44' : '#33333322' }
          },
          y: {
            stacked: true,
            ticks: { color: this.isDark ? '#ffffff' : '#333333' },
            grid: { color: this.isDark ? '#ffffff44' : '#33333322' }
          }
        },
        indexAxis: 'y',
      }
    },
    isBudgetExceeded(budget) {
      return budget.spentCents > budget.amountCents;
    },
    isBudgetCloseToMax(budget) {
      return budget.spentCents > budget.amountCents * 0.75;
    },
    percentileTimeSpent(budget) {
      const start = moment(budget.fromDate, "YYYY-MM-DD");
      const end = moment(budget.toDate, "YYYY-MM-DD");
      if (end.isBefore(moment.now())) {
        return 1.0;
      }
      const duration = moment.duration(start.diff(end)).asDays();
      const daysSinceStart = moment.duration(start.diff(moment.now())).asDays();

      return daysSinceStart / duration;
    },
    chartData(budget) {
      let colors = ["#CCCCCC", "#34eb71", "#CCCCCC"];

      if (this.isBudgetCloseToMax(budget)) {
        colors[1] = "#edd609";
      }

      if (this.isBudgetExceeded(budget)) {
        colors[1] = "#f79188"
      }

      return {
        labels: ["Size", "Spent", "Time"],
        datasets: [{
          data: [
            Math.floor(Math.abs(budget.amountCents / 100.0)),
            Math.floor(Math.abs(budget.spentCents / 100.0)),
            this.percentileTimeSpent(budget) * Math.abs(budget.amountCents / 100.0),
          ],
          backgroundColor: colors
        }],
      }
    }
  }
}
;
</script>

<style scoped>
.budget-details {
  display: grid;
  grid-template: 'description'
                 'budget_data'
                 'budget_graph_spent';
  margin-bottom: 15px;
  row-gap: 5px;
  background-color: var(--color-background)
}

.budget-description {
  grid-area: description;
  text-align: left;
  padding-left: 5px;
  background-color: var(--color-negative-background);
  color: var(--color-negative-text);
}

.budget-graph-spent-container {
  width: 100%;
}

.budget-graph-spent {
  grid-area: budget_graph_spent;
  display: block;
  width: 100%;
  height: 160px;
}

.budget-data {
  display: grid;
  grid-template: 'label value';
  margin-bottom: 3px;
  background-color: var(--color-background)
}

.budget-data-label {
  grid-area: label;
  text-align: left;
  font-size: var(--pf-text-large-font-size);
  color: var(--color-text);
}

.budget-data-value {
  grid-area: value;
  text-align: right;
  font-size: var(--pf-text-large-font-size);
  color: var(--color-text);
}

</style>
