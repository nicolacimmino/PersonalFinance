<template>
  <div class="budget-details">
    <div class="description pf-text-large">
      {{ budget.description }}
    </div>
    <div class="budget-size pf-text-medium">
      {{ (!privacy) ? Math.floor(Math.abs(budget.amount_cents / 100.0)) + "&nbsp;" + budget.currency : "" }}
    </div>
    <div class="budget-dates pf-text-medium">
      {{ budget.from_date }} - {{ budget.to_date }}
    </div>
    <div v-if="privacy" class="budget-spent pf-text-medium">
      Spent: {{ Math.floor((100.0 * Math.abs(budget.spent_cents)) / budget.amount_cents) }}%
    </div>
    <div v-else class="budget-spent pf-text-medium">
      Spent: {{ Math.floor(Math.abs(budget.spent_cents / 100.0)) }} {{ budget.currency }}
    </div>
    <div class="budget_transactions pf-text-medium">
      Transactions: {{ budget.transactions }}
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
  grid-template: 'description'
                 'budget_dates'
                 'budget_size'
                 'budget_spent'
                 'budget_transactions'
                 'budget_graph';
  margin-bottom: 15px;
  row-gap: 5px;
  background-color: #E9B87222;
}

.description {
  grid-area: description;
  text-align: left;
  padding-left: 5px;
  background-color: #6494AA;
  color: white;
}

.budget-size {
  grid-area: budget_size;
  text-align: center;
  padding-right: 5px;
}

.budget_transactions {
  grid-area: budget_transactions;
  text-align: center;
  padding-right: 5px;
}

.budget-dates {
  grid-area: budget_dates;
  text-align: center;
  padding-right: 5px;
}

.budget-spent {
  grid-area: budget_spent;
  text-align: center;
  padding-right: 5px;
}


.budget_graph {
  grid-area: budget_graph;
  width: 190px;
  margin: auto auto;
}

</style>
