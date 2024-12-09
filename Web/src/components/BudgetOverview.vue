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
          {{ budget.from_date }}
        </div>
      </div>
      <div class="budget-data">
        <div class="budget-data-label">
          Valid to
        </div>
        <div class="budget-data-value">
          {{ budget.to_date }}
        </div>
      </div>
      <div class="budget-data" v-if="!privacy">
        <div class="budget-data-label">
          Size
        </div>
        <div class="budget-data-value">
          {{ Math.floor(Math.abs(budget.amount_cents / 100.0))  }} EUR
        </div>
      </div>
      <div class="budget-data" v-if="!privacy">
        <div class="budget-data-label">
          Spent
        </div>
        <div class="budget-data-value">
          {{ Math.floor(Math.abs(budget.spent_cents / 100.0)) }} {{ budget.currency }}
        </div>
      </div>
      <div class="budget-data">
        <div class="budget-data-label">
          Spent %
        </div>
        <div class="budget-data-value">
          {{ Math.floor((100.0 * Math.abs(budget.spent_cents)) / budget.amount_cents) }}%
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
    <div class="budget-graph">
      <Pie id="{{budget.id}}"
           :options="chartOptions(budget)"
           :data="chartData(budget)"
      />
    </div>
  </div>
</template>

<script>
import 'primeicons/primeicons.css'
import {Pie} from 'vue-chartjs'
import {Chart as ChartJS, ArcElement, Tooltip, Legend} from 'chart.js'
import ChartDataLabels from 'chartjs-plugin-datalabels'
import 'primeicons/primeicons.css'

ChartJS.register(ArcElement, Tooltip, Legend, ChartDataLabels)

export default {
  components: {
    Pie: Pie
  },
  props: {
    budget: Object,
    privacy: Boolean
  },
  methods: {
    chartOptions(budget) {
      return {
        responsive: true,
        layout: {
          padding: 10,
        },
        plugins: {
          datalabels: {
            formatter: (value, ctx) => {
              if (ctx.chart.data.datasets[0].data.length === 1) {
                return null
              }
              let sum = 0;
              ctx.chart.data.datasets[0].data.map(data => {
                sum += data;
              });
              return (value * 100 / sum).toFixed(2) + "%";
            },
            color: '#333333',
            backgroundColor: '#DDDDDDAE',
            borderRadius: 10,
            anchor: 'end',
            font: {
              size: 15
            }
          }
        }
      }
    },
    isBudgetExceeded(budget) {
      return budget.spent_cents > budget.amount_cents;
    },
    isBudgetCloseToMax(budget) {
      return budget.spent_cents > budget.amount_cents * 0.75;
    },
    chartData(budget) {
      let colors = ["#09ed55", "#CCCCCC"];

      if (this.isBudgetCloseToMax(budget)) {
        colors = ["#edd609", "#CCCCCC"]
      }

      if (this.isBudgetExceeded(budget)) {
        colors = ["#f79188"]
      }

      return {
        datasets: [{
          backgroundColor: colors,
          data: [
            Math.floor(Math.abs(budget.spent_cents / 100.0)),
            (budget.spent_cents < budget.amount_cents) ?
                Math.floor(Math.abs(budget.amount_cents / 100.0) - Math.abs(budget.spent_cents / 100.0)) : null,
          ].filter(item => {
            return item !== null
          })
        }]
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
                 'budget_graph';
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

.budget-graph {
  grid-area: budget_graph;
  width: 190px;
  margin: auto auto;
}

.budget-data {
  display: grid;
  grid-template: 'label value';
  margin-bottom: 15px;
  row-gap: 5px;
  background-color: var(--color-background)
}

.budget-data-label {
  grid-area: label;
  text-align: left;
  font-size: var(--pf-text-medium-font-size);
  color: var(--color-text);
}

.budget-data-value {
  grid-area: value;
  text-align: right;
  font-size: var(--pf-text-medium-font-size);
  color: var(--color-text);
}

</style>
