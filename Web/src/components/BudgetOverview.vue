<template>
  <div class="budget-details">
    <div class="description">
      {{ budget.description }}
    </div>
    <div class="budget-size">
      {{ (!privacy) ? Math.floor(Math.abs(budget.amount_cents / 100.0)) + " " + budget.currency : "" }}
    </div>
    <div class="budget-dates">
      {{ budget.from_date }} - {{ budget.to_date }}
    </div>
    <div class="budget_info">
      <div class="budget_info_container">
        <div v-if="privacy" class="budget_spent">
          Spent: {{ Math.floor((100.0 * Math.abs(budget.spent_cents)) / budget.amount_cents) }}%
        </div>
        <div v-else class="budget_spent">
          Spent<br>
          {{ Math.floor(Math.abs(budget.spent_cents / 100.0)) }} {{ budget.currency }}
        </div>
        <div class="budget_transactions">
          Transactions<br>
          {{ budget.transactions }}
        </div>
      </div>
    </div>
    <div class="budget_graph">
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
  grid-template: 'description description budget-size'
                 'budget_dates budget_dates budget_dates'
                 'budget_info budget_graph budget_graph';
  /*padding: 5px;*/
  margin-bottom: 15px;
  background-color: #E9B87222;
}

.description {
  grid-area: description;
  text-align: left;
  padding-left: 5px;
  background-color: #6494AA;
  color: white;
  font-size: 10px;
  font-family: monospace;
}

.budget-size {
  grid-area: budget-size;
  text-align: right;
  padding-right: 5px;
  background-color: #6494AA;
  color: white;
  font-size: 10px;
  font-family: monospace;
}

.budget_info {
  grid-area: budget_info;
  vertical-align: center;
  align-content: center;
}

.budget_info_container {
  display: grid;
  grid: 'budget_spent'
        'budget_transactions';
}

.budget-dates {
  grid-area: budget_dates;
  font-size: 10px;
  font-family: monospace;
  padding: 5px;
  text-align: center;
}

.budget_spent {
  grid-area: budget_spent;
  font-size: 10px;
  font-family: monospace;
  padding: 5px;
  text-align: center;
}

.budget_transactions {
  grid-area: budget_transactions;
  font-size: 10px;
  font-family: monospace;
  padding: 5px;
  text-align: center;
}

.budget_graph {
  grid-area: budget_graph;
  width: 190px;
  /*padding: 10px;*/
}

</style>
